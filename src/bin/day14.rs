#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(test)]
extern crate test;

use std::collections::*;

fn gen2(mask_floating: u64, addr: u64, x: u64, results: &mut Vec<u64>) {
    if x == 36 {
        results.push(addr);
    } else {
        match mask_floating & 1 {
            0 => {
                gen2(mask_floating >> 1, addr | (0 << x), x + 1, results);
            },
            1 => {
                gen2(mask_floating >> 1, addr | (0 << x), x + 1, results);
                gen2(mask_floating >> 1, addr | (1 << x), x + 1, results);
            },
            _ => { panic!() },
        }
    }
}

fn gen(mask_floating: u64) -> Vec<u64> {
    let mut r = vec![];
    gen2(mask_floating, 0, 0, &mut r);
    r
}

pub fn day14(input: String) -> (u64, u64) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut memory2: HashMap<u64, u64> = HashMap::new();

    let mut mask_fixed: u64 = 0;
    let mut mask_floating: u64 = 0;
    let mut mask_floating2 = vec![];

    for line in input.lines() {
        if line.starts_with("mask = ") {
            let poop = line.split(" = ").nth(1).unwrap();

            mask_fixed = poop.chars().fold(0, |acc, n: char| acc * 2 + (n == '1') as u64);
            mask_floating = poop.chars().fold(0, |acc, n: char| acc * 2 + (n == 'X') as u64);
            mask_floating2 = gen(mask_floating);
        }

        if line.starts_with("mem[") {
            let mut pieces = line[4..].split("] = ");

            let addr = pieces.next().unwrap().parse::<u64>().unwrap();
            let value = pieces.next().unwrap().parse::<u64>().unwrap();
            let mvalue = (value & mask_floating) | mask_fixed;

            memory.insert(addr, mvalue);

            for &addr2 in mask_floating2.iter() {
                let a = (addr & !mask_floating | mask_fixed) | addr2;
                memory2.insert(a, value);
            }
        }
    }

    (
        memory.iter().fold(0, |acc, (_,v)| acc + v),
        memory2.iter().fold(0, |acc, (_,v)| acc + v)
    )
}

aoc2020::day!(day14, "day14.in", bench_day14);
