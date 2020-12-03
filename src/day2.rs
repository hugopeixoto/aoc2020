use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

struct Password {
    low: i32,
    high: i32,
    character: char,
    password: String,
}

pub fn main() {
    let file = File::open("inputs/day2.in").unwrap();
    let re = Regex::new(r"(\d*)-(\d*) (.): (.*)").unwrap();

    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let captures = re.captures(&x).unwrap();

            Password {
                low: i32::from_str_radix(captures.get(1).unwrap().as_str(), 10).unwrap(),
                high: i32::from_str_radix(captures.get(2).unwrap().as_str(), 10).unwrap(),
                character: captures.get(3).unwrap().as_str().chars().next().unwrap(),
                password: captures.get(4).unwrap().as_str().clone().to_string(),
            }
        })
        .collect();

    println!(
        "{}",
        lines
            .iter()
            .filter(|entry| {
                (entry.low..=entry.high).contains(
                    &(entry
                        .password
                        .chars()
                        .filter(|x| x == &entry.character)
                        .count() as i32),
                )
            })
            .count()
    );

    println!(
        "{}",
        lines
            .iter()
            .filter(|entry| {
                (entry
                    .password
                    .chars()
                    .nth((entry.low - 1) as usize)
                    .unwrap()
                    == entry.character)
                    != (entry
                        .password
                        .chars()
                        .nth((entry.high - 1) as usize)
                        .unwrap()
                        == entry.character)
            })
            .count()
    );
}
