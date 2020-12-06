#![feature(iterator_fold_self)]
use std::{collections::HashSet, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day06")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn part1(content: &str) {
    let content = content.replace("\r\n", "\n");
    let answers: usize = content
        .split("\n\n")
        .map(|x| {
            x.trim()
                .chars()
                .filter(|c| c.is_ascii_lowercase())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();
    println!("Got {} \"yes\" answers for part 1.", answers);
}
fn part2(content: &str) {
    let content = content.replace("\r\n", "\n");
    let answers: usize = content
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|line| {
                    line.chars()
                        .filter(|c| c.is_ascii_lowercase())
                        .collect::<HashSet<_>>()
                })
                .fold_first(|set, a| set.intersection(&a).copied().collect::<HashSet<char>>())
                .expect("we have an intersection")
                .len()
        })
        .sum();
    println!("Got {} \"yes\" answers for part 2.", answers);
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
