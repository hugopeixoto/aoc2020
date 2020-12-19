#![feature(test)]
extern crate test;

use itertools::*;

pub fn day05(input: String) -> (usize, usize) {
    let mut lines = input
        .lines()
        .map(|x| x.chars().fold(0, |acc, c| acc * 2 + match c { 'B' | 'R' => 1, _ => 0 }))
        .collect::<Vec<_>>();

    lines.sort();

    let p1 = lines[lines.len() - 1];

    let p2 = lines
        .iter()
        .tuple_windows()
        .filter(|&(prev, current)| current - prev > 1)
        .map(|(_, current)| current - 1)
        .next()
        .unwrap();

    (p1, p2)
}

aoc2020::day!(day05, "day05.in", bench_day05);
