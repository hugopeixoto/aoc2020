#![feature(or_patterns)]
#![feature(test)]
extern crate test;

pub fn day10(input: String) -> (usize, u64) {
    let mut numbers: Vec<_> = input
        .lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    numbers.sort();

    let mut previous = 0;
    let mut ones = 0;
    let mut threes = 1;
    for n in numbers.iter() {
        if n - previous == 1 {
            ones += 1;
        }
        if n - previous == 3 {
            threes += 1;
        }
        previous = *n;
    }

    let p1 = ones * threes;

    let mut ways = vec![];
    ways.resize(numbers.len(), 0);

    for (i, n) in numbers.iter().enumerate() {
        if i >= 1 && n - numbers[i - 1] <= 3 {
            ways[i] += ways[i - 1];
        }
        if i >= 2 && n - numbers[i - 2] <= 3 {
            ways[i] += ways[i - 2];
        }
        if i >= 3 && n - numbers[i - 3] <= 3 {
            ways[i] += ways[i - 3];
        }
        if numbers[i] <= 3 {
            ways[i] += 1;
        }
    }

    (p1, ways[ways.len() - 1])
}

aoc2020::day!(day10, "day10.in", bench_day10);
