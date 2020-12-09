#![feature(str_split_once)]
use std::{collections::HashSet, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day09")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn part1(numbers: &[usize]) -> usize {
    for (idx, &num) in numbers.iter().enumerate().skip(25) {
        let ok = || -> bool {
            let to_check: HashSet<_> = numbers.iter().skip(idx - 25).take(25).collect();
            for &&num1 in to_check.iter() {
                if to_check.contains(&(num.wrapping_sub(num1))) {
                    return true;
                }
            }
            false
        }();
        if !ok {
            return num;
        }
    }
    panic!("no invalid number found");
}

fn part2(numbers: &[usize], invalid_num: usize) -> usize {
    for i in 0..numbers.len() {
        let mut acc = 0;
        for j in i..numbers.len() {
            acc += numbers[j];
            if acc == invalid_num && i != j {
                return numbers[i..=j]
                    .iter()
                    .min()
                    .expect("we have at least 1 element")
                    + numbers[i..=j]
                        .iter()
                        .max()
                        .expect("we have at least 1 element");
            }
            if acc > invalid_num {
                break;
            }
        }
    }
    panic!("could not find a range summing to {}", invalid_num);
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    let numbers: Vec<_> = content
        .lines()
        .map(|x| x.parse::<usize>().expect("parsing error"))
        .collect();

    let invalid_num = part1(&numbers);
    println!("found invalid number: {}", invalid_num);

    let weakness = part2(&numbers, invalid_num);
    println!("found encryption weakness: {}", weakness);
}
