use std::fs::File;
use std::io::{self, BufRead};

pub fn main() {
    let file = File::open("inputs/day1.in").unwrap();

    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| i32::from_str_radix(&x, 10).unwrap())
        .collect();

    'part1: for a in lines.iter() {
        for b in lines.iter() {
            if a + b == 2020 {
                println!("{}", a * b);
                break 'part1;
            }
        }
    }

    'part2: for a in lines.iter() {
        for b in lines.iter() {
            for c in lines.iter() {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    break 'part2;
                }
            }
        }
    }
}
