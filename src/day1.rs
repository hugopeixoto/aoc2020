use std::collections::*;
use std::fs::File;
use std::io::{self, BufRead};

pub fn main() {
    let file = File::open("inputs/day1.in").unwrap();

    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| i32::from_str_radix(&x, 10).unwrap())
        .collect();

    let set: HashSet<_> = lines.iter().collect();

    for a in lines.iter() {
        if set.contains(&(2020 - a)) {
            println!("{}", a * (2020 - a));
            break;
        }
    }

    'part2: for a in lines.iter() {
        for b in lines.iter() {
            if set.contains(&(2020 - a - b)) {
                println!("{}", a * b * (2020 - a - b));
                break 'part2;
            }
        }
    }
}
