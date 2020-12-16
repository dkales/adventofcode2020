#![feature(str_split_once)]
use std::{collections::HashSet, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day16")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn part1(input: &str) {
    let ranges: Vec<_> = input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            x.split(":")
                .skip(1)
                .next()
                .expect("have valid input")
                .trim()
                .split(" or ")
                .map(|y| {
                    let v: Vec<_> = y
                        .split("-")
                        .map(|z| z.parse::<u64>().expect("have valid range"))
                        .collect();
                    v[0]..=v[1]
                })
        })
        .flatten()
        .collect();
    println!("Ranges: {:?}", ranges);
    let solution: u64 = input
        .lines()
        .skip_while(|&x| x != "nearby tickets:")
        .skip(1)
        .map(|x| {
            x.split(",")
                .map(|y| y.parse::<u64>().expect("have valid ticket number"))
        })
        .flatten()
        .filter(|x| !ranges.iter().any(|range| range.contains(x)))
        .sum();
    println!("Part 1: {}", solution);
}
fn part2(input: &str) {
    let ranges: Vec<Vec<_>> = input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            x.split(":")
                .skip(1)
                .next()
                .expect("have valid input")
                .trim()
                .split(" or ")
                .map(|y| {
                    let v: Vec<_> = y
                        .split("-")
                        .map(|z| z.parse::<u64>().expect("have valid range"))
                        .collect();
                    v[0]..=v[1]
                })
                .collect()
        })
        .collect();
    println!("Ranges: {:?}", ranges);
    let my_ticket: Vec<_> = input
        .lines()
        .skip_while(|&x| x != "your ticket:")
        .skip(1)
        .next()
        .expect("have my ticket")
        .split(",")
        .map(|y| y.parse::<u64>().expect("have valid ticket number"))
        .collect();
    let tickets: Vec<Vec<u64>> = input
        .lines()
        .skip_while(|&x| x != "nearby tickets:")
        .skip(1)
        .map(|x| {
            x.split(",")
                .map(|y| y.parse::<u64>().expect("have valid ticket number"))
                .collect()
        })
        .chain(std::iter::once(my_ticket.clone()))
        .filter(|ticket| {
            ticket
                .iter()
                .all(|number| ranges.iter().flatten().any(|range| range.contains(number)))
        })
        .collect();

    let mut possible_fields = vec![HashSet::new(); ranges.len()];
    for i in 0..ranges.len() {
        for j in 0..ranges.len() {
            if tickets
                .iter()
                .all(|x| ranges[j].iter().any(|range| range.contains(&x[i])))
            {
                possible_fields[i].insert(j);
            }
        }
    }
    while !possible_fields
        .iter()
        .all(|possibilites| possibilites.len() == 1)
    {
        let solved: Vec<_> = possible_fields
            .iter()
            .filter(|x| x.len() == 1)
            .flatten()
            .copied()
            .collect();
        possible_fields
            .iter_mut()
            .filter(|x| x.len() > 1)
            .for_each(|x| {
                solved.iter().for_each(|val| {
                    x.remove(val);
                })
            });
    }
    println!("{:?}", possible_fields);

    let solution: u64 = possible_fields
        .iter()
        .enumerate()
        .filter(|x| (0..6).contains(x.1.iter().next().unwrap()))
        .map(|x| my_ticket[x.0])
        .product();
    println!("Part 2: {}", solution);
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
