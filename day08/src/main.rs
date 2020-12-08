#![feature(str_split_once)]
use std::{collections::HashSet, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day08")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn part1(content: &str) -> (isize, bool) {
    let mut accumulator = 0isize;
    let instructions = content
        .lines()
        .map(|x| x.split_once(" ").expect("correct format"))
        .collect::<Vec<(&str, &str)>>();
    let mut already_visited: HashSet<usize> = HashSet::new();
    let mut current_instruction = 0usize;

    loop {
        if current_instruction == instructions.len() {
            return (accumulator, false);
        }
        if already_visited.contains(&current_instruction) {
            return (accumulator, true);
        }
        already_visited.insert(current_instruction);

        match instructions.get(current_instruction) {
            Some(("nop", _)) => current_instruction += 1,
            Some(("jmp", amount)) => {
                current_instruction = (current_instruction as isize
                    + amount.parse::<isize>().expect("correct format for jmp"))
                    as usize;
            }
            Some(("acc", amount)) => {
                accumulator += amount.parse::<isize>().expect("correct format for jmp");
                current_instruction += 1;
            }
            _ => panic!("invalid instruction"),
        };
    }
}

fn part2(content: &str) -> isize {
    let num_jmp = content.matches("jmp").count();
    for i in 0..num_jmp {
        let tmp = content.replacen("jmp", "temp", i + 1);
        let tmp2 = tmp.as_str().replacen("temp", "jmp", i);
        let modified = tmp2.as_str().replace("temp", "nop");
        if let (acc, false) = part1(modified.as_str()) {
            return acc;
        }
    }
    let num_nop = content.matches("nop").count();
    for i in 0..num_nop {
        let tmp = content.replacen("nop", "temp", i + 1);
        let tmp2 = tmp.as_str().replacen("temp", "nop", i);
        let modified = tmp2.as_str().replace("temp", "jmp");
        if let (acc, false) = part1(modified.as_str()) {
            return acc;
        }
    }
    panic!("could not find a valid program");
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    let (accumulator, _) = part1(&content);
    println!("Value of accumulator = {}", accumulator);

    let accumulator = part2(&content);
    println!("Value of accumulator of fixed program = {}", accumulator);
}
