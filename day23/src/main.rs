use std::{collections::{VecDeque}, path::PathBuf, vec};
use itertools::Itertools;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day23")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn part1(input: &str) {
    let mut cups: VecDeque<_> = input
        .chars()
        .map(|x| x.to_string().parse::<usize>().expect("valid input"))
        .collect();
    let m = cups.len();

    for _ in 0..100 {
        let cur = cups.pop_front().unwrap();
        cups.push_back(cur);
        let cup1 = cups.pop_front().unwrap();
        let cup2 = cups.pop_front().unwrap();
        let cup3 = cups.pop_front().unwrap();
        let mut ele = (cur + m - 2) % m + 1;
        while !cups.contains(&ele) {
            ele = (ele + m - 2) % m + 1;
        }
        let idx = cups
            .iter()
            .enumerate()
            .filter(|x| x.1 == &ele)
            .next()
            .expect("have 1 element")
            .0;
        cups.insert(idx + 1, cup1);
        cups.insert(idx + 2, cup2);
        cups.insert(idx + 3, cup3);
        //println!("cur: {}", cur);
        //println!("pickup: {}, {}, {}", cup1, cup2, cup3);
        //println!("dest: {}", ele);
        //println!("{:?}", cups);
    }
    //println!("{:?}", cups);
    while cups.front() != Some(&1) {
        let x = cups.pop_front().unwrap();
        cups.push_back(x);
    }
    let label = cups
        .iter()
        .skip(1)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");
    println!("{}", label);
}

fn part2(input: &str) {
    const NUM_CUPS : usize = 1_000_000;
    let mut cups = vec![0usize; NUM_CUPS];
    for (a,b) in input
        .chars()
        .map(|x| x.to_string().parse::<usize>().expect("valid input"))
        .chain(10..=NUM_CUPS)
        .tuple_windows() {
            cups[a-1] = b-1;
        };
    let mut current =input.chars().next().unwrap().to_digit(10).expect("have digit") as usize -1;
    cups[NUM_CUPS-1] = current;
    let m = cups.len();
    let steps = 10_000_000;
    for _ in 0..steps {
        let cup1 = cups[current];
        let cup2 = cups[cup1];
        let cup3 = cups[cup2];
        let end = cups[cup3];
        cups[current] = end;
        let mut ele = (current + m - 1) % m;
        while ele == cup1 || ele == cup2 || ele == cup3 {
            ele = (ele + m - 1) % m;
        }
        let end  = cups[ele];
        cups[ele] = cup1;
        cups[cup3] = end;
        current = cups[current];
    }
    let a = cups[0];
    let b = cups[a];
    println!("{}", (a+1)*(b+1));
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
