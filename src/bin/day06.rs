#![feature(test)]
extern crate test;

pub fn day06(input: String) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut counts = vec![];
    counts.resize(26, 0);

    for entry in input.trim().split("\n\n") {
        let people = entry.lines().count();

        for c in counts.iter_mut() {
            *c = 0;
        }

        for c in entry.chars() {
            match c {
                'a'..='z' => { counts[c as usize - 'a' as usize] += 1; },
                _ => {},
            }
        }

        for &c in counts.iter() {
            if c > 0 {
                p1 += 1;
            }
            if c == people {
                p2 += 1;
            }
        }
    }

    (p1, p2)
}

aoc2020::day!(day06, "day06.in", bench_day06);
