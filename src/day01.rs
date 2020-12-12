use std::fs::File;
use std::io::{self, BufRead};

pub fn main() {
    let file = File::open("inputs/day1.in").unwrap();

    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| usize::from_str_radix(&x, 10).unwrap())
        .collect();

    let mut set = vec![];
    set.resize(2020*2, false);

    for &n in lines.iter() {
        set[n] = true;
    }

    for &a in lines.iter() {
        if set[2020 - a] {
            println!("{}", a * (2020 - a));
            break;
        }
    }

    'part2: for &a in lines.iter() {
        for &b in lines.iter() {
            if a + b <= 2020 && set[2020 - (a + b)] {
                println!("{}", a * b * (2020 - a - b));
                break 'part2;
            }
        }
    }
}
