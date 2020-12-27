use std::{collections::HashMap, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day25")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn part1(input: &str) {
    let p = 20201227u64;
    let pk :Vec<_>= input.lines().map(|x| x.parse::<u64>().expect("valid input")).collect();
    let sk: Vec<_> = pk.iter().map(|&y| {
        let g = 7u64;
        let mut acc = g;
        let mut x = 1u64;
        while acc != y {
            acc *= g;
            acc %= p;
            x+=1;
        }
        x
    }).collect();

    let w = (0..sk[0]).fold(1u64, |acc,_| acc * pk[1] % p);
    println!("shared key: {}", w);

}
fn part2(input: &str) {

}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
