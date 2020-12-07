#![feature(str_split_once)]
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day07")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn part1(content: &str) {
    let mut contained_by: HashMap<String, Vec<String>> = HashMap::new();
    for line in content.lines() {
        if let Some((color, rest)) = line.split_once(" bags contain ") {
            for part in rest.split(",") {
                let contained_color = part
                    .split_ascii_whitespace()
                    .skip(1)
                    .take(2)
                    .collect::<Vec<_>>()
                    .join(" ");
                contained_by
                    .entry(contained_color)
                    .or_insert(Vec::new())
                    .push(color.to_string());
            }
        }
    }
    let mut can_contain_shiny_gold: HashSet<String> = contained_by
        .get("shiny gold")
        .expect("we must have a shiny gold rule")
        .clone()
        .into_iter()
        .collect();
    loop {
        let num_before = can_contain_shiny_gold.len();
        for color in can_contain_shiny_gold.clone().into_iter() {
            contained_by.entry(color).or_default().iter().for_each(|x| {
                can_contain_shiny_gold.insert(x.to_owned());
            });
        }
        if num_before == can_contain_shiny_gold.len() {
            break;
        }
    }
    println!(
        "{} different bags can contain a shiny gold one.",
        can_contain_shiny_gold.len()
    );
}

fn get_bag_amount_contained_within(
    bag_rules: &HashMap<String, Vec<(String, usize)>>,
    color: &str,
) -> usize {
    let mut amount = 0;

    if let Some(v) = bag_rules.get(color) {
        for (col, num) in v {
            amount += num;
            amount += num * get_bag_amount_contained_within(bag_rules, col);
        }
    }

    return amount;
}

fn part2(content: &str) {
    let mut contains: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for line in content.lines() {
        if let Some((color, rest)) = line.split_once(" bags contain ") {
            if rest.contains("no other bags") {
                continue;
            }
            for part in rest.split(",") {
                let contained_color = part
                    .split_ascii_whitespace()
                    .skip(1)
                    .take(2)
                    .collect::<Vec<_>>()
                    .join(" ");
                let contained_amount = part
                    .split_ascii_whitespace()
                    .take(1)
                    .collect::<String>()
                    .parse::<usize>()
                    .expect("must have a bag amount");
                contains
                    .entry(color.to_string())
                    .or_insert(Vec::new())
                    .push((contained_color, contained_amount));
            }
        }
    }
    println!(
        "Number of bags in my shiny gold one are {}.",
        get_bag_amount_contained_within(&contains, "shiny gold")
    );
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
