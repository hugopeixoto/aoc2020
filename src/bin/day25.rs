#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

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

pub fn encrypt(subject: u64, privkey: u64) -> u64 {
    modpow(subject, privkey, 20201227)
}

pub fn decrypt(pubkey: u64) -> u64 {
    let mut loopsize = 0;
    let subject = 7;
    let mut value = 1;

    loop {
        if value == pubkey {
            break;
        }

        loopsize += 1;
        value = (value * subject) % 20201227;
    }

    loopsize
}

pub fn day25(input: String) -> (u64, u64) {
    let mut lines = input.lines();

    let card = lines.next().unwrap().parse::<u64>().unwrap();
    let door = lines.next().unwrap().parse::<u64>().unwrap();

    let enc1 = encrypt(door, decrypt(card));

    (enc1, 0)
}

aoc2020::day!(day25, "day25.in", bench_day25);
