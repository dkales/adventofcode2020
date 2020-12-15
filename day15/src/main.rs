#![feature(str_split_once)]
use std::{collections::HashMap, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day15")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn game(input: &str, target: usize) {
    let mut mem: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut last_num = 0;
    let mut index: usize = 0;
    for num in input
        .split(",")
        .map(|x| x.parse::<usize>().expect("valid input"))
    {
        (*mem.entry(last_num).or_default()).push(index);
        index += 1;
        last_num = num;
    }
    while index < target {
        let entry = mem.entry(last_num).or_default();
        entry.push(index);
        if entry.len() < 2 {
            last_num = 0;
        } else {
            last_num = entry[entry.len() - 1] - entry[entry.len() - 2];
        }
        index += 1;
    }

    println!("Last Memory value: {}", last_num);
}

fn part1(input: &str) {
    game(input, 2020);
}

fn part2(input: &str) {
    game(input, 30000000);
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
