#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(test)]
extern crate test;

use std::collections::*;

#[derive(Debug, Copy, Clone)]
struct Entry {
    fixed: u64,
    floating: u64,
    value: u64,
}

impl core::fmt::Display for Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", (0..36).map(|i| match (1&(self.floating >> (36-1-i)), 1&(self.fixed >> (36-1-i))) { (1, _) => 'X', (_, 0) => '0', (_, _) => '1' }).collect::<String>())
    }
}

// 001XX
// -----
// 00101 => 00101 vs 00100, 00110, 00111 => 00101 vs 0011X | 00100
// 0010X => 0010X vs 0011X               => 0010X vs 0011X
// X0100 => X0100 vs 00101, 00110, 00111 => X0100 vs 0011X | 00101
// 10100 => 10100 vs 001XX               => 10100 vs 001XX
// X010X => X010X vs 0011X               => X010X vs 0011X
// 001XX => 001XX vs ø                   => 001XX vs ø
// ================
//          001XX => 0b00100, 0b00011
// ----
// POTATO   00101 => 0b00101, 0b00000 => 001X0 | 00111
// POTATO   X0101 => 0b00101, 0b10000 => 001X0 | 00111
// POTATO   X0100 => 0b00100, 0b10000 => 001X1 | 00110
// POTATO   0010X => 0b00100, 0b00001 => 0011X
// POTATO   X010X => 0b00100, 0b10001 => 0011X
// POTATO v 10100 => 0b10100, 0b00000 => 001XX
// POTATO v 001XX => 0b00100, 0b00011 => ø
//
// 01XXX vs 01010
// 01XX1    00001 # 01__1 | _____
// 01X00    00010 # 01_0_ | ____0
// 01110    00100 # 011__ | ___10
//
// 001XX vs 00101
// 001X0
// 00111
fn gen3(entry: &Entry, new_entry: &Entry) -> Vec<Entry> {
    if entry.floating == new_entry.floating && entry.fixed == new_entry.fixed {
        return vec![];
    }

    if (new_entry.fixed & !entry.floating) != (entry.fixed & !new_entry.floating) {
        return vec![*entry];
    }

    let xes = entry.floating & !new_entry.floating;
    let common_xes = entry.floating & new_entry.floating;

    let mut r = vec![];
    let mut accum = 0;
    for i in 0..36 {
        if 1&(xes >> i) != 0 {
            let keeps = (1 << (i+1)) - 1;
            r.push(Entry {
                fixed: entry.fixed | (!(new_entry.fixed) & (1 << i)) | accum,
                floating: (xes & !keeps) | common_xes,
                value: entry.value,
            });
            accum |= new_entry.fixed & (1 << i);
        }
    }

    r
}

pub fn day14(input: String) -> (u64, u64) {
    let mut memory1: HashMap<u64, u64> = HashMap::new();
    let mut memory2: Vec<Entry> = Vec::new();

    let mut mask_fixed: u64 = 0;
    let mut mask_floating: u64 = 0;

    for line in input.lines() {
        if line.starts_with("mask = ") {
            let poop = line.split(" = ").nth(1).unwrap();

            mask_fixed = poop.chars().fold(0, |acc, n: char| acc * 2 + (n == '1') as u64);
            mask_floating = poop.chars().fold(0, |acc, n: char| acc * 2 + (n == 'X') as u64);
        }

        if line.starts_with("mem[") {
            let mut pieces = line[4..].split("] = ");

            let addr = pieces.next().unwrap().parse::<u64>().unwrap();
            let value = pieces.next().unwrap().parse::<u64>().unwrap();
            let mvalue = (value & mask_floating) | mask_fixed;

            let new_entry = Entry {
                fixed: ((addr | mask_fixed) & !mask_floating),
                floating: mask_floating,
                value: value,
            };

            memory1.insert(addr, mvalue);

            let mut newentries = vec![];
            for entry in memory2 {
                for r in gen3(&entry, &new_entry) {
                    newentries.push(r);
                }
            }

            memory2 = newentries;
            memory2.push(new_entry);
        }
    }

    let mut p2 = 0;
    for e in memory2.iter() {
        p2 += e.value * (1 << e.floating.count_ones());
    }

    (
        memory1.values().sum(),
        p2,
    )
}

aoc2020::day!(day14, "day14.in", bench_day14);
