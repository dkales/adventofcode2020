#![feature(str_split_once)]
use regex::Regex;
use std::{collections::HashMap, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day19")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}
#[derive(Debug)]
enum Rule {
    CharRule(char),
    OrRule(Vec<usize>, Vec<usize>),
    ConcatRule(Vec<usize>),
    OneOrMoreRule(usize),
    HackRule(usize, usize),
}
const MAX_WORKAROUND_LEN: usize = 5;

fn build_regex_string(rules: &HashMap<usize, Rule>, idx: usize) -> String {
    match &rules[&idx] {
        Rule::CharRule(c) => c.to_string(),
        Rule::ConcatRule(v) => v
            .iter()
            .map(|&i| build_regex_string(rules, i))
            .collect::<Vec<_>>()
            .join(""),
        Rule::OrRule(v1, v2) => {
            String::from("(")
                + &v1
                    .iter()
                    .map(|&i| build_regex_string(rules, i))
                    .collect::<Vec<_>>()
                    .join("")
                + "|"
                + &v2
                    .iter()
                    .map(|&i| build_regex_string(rules, i))
                    .collect::<Vec<_>>()
                    .join("")
                + ")"
        }
        Rule::OneOrMoreRule(idx) => String::from("(") + &build_regex_string(rules, *idx) + ")+",
        Rule::HackRule(idx1, idx2) => {
            let rule1 = build_regex_string(rules, *idx1);
            let rule2 = build_regex_string(rules, *idx2);
            String::from("(")
                + &(1..=MAX_WORKAROUND_LEN)
                    .map(|i| format!("(({}){{{}}}({}){{{}}})", rule1, i, rule2, i))
                    .collect::<Vec<_>>()
                    .join("|")
                + ")"
        }
    }
}

fn part1(input: &str) {
    let input = input.replace("\r\n", "\n");
    let (rules, inputs) = input.split_once("\n\n").expect("have input");
    let rules: HashMap<usize, Rule> = rules
        .lines()
        .map(|line| {
            let (idx, rule) = line.split_once(":").expect("valid rule");
            let idx = idx.parse::<usize>().expect("valid index");
            if rule.contains("\"") {
                return (
                    idx,
                    Rule::CharRule(
                        rule.trim()
                            .trim_matches('"')
                            .chars()
                            .next()
                            .expect("have char"),
                    ),
                );
            }
            if rule.contains("|") {
                let mut a = rule.split("|").map(|x| {
                    x.split_ascii_whitespace()
                        .map(|x| x.parse::<usize>().expect("rule valid"))
                        .collect::<Vec<_>>()
                });
                let or = Rule::OrRule(
                    a.next().expect("have element"),
                    a.next().expect("have second element"),
                );
                return (idx, or);
            }
            let r = Rule::ConcatRule(
                rule.split_ascii_whitespace()
                    .map(|x| x.parse::<usize>().expect("rule valid"))
                    .collect::<Vec<_>>(),
            );
            (idx, r)
        })
        .collect();
    let regex = String::from("^") + &build_regex_string(&rules, 0) + "$";
    let regex = Regex::new(&regex).expect("have valid regex");
    let num_good = inputs.lines().filter(|line| regex.is_match(line)).count();
    println!("Have {:?} matches", num_good);
}
fn part2(input: &str) {
    let input = input.replace("\r\n", "\n");
    let (rules, inputs) = input.split_once("\n\n").expect("have input");
    let mut rules: HashMap<usize, Rule> = rules
        .lines()
        .map(|line| {
            let (idx, rule) = line.split_once(":").expect("valid rule");
            let idx = idx.parse::<usize>().expect("valid index");
            if rule.contains("\"") {
                return (
                    idx,
                    Rule::CharRule(
                        rule.trim()
                            .trim_matches('"')
                            .chars()
                            .next()
                            .expect("have char"),
                    ),
                );
            }
            if rule.contains("|") {
                let mut a = rule.split("|").map(|x| {
                    x.split_ascii_whitespace()
                        .map(|x| x.parse::<usize>().expect("rule valid"))
                        .collect::<Vec<_>>()
                });
                let or = Rule::OrRule(
                    a.next().expect("have element"),
                    a.next().expect("have second element"),
                );
                return (idx, or);
            }
            let r = Rule::ConcatRule(
                rule.split_ascii_whitespace()
                    .map(|x| x.parse::<usize>().expect("rule valid"))
                    .collect::<Vec<_>>(),
            );
            (idx, r)
        })
        .collect();

    rules.insert(8, Rule::OneOrMoreRule(42));
    rules.insert(11, Rule::HackRule(42, 31));
    let regex = String::from("^") + &build_regex_string(&rules, 0) + "$";
    let regex = Regex::new(&regex).expect("have valid regex");
    let num_good = inputs.lines().filter(|line| regex.is_match(line)).count();
    println!("Have {:?} matches", num_good);
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
