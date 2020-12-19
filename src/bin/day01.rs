#![feature(test)]
extern crate test;

pub fn day01(input: String) -> (usize, usize) {
    let lines: Vec<_> = input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut set = vec![];
    set.resize(2020, false);

    for &n in lines.iter() {
        set[n] = true;
    }

    for &a in lines.iter() {
        if set[2020 - a] {
            p1 = a * (2020 - a);
            break;
        }
    }

    'part2: for &a in lines.iter() {
        for &b in lines.iter() {
            if a + b <= 2020 && set[2020 - (a + b)] {
                p2 = a * b * (2020 - a - b);
                break 'part2;
            }
        }
    }

    (p1, p2)
}

aoc2020::day!(day01, "day01.in", bench_day01);
