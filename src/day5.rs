use std::fs::File;
use std::io::{self, BufRead};

pub fn main() {
    let file = File::open("inputs/day5.in").unwrap();

    let mut lines = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.replace("F", "0").replace("B", "1").replace("R", "1").replace("L", "0"))
        .map(|x| i32::from_str_radix(&x, 2).unwrap())
        .collect::<Vec<_>>();

    println!("{}", lines.iter().max().unwrap());

    lines.sort();

    let mut previous = None;
    for item in lines.iter() {
        if previous.is_some() && item - previous.unwrap() > 1 {
            println!("{}", item - 1);
            break;
        }

        previous = Some(item);
    }
}
