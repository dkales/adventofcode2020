#![feature(str_split_once)]
use itertools::Itertools;
use std::{collections::HashMap, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day14")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn part1(input: &str) {
    let mut mask = "";
    let mut mem: HashMap<usize, u64> = HashMap::new();
    for line in input.lines() {
        match &line[0..4] {
            "mask" => {
                mask = line.split("=").skip(1).next().expect("have mask").trim();
            }
            "mem[" => {
                let index = line[4..]
                    .split("]")
                    .next()
                    .expect("have index")
                    .parse::<usize>()
                    .expect("valid index");
                let mut new_value = line
                    .split("=")
                    .skip(1)
                    .next()
                    .expect("have value")
                    .trim()
                    .parse::<u64>()
                    .expect("valid value");
                for (bit, action) in mask.chars().rev().enumerate() {
                    match action {
                        '1' => {
                            new_value |= 1u64 << bit;
                        }
                        '0' => {
                            new_value &= !(1u64 << bit);
                        }
                        'X' => {}
                        _ => panic!("invalid mask"),
                    };
                }
                *mem.entry(index).or_default() = new_value;
            }
            _ => panic!("invalid instruction"),
        };
    }

    println!("Sum of memory values: {}", mem.values().sum::<u64>());
}

fn part2(input: &str) {
    let mut mask = "";
    let mut mem: HashMap<usize, u64> = HashMap::new();
    for line in input.lines() {
        match &line[0..4] {
            "mask" => {
                mask = line.split("=").skip(1).next().expect("have mask").trim();
            }
            "mem[" => {
                let mut index = line[4..]
                    .split("]")
                    .next()
                    .expect("have index")
                    .parse::<usize>()
                    .expect("valid index");
                let new_value = line
                    .split("=")
                    .skip(1)
                    .next()
                    .expect("have value")
                    .trim()
                    .parse::<u64>()
                    .expect("valid value");
                let mut floating: Vec<usize> = vec![];
                for (bit, action) in mask.chars().rev().enumerate() {
                    match action {
                        '1' => {
                            index |= 1usize << bit;
                        }
                        '0' => {}
                        'X' => {
                            floating.push(1usize << bit);
                            index &= !(1usize << bit);
                        }
                        _ => panic!("invalid mask"),
                    };
                }
                let n = floating.len();
                floating.append(&mut vec![0usize; n]);
                for a in floating.iter().combinations(n) {
                    let idx = index + a.into_iter().sum::<usize>();
                    *mem.entry(idx).or_default() = new_value;
                }
            }
            _ => panic!("invalid instruction"),
        };
    }

    println!("Sum of memory values: {}", mem.values().sum::<u64>());
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
