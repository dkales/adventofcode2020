use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day01")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

struct Map {
    data: Vec<Vec<char>>,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let data: Vec<_> = input
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect();
        return Map { data };
    }
    fn count_trees_with_step(&self, step: (usize, usize)) -> usize {
        let mut coord = (0, 0);
        let mut count = 0;

        while coord.0 < self.data.len() {
            if self.data[coord.0][coord.1] == '#' {
                count += 1;
            }
            coord.0 += step.0;
            coord.1 += step.1;
            coord.1 %= self.data[0].len();
        }
        return count;
    }
}

fn part1(map: &Map) {
    println!(
        "Encountered {} trees with the 1,3 toboggan",
        map.count_trees_with_step((1, 3)),
    );
}

fn part2(map: &Map) {
    let prod: usize = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|&x| map.count_trees_with_step(x))
        .product();
    println!(
        "Product of all different toboggan tree encounters is {}",
        prod
    );
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    let map = Map::from_input(&content);

    part1(&map);

    part2(&map);
}
