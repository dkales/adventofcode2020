use std::{collections::HashMap, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "day04")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

struct Passport {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    _country_id: Option<usize>,
}

impl Passport {
    fn from_string(input: &str) -> Option<Self> {
        let data: HashMap<_, _> = input
            .split_ascii_whitespace()
            .map(|x| {
                let tmp: Vec<_> = x.split(":").collect();
                (tmp[0], tmp[1])
            })
            .collect();
        if data.contains_key("byr")
            && data.contains_key("iyr")
            && data.contains_key("eyr")
            && data.contains_key("hgt")
            && data.contains_key("hcl")
            && data.contains_key("ecl")
            && data.contains_key("pid")
        {
            return Some(Passport {
                birth_year: data["byr"].parse().expect("invalid byr format"),
                issue_year: data["iyr"].parse().expect("invalid iyr format"),
                expiration_year: data["eyr"].parse().expect("invalid eyr format"),
                height: data["hgt"].to_string(),
                hair_color: data["hcl"].to_string(),
                eye_color: data["ecl"].to_string(),
                passport_id: data["pid"].to_string(),
                _country_id: if data.contains_key("cid") {
                    Some(data["cid"].parse().expect("invalid cid format"))
                } else {
                    None
                },
            });
        }
        return None;
    }
    fn validate(&self) -> bool {
        if self.birth_year < 1920 || self.birth_year > 2002 {
            return false;
        }
        if self.issue_year < 2010 || self.issue_year > 2020 {
            return false;
        }
        if self.expiration_year < 2020 || self.expiration_year > 2030 {
            return false;
        }
        match self.eye_color.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
            _ => return false,
        }
        if self.hair_color.chars().next() != Some('#')
            || self.hair_color.chars().skip(1).count() != 6
            || !self.hair_color.chars().skip(1).all(|x| match x {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' | 'b' | 'c'
                | 'd' | 'e' | 'f' => true,
                _ => false,
            })
        {
            return false;
        }
        if self.passport_id.chars().count() != 9
            || !self.passport_id.chars().all(|x| match x {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
                _ => false,
            })
        {
            return false;
        }
        if !(self.height.ends_with("cm") || self.height.ends_with("in")) {
            return false;
        }
        if self.height.ends_with("cm") {
            match self.height[..self.height.len() - 2].parse::<usize>() {
                Ok(h) => {
                    if h < 150 || h > 193 {
                        return false;
                    }
                }
                Err(_) => return false,
            }
        }
        if self.height.ends_with("in") {
            match self.height[..self.height.len() - 2].parse::<usize>() {
                Ok(h) => {
                    if h < 59 || h > 76 {
                        return false;
                    }
                }
                Err(_) => return false,
            }
        }

        return true;
    }
}

fn part1(content: &str) {
    let content = content.replace("\r\n", "\n");
    let passports: Vec<_> = content
        .split("\n\n")
        .map(|x| Passport::from_string(x))
        .filter_map(|x| x)
        .collect();
    println!("Got {} valid passports for part 1.", passports.len());
}
fn part2(content: &str) {
    let content = content.replace("\r\n", "\n");
    let passports: Vec<_> = content
        .split("\n\n")
        .map(|x| Passport::from_string(x))
        .filter_map(|x| x)
        .filter(|x| x.validate())
        .collect();
    println!("Got {} valid passports for part 2.", passports.len());
}

fn main() {
    let opt = Opt::from_args();
    let content = std::fs::read_to_string(opt.input).expect("could not read input file");

    part1(&content);

    part2(&content);
}
