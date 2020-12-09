use itertools::*;
use std::collections::*;
use std::fs::read_to_string;

pub fn main() {
    let text = read_to_string("inputs/day9.in").unwrap();

    let numbers: Vec<_> = text
        .trim()
        .split("\n")
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

    println!("{:?}", target);

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
                println!("{}", x + y);
                break;
            }
        }
    }
}
