#![feature(str_split_once)]
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day13")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn part1(input: &str) {
    let timestamp = input
        .lines()
        .next()
        .expect("have valid input")
        .parse::<u64>()
        .expect("have valid input");
    let buses = input
        .lines()
        .skip(1)
        .next()
        .expect("have valid input")
        .split(",")
        .filter(|&x| x != "x")
        .map(|x| x.parse::<u64>().expect("have valid input"));
    let timestamp_offset = buses.map(|x| (x - (timestamp % x), x));
    let min_timestamp = timestamp_offset
        .min_by(|x, y| x.0.cmp(&y.0))
        .expect("have a minimum");
    println!(
        "have bus {} in {} minutes, solution = {}",
        min_timestamp.1,
        min_timestamp.0,
        min_timestamp.0 * min_timestamp.1
    );
}

fn part2(input: &str) {
    let buses: Vec<(u64, u64)> = input
        .lines()
        .skip(1)
        .next()
        .expect("have valid input")
        .split(",")
        .enumerate()
        .filter(|&(_, x)| x != "x")
        .map(|(i, x)| (i as u64, x.parse::<u64>().expect("have valid input")))
        .map(|(i, bus)| (bus, (bus - (i % bus)) % bus))
        .collect();

    // do CRT over the buses
    let m: u64 = buses.iter().map(|x| x.0).product();

    let sol: u64 = buses
        .iter()
        .map(|x| {
            modinverse::modinverse((m / x.0) as i64, x.0 as i64).expect("modinv is possible") as u64
                * (m / x.0)
                * x.1
        })
        .sum();
    println!("The first timestamp where this is possible is {}", sol % m);
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
