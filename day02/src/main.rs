use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day02")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

struct PasswordPolicy {
    number1: usize,
    number2: usize,
    letter: char,
    password: String,
}

impl PasswordPolicy {
    fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|&x| x == self.letter).count();
        return (count >= self.number1) && (count <= self.number2);
    }
    fn is_valid2(&self) -> bool {
        let chars: Vec<_> = self.password.chars().collect();
        return (chars[self.number1] == self.letter) ^ (chars[self.number2] == self.letter);
    }
}

fn parse_input(input: &str) -> Vec<PasswordPolicy> {
    input
        .lines()
        .map(|x| {
            let idx = x.find("-").expect("invalid format");
            let idx2 = x.find(" ").expect("invalid format");
            let num1: usize = x[0..idx].parse().expect("invalid format");
            let num2: usize = x[idx + 1..idx2].parse().expect("invalid format");
            let c = x.chars().collect::<Vec<_>>()[idx2 + 1];
            let pw = String::from(&x[idx2 + 3..]);
            PasswordPolicy {
                number1: num1,
                number2: num2,
                letter: c,
                password: pw,
            }
        })
        .collect()
}

fn part1(values: &[PasswordPolicy]) {
    let valid_count = values.iter().filter(|&x| x.is_valid()).count();
    println!("{} passwords are valid according to policy 1.", valid_count);
}

fn part2(values: &[PasswordPolicy]) {
    let valid_count = values.iter().filter(|&x| x.is_valid2()).count();
    println!("{} passwords are valid according to policy 2.", valid_count);
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    let passwords = parse_input(&content);

    part1(&passwords);

    part2(&passwords);
}
