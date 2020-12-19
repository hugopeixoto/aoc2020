#![feature(or_patterns)]
#![feature(test)]
extern crate test;

use itertools::*;
use std::collections::*;

pub fn day09(input: String) -> (i64, i64) {
    let numbers: Vec<_> = input
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let n = 25;
    let mut target = 0;
    let mut seen: HashSet<i64> = numbers[0..n].iter().cloned().collect();

    for i in n..numbers.len() {
        seen.insert(numbers[i - 1]);

        let found = ((i - n)..i)
            .any(|j| numbers[i] > numbers[j] && seen.contains(&(numbers[i] - numbers[j])));

        if !found {
            target = numbers[i];
            break;
        }

        seen.remove(&numbers[i - n]);
    }

    let p1 = target;

    let mut lower = 0;
    let mut current = 0;
    for i in 0..numbers.len() {
        current += numbers[i];
        while current > target {
            current -= numbers[lower];
            lower += 1;
        }

        if current == target && lower != i {
            if let MinMaxResult::MinMax(x, y) = numbers[lower..i].iter().minmax() {
                return (p1, x + y);
            }
        }
    }

    (0, 0)
}

aoc2020::day!(day09, "day09.in", bench_day09);
