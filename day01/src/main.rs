use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day01")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn find_sum_in_sorted_vec(values: &[u64], target: u64) -> Option<(u64, u64)> {
    let mut i = 0;
    let mut j = values.len() - 1;

    while i < j {
        if values[i] + values[j] == target {
            return Some((values[i], values[j]));
        }
        if values[i] + values[j] > target {
            j -= 1;
        } else {
            i += 1;
        }
    }
    return None;
}

fn part1(values: &[u64]) {
    let target = 2020;
    match find_sum_in_sorted_vec(&values, target) {
        Some((a, b)) => println!("Found {} + {} = {}, Product = {}", a, b, target, a * b),
        None => eprintln!("Could not find a pair that sums to 2020 in input"),
    };
}

fn part2(values: &[u64]) {
    for val in values.iter() {
        let target = 2020 - val;
        if let Some((a, b)) = find_sum_in_sorted_vec(&values, target) {
            println!("Found {} + {} + {}, Product = {}", a, b, val, a * b * val);
            return;
        };
    }
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    let mut values: Vec<_> = content
        .lines()
        .map(|x| x.parse::<u64>().expect("input not u64"))
        .collect();
    values.sort();

    part1(&values);

    part2(&values);
}
