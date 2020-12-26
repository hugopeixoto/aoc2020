#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

use std::collections::*;

pub fn modpow(mut n: u64, mut e: u64, m: u64) -> u64 {
    n = n%m;
    let mut r = 1;
    while e > 0 {
        if e % 2 == 0 {
            n = (n*n)%m;
        } else {
            r = (r*n)%m;
            n = (n*n)%m;
        }
        e /= 2;
    }

    r
}

pub fn exgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b < a {
        exgcd(b, a)
    } else { // a < b
        let (mut old_r, mut r) = (a, b);
        let (mut old_s, mut s) = (1, 0);
        let (mut old_t, mut t) = (0, 1);

        while r != 0 {
            let quotient = old_r / r;
            (old_r, r) = (r, old_r - quotient * r);
            (old_s, s) = (s, old_s - quotient * s);
            (old_t, t) = (t, old_t - quotient * t);
        }

        (old_r, old_s, old_t)
    }
}


pub fn encrypt(subject: u64, privkey: u64) -> u64 {
    modpow(subject, privkey, 20201227)
}

pub fn decrypt(n: u64) -> Option<u64> {
    const MOD: u64 = 20201227;
    const ALPHA: u64 = 7;

    let m = (n as f64).sqrt().ceil() as usize;

    let mut alphas = HashMap::new();

    let mut aj = 1;
    for j in 0..m {
        alphas.insert(aj, j);
        aj = (aj * ALPHA) % MOD;
    }

    let alpha_to_minus_m = modpow(ALPHA, MOD - m as u64 - 1, MOD);

    let mut gamma = n;
    for i in 0..m as u64 {
        if let Some(&j) = alphas.get(&gamma) {
            return Some(i * (m as u64) + j as u64);
        }

        gamma = (gamma * alpha_to_minus_m) % MOD;
    }

    None
}

pub fn day25(input: String) -> (u64, u64) {
    let mut lines = input.lines();

    let card = lines.next().unwrap().parse::<u64>().unwrap();
    let door = lines.next().unwrap().parse::<u64>().unwrap();

    let enc1 = encrypt(door, decrypt(card).unwrap());

    (enc1, 0)
}

aoc2020::day!(day25, "day25.in", bench_day25);
