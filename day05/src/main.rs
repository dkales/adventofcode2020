use std::{collections::HashSet, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day05")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

struct Seat {
    seat_id: usize,
}

impl Seat {
    fn from_string(input: &str) -> Self {
        let seat_id = input
            .chars()
            .map(|x| match x {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => panic!("invalid input"),
            })
            .fold(0usize, |sum, bit| sum * 2 + bit);
        Seat { seat_id }
    }
}

fn part1(seats: &[Seat]) {
    let highest_seat = seats
        .iter()
        .max_by(|a, b| a.seat_id.cmp(&b.seat_id))
        .expect("we should have a highest seat id");
    println!("Highest seat id = {}", highest_seat.seat_id);
}
fn part2(seats: &[Seat]) {
    let highest_seat = seats
        .iter()
        .max_by(|a, b| a.seat_id.cmp(&b.seat_id))
        .expect("we should have a highest seat id");
    let lowest_seat = seats
        .iter()
        .min_by(|a, b| a.seat_id.cmp(&b.seat_id))
        .expect("we should have a lowest seat id");
    let seats: HashSet<_> = seats.iter().map(|x| x.seat_id).collect();
    for seat_id in lowest_seat.seat_id..=highest_seat.seat_id {
        if !seats.contains(&seat_id)
            && seats.contains(&(seat_id - 1))
            && seats.contains(&(seat_id + 1))
        {
            println!("Found my seat id: {}", seat_id);
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    let seats: Vec<_> = content.lines().map(Seat::from_string).collect();

    part1(&seats);

    part2(&seats);
}
