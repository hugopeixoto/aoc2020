#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

pub fn encrypt(subject: u64, privkey: u64) -> u64 {
    let mut value = 1;
    for _ in 0..privkey {
        value = (value * subject) % 20201227;
    }

    value
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

    let cardprivkey = decrypt(card);
    let doorprivkey = decrypt(door);

    let enc1 = encrypt(door, cardprivkey);
    let enc2 = encrypt(card, doorprivkey);

    (enc1, enc2)
}

aoc2020::day!(day25, "day25.in", bench_day25);
