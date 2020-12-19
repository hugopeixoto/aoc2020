#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(test)]
extern crate test;

pub fn day15(input: String) -> (usize, usize) {
    let mut indexes = vec![];
    indexes.resize(30_000_000, -1);

    let mut count = 0;
    for (i, c) in input.trim().split(',').enumerate() {
        let n = c.parse::<usize>().unwrap();
        indexes[n] = i as i32;
        count += 1;
    }

    let mut next = 0;

    while count < 2020 - 1 {
        let n = next;

        next = if indexes[n] < 0 { 0 } else { count - indexes[n] as usize };
        indexes[n] = count as i32;
        count += 1;
    }

    let p1 = next;

    while count < 30_000_000 - 1 {
        let n = next;

        next = if indexes[n] < 0 { 0 } else { count - indexes[n] as usize };
        indexes[n] = count as i32;
        count += 1;
    }

    let p2 = next;

    (p1, p2)
}

aoc2020::day!(day15, "day15.in", bench_day15);
