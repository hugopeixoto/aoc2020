use std::fs::read_to_string;

pub fn main() {
    let text = read_to_string("inputs/day10.in").unwrap();

    let mut numbers: Vec<_> = text
        .trim()
        .split("\n")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    numbers.sort();

    let mut previous = 0;
    let mut ones = 0;
    let mut threes = 1;
    for n in numbers.iter() {
        if n - previous == 1 { ones += 1; }
        if n - previous == 3 { threes += 1; }
        previous = *n;
    }

    println!("{:?}", ones * threes);

    let mut ways: Vec<u64> = Vec::new();
    ways.resize(numbers.len(), 0);

    for (i, n) in numbers.iter().enumerate() {
        if i >= 1 && n - numbers[i - 1] <= 3 { ways[i] += ways[i - 1]; }
        if i >= 2 && n - numbers[i - 2] <= 3 { ways[i] += ways[i - 2]; }
        if i >= 3 && n - numbers[i - 3] <= 3 { ways[i] += ways[i - 3]; }
        if numbers[i] <= 3 { ways[i] += 1; }
    }

    println!("{:?}", ways[ways.len() - 1]);
}
