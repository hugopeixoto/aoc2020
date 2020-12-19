#![feature(test)]
extern crate test;

pub fn day02(input: String) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut low = 0;
    let mut high = 0;
    let mut state = 0;
    let mut i = 0;
    let mut matches1 = 0;
    let mut matches2 = 0;
    let mut character: char = '\0';

    for c in input.chars() {
        match (c, state) {
            ('\n', _) => {
                p1 += (low <= matches1 && matches1 <= high) as usize;
                p2 += (matches2 == 1) as usize;

                low = 0;
                high = 0;
                state = 0;
                i = 0;
                matches1 = 0;
                matches2 = 0;
            },
            ('0'..='9', 0) => {
                low = low * 10 + (c as usize - '0' as usize);
            },
            ('0'..='9', _) => {
                high = high * 10 + (c as usize - '0' as usize);
            },
            ('-', _) => { state = 1; },
            ('a'..='z', 1) => {
                character = c;
                state = 2;
            },
            ('a'..='z', _) => {
                if c == character {
                    matches1 += 1;
                    if i+1 == low || i+1 == high { matches2 += 1 }
                }

                i += 1;
            },
            _ => {}
        }
    }

    (p1, p2)
}

aoc2020::day!(day02, "day02.in", bench_day02);
