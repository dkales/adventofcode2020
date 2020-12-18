#![feature(str_split_once)]
use std::{collections::HashSet, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day18")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn solve1(input: &str) -> isize {
    if !input.contains("(") {
        let mut nums = input
            .split_ascii_whitespace()
            .filter_map(|x| x.parse::<isize>().ok());
        let ops = input
            .split_ascii_whitespace()
            .filter(|&x| x == "+" || x == "*");

        let mut acc = nums.next().expect("have 1 element");
        for (x, op) in nums.zip(ops) {
            match op {
                "+" => acc += x,
                "*" => acc *= x,
                _ => unreachable!(),
            }
        }

        return acc;
    }
    let index_open: Vec<_> = input
        .char_indices()
        .filter_map(|(idx, c)| if c == '(' { Some(idx) } else { None })
        .collect();
    let index_close: Vec<_> = input
        .char_indices()
        .filter_map(|(idx, c)| if c == ')' { Some(idx) } else { None })
        .collect();
    let mut i = 0;
    let mut open = index_open[i];
    let close = index_close[0];
    while input[open + 1..close].contains("(") {
        i += 1;
        open = index_open[i];
    }
    let sol = solve1(&input[open + 1..close]);
    let eq = input.replace(&input[open..=close], &sol.to_string());

    return solve1(&eq);
}

fn part1(input: &str) {
    println!("sum = {}", input.lines().map(|x| solve1(x)).sum::<isize>());
}
fn solve2(input: &str) -> isize {
    if !input.contains("(") {
        let mut eq = input.to_string();
        while eq.contains("+") {
            let tokens: Vec<_> = eq.split_ascii_whitespace().collect();
            let mul = tokens
                .iter()
                .enumerate()
                .filter_map(|(idx, &s)| if s == "+" { Some(idx) } else { None })
                .next()
                .expect("have one *");
            let num1 = tokens[mul - 1].parse::<isize>().expect("have num");
            let num2 = tokens[mul + 1].parse::<isize>().expect("have num");
            eq = eq.replace(&format!("{} + {}", num1, num2), &(num1 + num2).to_string());
        }
        while eq.contains("*") {
            let tokens: Vec<_> = eq.split_ascii_whitespace().collect();
            let mul = tokens
                .iter()
                .enumerate()
                .filter_map(|(idx, &s)| if s == "*" { Some(idx) } else { None })
                .next()
                .expect("have one *");
            let num1 = tokens[mul - 1].parse::<isize>().expect("have num");
            let num2 = tokens[mul + 1].parse::<isize>().expect("have num");
            eq = eq.replace(&format!("{} * {}", num1, num2), &(num1 * num2).to_string());
        }

        return eq.parse::<isize>().expect("have solution");
    }
    let index_open: Vec<_> = input
        .char_indices()
        .filter_map(|(idx, c)| if c == '(' { Some(idx) } else { None })
        .collect();
    let index_close: Vec<_> = input
        .char_indices()
        .filter_map(|(idx, c)| if c == ')' { Some(idx) } else { None })
        .collect();
    let mut i = 0;
    let mut open = index_open[i];
    let close = index_close[0];
    while input[open + 1..close].contains("(") {
        i += 1;
        open = index_open[i];
    }
    let sol = solve2(&input[open + 1..close]);
    let eq = input.replace(&input[open..=close], &sol.to_string());

    return solve2(&eq);
}
fn part2(input: &str) {
    println!("sum = {}", input.lines().map(|x| solve2(x)).sum::<isize>());
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
