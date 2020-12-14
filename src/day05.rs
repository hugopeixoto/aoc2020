use itertools::*;
use std::fs::read_to_string;

pub fn main() {
    let input = read_to_string("inputs/day5.in").unwrap();

    let mut lines = input
        .trim()
        .split('\n')
        .map(|x| {
            let mut n = 0;

            for c in x.chars() {
                n = n*2 + match c {
                    'B' => 1,
                    'R' => 1,
                    _ => 0,
                };
            }

            n
        })
        .collect::<Vec<_>>();

    lines.sort();

    println!("{}", lines[lines.len() - 1]);

    let missing = lines
        .iter()
        .tuple_windows()
        .filter(|&(prev, current)| current - prev > 1)
        .map(|(_, current)| current - 1)
        .next()
        .unwrap();

    println!("{}", missing);
}
