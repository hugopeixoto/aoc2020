#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(test)]
extern crate test;

use std::collections::*;

#[derive(Debug, Clone)]
struct Entry {
    fixed: u64,
    floating: u64,
    value: u64,
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
fn cover(entry: &Entry, new_entry: &Entry) -> bool {
    entry.floating == new_entry.floating && entry.fixed == new_entry.fixed
}

fn intersect(entry: &Entry, new_entry: &Entry) -> bool {
    (new_entry.fixed & !entry.floating) == (entry.fixed & !new_entry.floating)
}

fn gen3(entry: &Entry, new_entry: &Entry) -> Vec<Entry> {
    if !intersect(entry, new_entry) {
        return vec![entry.clone()];
    }

    if cover(entry, new_entry) {
        return vec![];
    }

    let xes = entry.floating & !new_entry.floating;
    let common_xes = entry.floating & new_entry.floating;

    let mut r = vec![];
    let mut accum = 0;
    for i in 0..36 {
        if xes & (1 << i) != 0 {
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
            let mask = line.split(" = ").nth(1).unwrap();

            mask_fixed = mask.chars().fold(0, |acc, n: char| acc * 2 + (n == '1') as u64);
            mask_floating = mask.chars().fold(0, |acc, n: char| acc * 2 + (n == 'X') as u64);
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

            let mut s = 0;
            let pl = memory2.len();
            for i in 0..pl {
                if s == i && !intersect(&memory2[i], &new_entry) {
                    s += 1;
                } else {
                    for r in gen3(&memory2[i], &new_entry) {
                        if s <= i {
                            memory2[s] = r;
                            s += 1;
                        } else {
                            memory2.push(r);
                        }
                    }
                }
            }

            memory2.push(new_entry);

            let mut dropped = 0;
            while s < pl {
                memory2[s] = memory2[memory2.len() - 1].clone();
                s += 1;
                dropped += 1;
            }

            memory2.truncate(memory2.len() - dropped);
        }
    }

    let mut p2 = 0;
    for e in memory2.iter() {
        p2 += e.value * (1 << e.floating.count_ones());
    }

    (memory1.values().sum(), p2)
}

aoc2020::day!(day14, "day14.in", bench_day14);
