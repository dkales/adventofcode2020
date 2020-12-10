#![feature(str_split_once)]
use itertools::Itertools;
use std::{collections::HashMap, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day10")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn part1(numbers: &[usize]) {
    let mut sorted = numbers.to_owned();
    sorted.push(0);
    sorted.push(sorted.iter().max().expect("has max element") + 3);
    sorted.sort();
    let (a, _b, c) =
        sorted
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .fold((0, 0, 0), |(a, b, c), diff| match diff {
                1 => (a + 1, b, c),
                2 => (a, b + 1, c),
                3 => (a, b, c + 1),
                _ => (a, b, c),
            });
    println!(
        "Number of 1-jolt diffs times number of 3-jolt diffs = {}",
        a * c
    );
}

fn part2(numbers: &[usize]) {
    let mut sorted = numbers.to_owned();
    sorted.push(0);
    let goal = sorted.iter().max().expect("has max element") + 3usize;
    sorted.sort();
    let mut ways_to_connect: HashMap<usize, usize> = HashMap::new();
    ways_to_connect.insert(goal, 1);

    for &adapter in sorted.iter().rev() {
        for offset in 1..=3 {
            let combinations = *ways_to_connect.entry(adapter + offset).or_default();
            *ways_to_connect.entry(adapter).or_default() += combinations;
        }
    }
    println!(
        "We have {} combinations to connect the things",
        ways_to_connect.get(&0).unwrap()
    );
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    let numbers: Vec<_> = content
        .lines()
        .map(|x| x.parse::<usize>().expect("parsing error"))
        .collect();

    part1(&numbers);

    part2(&numbers);
}
