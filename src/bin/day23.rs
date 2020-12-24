#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

fn next(ns: &mut Vec<usize>, current: usize) -> usize {
    let next1 = ns[current];
    let next2 = ns[next1];
    let next3 = ns[next2];
    let next4 = ns[next3];

    let k = ns.len() - 1;

    let target = (1..=4)
        .map(|i| (current - 1 + k - i) % k + 1)
        .find(|&target| target != next1 && target != next2 && target != next3)
        .unwrap();

    ns[next3] = ns[target];
    ns[target] = ns[current];
    ns[current] = next4;

    next4
}

pub fn day23(input: String) -> (usize, u64) {
    let numbers = input.trim().chars().map(|c| c as usize - '0' as usize).collect::<Vec<_>>();

    let mut positions = Vec::new();
    positions.resize(numbers.len() + 1, 0);

    for i in 1..=numbers.len() {
        positions[numbers[i - 1]] = numbers[i%numbers.len()];
    }

    let mut target = numbers[0];
    for _ in 0..100 {
        target = next(&mut positions, target);
    }

    let mut p1 = 0;
    let mut n = 1;
    for _ in 1..9 {
        n = positions[n];
        p1 = p1*10 + n;
    }

    let width = 1_000_000;
    let mut numbers2 = numbers.clone();
    numbers2.resize(width, 0);
    for i in numbers.len()..width {
        numbers2[i] = i + 1;
    }

    let mut positions2 = Vec::new();
    positions2.resize(width + 1, 0);

    for i in 1..=numbers2.len() {
        positions2[numbers2[i - 1]] = numbers2[i%numbers2.len()];
    }

    let mut target = numbers[0];
    for _ in 0..10_000_000 {
        target = next(&mut positions2, target);
    }

    let p2 = positions2[1] as u64 * positions2[positions2[1]] as u64;

    (p1, p2)
}

aoc2020::day!(day23, "day23.in", bench_day23);
