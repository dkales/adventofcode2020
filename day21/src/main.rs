use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day21")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn part1(input: &str) {
    let mut ing_list: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_ing: Vec<String> = Vec::new();
    for line in input.lines() {
        let (ing, allergens) = line.split_at(line.find(" (").expect("valid input"));
        let ing: HashSet<_> = ing.split_ascii_whitespace().map(str::to_owned).collect();
        ing.iter().for_each(|x| {
            all_ing.push(x.clone());
        });
        let allergens: Vec<_> = allergens
            .trim_end_matches(")")
            .trim_start_matches(" (contains ")
            .split(", ")
            .map(str::to_owned)
            .collect();
        for a in allergens {
            ing_list
                .entry(a)
                .and_modify(|list| *list = list.intersection(&ing).cloned().collect::<HashSet<_>>())
                .or_insert(ing.clone());
        }
    }
    let potential_allergens: HashSet<_> = ing_list.iter().map(|x| x.1.iter()).flatten().collect();
    let safe_ing = all_ing
        .iter()
        .filter(|&x| !potential_allergens.contains(x))
        .count();
    println!("safe: {}", safe_ing);
}
fn part2(input: &str) {
    let mut ing_list: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines() {
        let (ing, allergens) = line.split_at(line.find(" (").expect("valid input"));
        let ing: HashSet<_> = ing.split_ascii_whitespace().map(str::to_owned).collect();
        let allergens: Vec<_> = allergens
            .trim_end_matches(")")
            .trim_start_matches(" (contains ")
            .split(", ")
            .map(str::to_owned)
            .collect();
        for a in allergens {
            ing_list
                .entry(a)
                .and_modify(|list| *list = list.intersection(&ing).cloned().collect::<HashSet<_>>())
                .or_insert(ing.clone());
        }
    }
    let mut confirmed = Vec::new();
    while ing_list.iter().count() > 0 {
        for x in ing_list.iter().filter(|x| x.1.len() == 1) {
            confirmed.push((x.0.to_owned(), x.1.iter().next().unwrap().to_owned()));
        }
        for (all,ing) in confirmed.iter() {
            ing_list.remove(all);
            ing_list.iter_mut().for_each(|x| {x.1.remove(ing);})
        }
    }
    confirmed.sort_by(|a,b| a.0.cmp(&b.0));
    let sol = confirmed.into_iter().map(|x| x.1).collect::<Vec<_>>().join(",");
    println!("Solution: {}", sol);
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
