use std::{collections::HashSet, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day24")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn insert_path(paths: &mut HashSet<(isize, isize)>, path: &str) {
    let mut coords = (0isize, 0isize);
    for op in path
        .replace("nw", "1")
        .replace("ne", "2")
        .replace("sw", "3")
        .replace("se", "4")
        .chars()
    {
        match op {
            'w' => {
                coords.0 -= 2;
            }
            'e' => {
                coords.0 += 2;
            }
            '1' => {
                coords.0 -= 1;
                coords.1 += 1;
            }
            '2' => {
                coords.0 += 1;
                coords.1 += 1;
            }
            '3' => {
                coords.0 -= 1;
                coords.1 -= 1;
            }
            '4' => {
                coords.0 += 1;
                coords.1 -= 1;
            }
            _ => panic!("invalid input"),
        }
    }
    if paths.contains(&coords) {
        paths.remove(&coords);
    } else {
        paths.insert(coords);
    }
}

fn part1(input: &str) -> HashSet<(isize, isize)> {
    let mut tiles = HashSet::new();
    input.lines().for_each(|path| insert_path(&mut tiles, path));
    println!("Black Tiles: {}", tiles.len());
    tiles
}

const NEIGBORS: [(isize, isize); 6] = [(-2, 0), (-1, 1), (1, 1), (2, 0), (1, -1), (-1, -1)];

fn update_tiles(input: &HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    let mut next = HashSet::new();

    for coords in input.iter() {
        let black_neighbors = NEIGBORS.iter().map(|x| (coords.0 + x.0, coords.1+x.1)).filter(|x| input.contains(x)).count();
        if black_neighbors == 1 || black_neighbors == 2 {
            next.insert(*coords);
        }
        for neighbor in NEIGBORS.iter().map(|x| (coords.0 + x.0, coords.1+x.1)).filter(|x| !input.contains(x)) {
            let black_neighbors = NEIGBORS.iter().map(|x| (neighbor.0 + x.0, neighbor.1+x.1)).filter(|x| input.contains(x)).count();
            if black_neighbors == 2 {
                next.insert(neighbor);
            }

        }
    }

    next
}

fn part2(input: &HashSet<(isize, isize)>) {
    let mut current = input.clone();
    for i in 0..100 {
        current = update_tiles(&current);
        println!("Day {}: {}", i + 1, current.len());
    }
    println!("Black Tiles after 100 days: {}", current.len());
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    let tiles = part1(&content);

    part2(&tiles);
}
