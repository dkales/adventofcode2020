#![feature(str_split_once)]
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day11")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

fn update_state(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut updated = Vec::new();
    let rows = map.len();
    let cols = map[0].len();
    for i in 0..rows {
        let mut row = Vec::new();
        for j in 0..cols {
            let mut neighbours = 0;
            for (x, y) in [
                (0, 0),
                (0, 1),
                (1, 0),
                (0, 2),
                (2, 0),
                (1, 2),
                (2, 1),
                (2, 2),
            ]
            .iter()
            {
                if let Some(a) = map.get(i.wrapping_add(*x).wrapping_sub(1)) {
                    if let Some('#') = a.get(j.wrapping_add(*y).wrapping_sub(1)) {
                        neighbours += 1;
                    }
                }
            }
            match map
                .get(i)
                .expect("have this row")
                .get(j)
                .expect("have this col")
            {
                '#' if neighbours >= 4 => {
                    row.push('L');
                }
                'L' if neighbours == 0 => {
                    row.push('#');
                }
                &a => {
                    row.push(a);
                }
            }
        }
        updated.push(row);
    }
    return updated;
}
fn update_state2(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut updated = Vec::new();
    let rows = map.len();
    let cols = map[0].len();
    for i in 0..rows {
        let mut row = Vec::new();
        for j in 0..cols {
            let mut neighbours = 0;
            for (x, y) in [
                (0, 0),
                (0, 1),
                (1, 0),
                (0, 2),
                (2, 0),
                (1, 2),
                (2, 1),
                (2, 2),
            ]
            .iter()
            {
                let mut x_idx = i.wrapping_add(*x).wrapping_sub(1);
                let mut y_idx = j.wrapping_add(*y).wrapping_sub(1);
                loop {
                    match map.get(x_idx) {
                        Some(a) => match a.get(y_idx) {
                            Some('#') => {
                                neighbours += 1;
                                break;
                            }
                            Some('.') => {
                                x_idx = x_idx.wrapping_add(*x).wrapping_sub(1);
                                y_idx = y_idx.wrapping_add(*y).wrapping_sub(1);
                            }
                            _ => break,
                        },
                        None => break,
                    }
                }
            }
            match map
                .get(i)
                .expect("have this row")
                .get(j)
                .expect("have this col")
            {
                '#' if neighbours >= 5 => {
                    row.push('L');
                }
                'L' if neighbours == 0 => {
                    row.push('#');
                }
                &a => {
                    row.push(a);
                }
            }
        }
        updated.push(row);
    }
    return updated;
}

fn update_until_no_more_changes(
    map: &[Vec<char>],
    update_func: &dyn Fn(&[Vec<char>]) -> Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    let mut map = update_func(map);
    loop {
        let updated = update_func(&map);
        let rows = map.len();
        let cols = map[0].len();
        let mut eq = true;
        for i in 0..rows {
            for j in 0..cols {
                if map[i][j] != updated[i][j] {
                    eq = false;
                }
            }
        }
        if eq {
            return map;
        }
        //println!(
        //"{}\n",
        //map.iter().map(|x| x.iter().collect::<String>()).join("\n")
        //);
        map = updated;
    }
}

fn part1(map: &[Vec<char>]) {
    let final_state = update_until_no_more_changes(map, &update_state);
    println!(
        "Have {} occupied seats.",
        final_state.iter().flatten().filter(|&&x| x == '#').count()
    );
}

fn part2(map: &[Vec<char>]) {
    let final_state = update_until_no_more_changes(map, &update_state2);
    println!(
        "Have {} occupied seats.",
        final_state.iter().flatten().filter(|&&x| x == '#').count()
    );
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    let map: Vec<Vec<char>> = content
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect();

    part1(&map);

    part2(&map);
}
