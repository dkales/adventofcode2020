#![feature(str_split_once)]
use core::panic;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day12")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), default_value = "input")]
    input: PathBuf,
}

struct Ship {
    coords: (isize, isize),
    dir: usize,
}

impl Ship {
    fn new() -> Self {
        Ship {
            coords: (0, 0),
            dir: 0,
        }
    }

    fn process_instruction(&mut self, inst: &str) {
        let mut c = inst.chars().next().expect("valid instruction");
        let num = inst[1..].parse::<isize>().expect("valid instruction");
        if c == 'F' {
            c = match self.dir {
                0 => 'E',
                90 => 'N',
                180 => 'W',
                270 => 'S',
                _ => panic!("invalid ship state"),
            };
        }
        match c {
            'N' => {
                self.coords.1 += num;
            }
            'S' => {
                self.coords.1 -= num;
            }
            'E' => {
                self.coords.0 += num;
            }
            'W' => {
                self.coords.0 -= num;
            }
            'L' => self.dir = (self.dir + num as usize).rem_euclid(360),
            'R' => self.dir = (self.dir + 360 - num as usize).rem_euclid(360),
            _ => panic!("invalid instruction"),
        }
    }

    fn manhattan_distance(&self) -> usize {
        self.coords.0.abs() as usize + self.coords.1.abs() as usize
    }
}

fn part1(input: &str) {
    let mut ship = Ship::new();
    input.lines().for_each(|x| ship.process_instruction(x));
    println!(
        "Ship moved a manhattan distance of {}",
        ship.manhattan_distance()
    );
}
struct Ship2 {
    coords: (isize, isize),
    waypoint: (isize, isize),
}

impl Ship2 {
    fn new() -> Self {
        Ship2 {
            coords: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn process_instruction(&mut self, inst: &str) {
        let c = inst.chars().next().expect("valid instruction");
        let mut num = inst[1..].parse::<isize>().expect("valid instruction");
        match c {
            'F' => {
                self.coords.0 += num * self.waypoint.0;
                self.coords.1 += num * self.waypoint.1;
            }
            'N' => {
                self.waypoint.1 += num;
            }
            'S' => {
                self.waypoint.1 -= num;
            }
            'E' => {
                self.waypoint.0 += num;
            }
            'W' => {
                self.waypoint.0 -= num;
            }
            'L' | 'R' => {
                if c == 'R' {
                    num = -num;
                }
                let rad = num as f64 * std::f64::consts::PI / 180.0;
                let x = self.waypoint.0 * rad.cos() as isize - self.waypoint.1 * rad.sin() as isize;
                let y = self.waypoint.0 * rad.sin() as isize + self.waypoint.1 * rad.cos() as isize;
                self.waypoint = (x, y);
            }
            _ => panic!("invalid instruction"),
        }
    }

    fn manhattan_distance(&self) -> usize {
        self.coords.0.abs() as usize + self.coords.1.abs() as usize
    }
}

fn part2(input: &str) {
    let mut ship2 = Ship2::new();
    input.lines().for_each(|x| ship2.process_instruction(x));
    println!(
        "Ship2 moved a manhattan distance of {}",
        ship2.manhattan_distance()
    );
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
