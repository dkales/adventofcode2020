#![feature(str_split_once)]
use std::{collections::HashSet, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day17")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn get_neighbors(state: &HashSet<(isize, isize, isize)>, x: isize, y: isize, z: isize) -> usize {
    let mut neighbors = 0;
    for a in -1..=1 {
        for b in -1..=1 {
            for c in -1..=1 {
                if (a, b, c) != (0, 0, 0) && state.contains(&(x + a, y + b, z + c)) {
                    neighbors += 1;
                }
            }
        }
    }
    return neighbors;
}

fn update_state(state: &HashSet<(isize, isize, isize)>) -> HashSet<(isize, isize, isize)> {
    // find max indices
    let mut new_state = HashSet::new();
    let mut x_range = (0, 0);
    let mut y_range = (0, 0);
    let mut z_range = (0, 0);
    for (x, y, z) in state.iter() {
        if x_range.0 > *x {
            x_range.0 = *x;
        }
        if x_range.1 < *x {
            x_range.1 = *x;
        }
        if y_range.0 > *y {
            y_range.0 = *y;
        }
        if y_range.1 < *y {
            y_range.1 = *y;
        }
        if z_range.0 > *z {
            z_range.0 = *z;
        }
        if z_range.1 < *z {
            z_range.1 = *z;
        }
    }
    x_range = (x_range.0 - 1, x_range.1 + 1);
    y_range = (y_range.0 - 1, y_range.1 + 1);
    z_range = (z_range.0 - 1, z_range.1 + 1);
    for x in x_range.0..=x_range.1 {
        for y in y_range.0..=y_range.1 {
            for z in z_range.0..=z_range.1 {
                let neighbors = get_neighbors(state, x, y, z);
                if state.contains(&(x, y, z)) && (neighbors == 2 || neighbors == 3) {
                    new_state.insert((x, y, z));
                }
                if !state.contains(&(x, y, z)) && neighbors == 3 {
                    new_state.insert((x, y, z));
                }
            }
        }
    }

    new_state
}

fn part1(input: &str) {
    let mut active_cells: HashSet<_> = input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars().enumerate().filter_map(move |(y, c)| {
                if c == '#' {
                    Some((x as isize, y as isize, 0isize))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();

    for _ in 0..6 {
        active_cells = update_state(&active_cells);
    }
    println!("Active cells: {}", active_cells.len());
}
fn get_neighbors_4d(
    state: &HashSet<(isize, isize, isize, isize)>,
    x: isize,
    y: isize,
    z: isize,
    w: isize,
) -> usize {
    let mut neighbors = 0;
    for a in -1..=1 {
        for b in -1..=1 {
            for c in -1..=1 {
                for d in -1..=1 {
                    if (a, b, c, d) != (0, 0, 0, 0) && state.contains(&(x + a, y + b, z + c, w + d))
                    {
                        neighbors += 1;
                    }
                }
            }
        }
    }
    return neighbors;
}

fn update_state_4d(
    state: &HashSet<(isize, isize, isize, isize)>,
) -> HashSet<(isize, isize, isize, isize)> {
    // find max indices
    let mut new_state = HashSet::new();
    let mut x_range = (0, 0);
    let mut y_range = (0, 0);
    let mut z_range = (0, 0);
    let mut w_range = (0, 0);
    for (x, y, z, w) in state.iter() {
        if x_range.0 > *x {
            x_range.0 = *x;
        }
        if x_range.1 < *x {
            x_range.1 = *x;
        }
        if y_range.0 > *y {
            y_range.0 = *y;
        }
        if y_range.1 < *y {
            y_range.1 = *y;
        }
        if z_range.0 > *z {
            z_range.0 = *z;
        }
        if z_range.1 < *z {
            z_range.1 = *z;
        }
        if w_range.0 > *w {
            w_range.0 = *w;
        }
        if w_range.1 < *w {
            w_range.1 = *w;
        }
    }
    x_range = (x_range.0 - 1, x_range.1 + 1);
    y_range = (y_range.0 - 1, y_range.1 + 1);
    z_range = (z_range.0 - 1, z_range.1 + 1);
    w_range = (w_range.0 - 1, w_range.1 + 1);
    for x in x_range.0..=x_range.1 {
        for y in y_range.0..=y_range.1 {
            for z in z_range.0..=z_range.1 {
                for w in w_range.0..=w_range.1 {
                    let neighbors = get_neighbors_4d(state, x, y, z, w);
                    if state.contains(&(x, y, z, w)) && (neighbors == 2 || neighbors == 3) {
                        new_state.insert((x, y, z, w));
                    }
                    if !state.contains(&(x, y, z, w)) && neighbors == 3 {
                        new_state.insert((x, y, z, w));
                    }
                }
            }
        }
    }

    new_state
}
fn part2(input: &str) {
    let mut active_cells: HashSet<_> = input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars().enumerate().filter_map(move |(y, c)| {
                if c == '#' {
                    Some((x as isize, y as isize, 0isize, 0isize))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();

    for _ in 0..6 {
        active_cells = update_state_4d(&active_cells);
    }
    println!("Active cells: {}", active_cells.len());
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
