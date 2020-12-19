#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(test)]
extern crate test;

pub fn day13(input: String) -> (u64, u64) {
    let mut lines = input.lines();

    let start_time = lines.next().unwrap().parse::<u64>().unwrap();

    let ids = lines.next().unwrap().split(',').collect::<Vec<_>>();

    let mut minimum = start_time;
    let mut minimal = 0;

    let mut delta = 0;
    let mut base = 0;
    let mut multiplier = 1;

    for i in ids.iter() {
        if *i != "x" {
            let curr = i.parse::<u64>().unwrap();
            for k in 0..curr {
                if (base + multiplier * k + delta) % curr == 0 {
                    base += multiplier * k;
                    multiplier *= curr;
                    break;
                }
            }

            let v = (curr - (start_time % curr)) % curr;
            if v < minimum {
                minimum = v;
                minimal = curr;
            }
        }
        delta += 1;
    }

    (minimum * minimal, base)
}

aoc2020::day!(day13, "day13.in", bench_day13);
