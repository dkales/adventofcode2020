use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day22")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn part1(input: &str) {
    let decks: Vec<_> = input
        .split("\r\n\r\n")
        .map(|x| {
            x.lines()
                .skip(1)
                .map(|line| line.parse::<usize>().expect("valid input"))
                .collect::<VecDeque<_>>()
        })
        .collect();
    let mut deck1 = decks[0].clone();
    let mut deck2 = decks[1].clone();

    loop {
        if deck1.is_empty() || deck2.is_empty() {
            break;
        }
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
    let score = deck1
        .iter()
        .chain(deck2.iter())
        .rev()
        .enumerate()
        .fold(0usize, |acc, (idx, u)| acc + (idx + 1) * u);
    println!("Score: {}", score);
}

fn recursive_game(deck1: &[usize], deck2: &[usize]) -> bool {
    let mut deck1: Vec<_> = deck1.iter().copied().collect();
    let mut deck2: Vec<_> = deck2.iter().copied().collect();
    let mut hist: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

    loop {
        if deck1.is_empty() || deck2.is_empty() {
            break;
        }
        if hist.contains(&(deck1.clone(), deck2.clone())) {
            return true;
        }
        hist.insert((deck1.clone(), deck2.clone()));
        let &card1 = deck1.first().unwrap();
        let &card2 = deck2.first().unwrap();
        deck1.remove(0);
        deck2.remove(0);
        let player1_wins = if deck1.len() >= card1 && deck2.len() >= card2 {
            recursive_game(&deck1[0..card1], &deck2[0..card2])
        } else {
            card1 > card2
        };
        if player1_wins {
            deck1.push(card1);
            deck1.push(card2);
        } else {
            deck2.push(card2);
            deck2.push(card1);
        }
    }

    return deck2.is_empty();
}

fn part2(input: &str) {
    let decks: Vec<_> = input
        .split("\r\n\r\n")
        .map(|x| {
            x.lines()
                .skip(1)
                .map(|line| line.parse::<usize>().expect("valid input"))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut deck1 = decks[0].clone();
    let mut deck2 = decks[1].clone();
    let mut hist: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

    loop {
        if deck1.is_empty() || deck2.is_empty() {
            break;
        }
        if hist.contains(&(deck1.clone(), deck2.clone())) {
            break;
        }
        hist.insert((deck1.clone(), deck2.clone()));
        let &card1 = deck1.first().unwrap();
        let &card2 = deck2.first().unwrap();
        deck1.remove(0);
        deck2.remove(0);
        let player1_wins = if deck1.len() >= card1 && deck2.len() >= card2 {
            recursive_game(&deck1[0..card1], &deck2[0..card2])
        } else {
            card1 > card2
        };
        if player1_wins {
            deck1.push(card1);
            deck1.push(card2);
        } else {
            deck2.push(card2);
            deck2.push(card1);
        }
    }
    let score = deck1
        .iter()
        .chain(deck2.iter())
        .rev()
        .enumerate()
        .fold(0usize, |acc, (idx, u)| acc + (idx + 1) * u);
    println!("Score: {}", score);
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
