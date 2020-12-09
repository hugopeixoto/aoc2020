use std::fs::read_to_string;
use itertools::*;

pub fn main() {
    let text = read_to_string("inputs/day9.in").unwrap();

    let numbers: Vec<_> = text
        .trim()
        .split("\n")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    println!("{}", numbers.len());

    let n = 25;
    let mut target = 0;
    for i in n..numbers.len() {
        let mut found = false;
        'out: for j in 0..n {
            for k in 0..n {
                if j != k && numbers[i - j - 1] + numbers[i - k - 1] == numbers[i] {
                    found = true;
                    break 'out;
                }
            }
        }

        if !found {
            target = numbers[i];
            break;
        }
    }

    println!("{:?}", target);

    let mut accum: Vec<_> = Vec::new();
    accum.resize(numbers.len() + 1, 0);

    for i in 0..numbers.len() {
        accum[i + 1] = accum[i] + numbers[i];
    }

    for i in 0..numbers.len() {
        for j in (i+1)..numbers.len() {
            if accum[j] - accum[i] == target {
                if let MinMaxResult::MinMax(x, y) = numbers[i..j].iter().minmax() {
                    println!("{}", x + y);
                }
            }
        }
    }
}
