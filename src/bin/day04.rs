#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(test)]
extern crate test;

const fn chr(c: char) -> usize {
    c as usize - 'a' as usize
}

const AMB: usize = chr('a')*26*26 + chr('m')*26 + chr('b');
const BLU: usize = chr('b')*26*26 + chr('l')*26 + chr('u');
const BRN: usize = chr('b')*26*26 + chr('r')*26 + chr('n');
const GRY: usize = chr('g')*26*26 + chr('r')*26 + chr('y');
const GRN: usize = chr('g')*26*26 + chr('r')*26 + chr('n');
const HZL: usize = chr('h')*26*26 + chr('z')*26 + chr('l');
const OTH: usize = chr('o')*26*26 + chr('t')*26 + chr('h');

fn check1(input: &str) -> bool {
    let mut state = 0;
    let mut mask = 0;
    let mut v = 0;

    for c in input.chars() {
        match (c, state, mask) {
            ('h', 0, 0) => { mask = 1; state += 1; },
            ('c', 1, 1) => { state += 1; },
            ('l', 2, 1) => { state += 1; },
            ('g', 1, 1) => { mask = 2; state += 1; },
            ('t', 2, 2) => { state += 1; },
            ('p', 0, 0) => { mask = 4; state += 1; },
            ('i', 1, 4) => { state += 1; },
            ('d', 2, 4) => { state += 1; },
            ('e', 0, 0) => { mask = 8; state += 1; },
            ('c', 1, 8) => { state += 1; },
            ('l', 2, 8) => { state += 1; },
            ('y', 1, 8) => { mask = 16; state += 1; },
            ('r', 2, 16) => { state += 1; },
            ('i', 0, 0) => { mask = 32; state += 1; },
            ('y', 1, 32) => { state += 1; },
            ('r', 2, 32) => { state += 1; },
            ('b', 0, 0) => { mask = 64; state += 1; },
            ('y', 1, 64) => { state += 1; },
            ('r', 2, 64) => { state += 1; },
            (':', 3, _) => { v |= mask; },
            (' ' | '\n', _, _) => { state = 0; mask = 0; },
            _ => {},
        }
    }

    v == 0b1_111_111
}

fn check2(input: &str) -> bool {
    let mut state = 0;
    let mut mask = 0;
    let mut v = 0;
    let mut good = false;
    let mut n = 0;

    let x = format!("{} ", input);

    for c in x.chars() {
        match (c, state, mask) {
            ('h', 0, 0) => { mask = 1; state += 1; },
            ('c', 1, 1) => { state += 1; },
            ('l', 2, 1) => { state += 1; },
            ('g', 1, 1) => { mask = 2; state += 1; },
            ('t', 2, 2) => { state += 1; },
            ('p', 0, 0) => { mask = 4; state += 1; },
            ('i', 1, 4) => { state += 1; },
            ('d', 2, 4) => { state += 1; },
            ('e', 0, 0) => { mask = 8; state += 1; },
            ('c', 1, 8) => { state += 1; },
            ('l', 2, 8) => { state += 1; },
            ('y', 1, 8) => { mask = 16; state += 1; },
            ('r', 2, 16) => { state += 1; },
            ('i', 0, 0) => { mask = 32; state += 1; },
            ('y', 1, 32) => { state += 1; },
            ('r', 2, 32) => { state += 1; },
            ('b', 0, 0) => { mask = 64; state += 1; },
            ('y', 1, 64) => { state += 1; },
            ('r', 2, 64) => { state += 1; },
            (':', 3, _) => { state += 1; },

            ('#', 4, 1) => { state += 1; },
            ('a'..='f' | '0'..='9', 5..=9, 1) => { state += 1; },
            ('a'..='f' | '0'..='9', 10, 1) => { state += 1; good = true; },

            ('0'..='9', 4, 2) => { n = n*10 + c as usize - '0' as usize; },
            ('c', 4, 2) => { state += 1; },
            ('m', 5, 2) => { state += 2; good = (150..=193).contains(&n); },
            ('i', 4, 2) => { state += 2; },
            ('n', 6, 2) => { state += 1; good = (59..=76).contains(&n); },

            ('0'..='9', 4..=11, 4) => { state += 1; },
            ('0'..='9', 12, 4) => { state += 1; good = true; },

            ('a'..='z', 4, 8) => { state += 1; n = chr(c); },
            ('a'..='z', 5, 8) => { state += 1; n = n * 26 + chr(c); },
            ('a'..='z', 6, 8) => { state += 1; n = n * 26 + chr(c); good = n == AMB || n == BLU || n == BRN || n == GRY || n == GRN || n == HZL || n == OTH; },

            ('0'..='9', 4, 16) => { n = n*10 + c as usize - '0' as usize; good = (2020..=2030).contains(&n) },
            ('0'..='9', 4, 32) => { n = n*10 + c as usize - '0' as usize; good = (2010..=2020).contains(&n) },
            ('0'..='9', 4, 64) => { n = n*10 + c as usize - '0' as usize; good = (1920..=2002).contains(&n) },

            (' ' | '\n', _, _) => {
                if good {
                    v |= mask;
                }
                state = 0;
                mask = 0;
                n = 0;
                good = false;
            },
            _ => { good = false; },
        }
    }

    v == 0b1_111_111
}

pub fn day04(input: String) -> (usize, usize) {
    let p1 = input
        .split("\n\n")
        .filter(|passport| check1(passport))
        .count();

    let p2 = input
        .split("\n\n")
        .filter(|passport| check2(passport))
        .count();

    (p1, p2)
}

aoc2020::day!(day04, "day04.in", bench_day04);
