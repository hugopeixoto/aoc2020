use std::fs::read_to_string;
use std::collections::*;

fn gen2(mask: &[char], addr: u64, x: u64, results: &mut Vec<u64>) {
    if mask.len() == 0 {
        results.push(addr);
    } else {
        match mask[0] {
            'X' => {
                gen2(&mask[1..], addr | (1 << x), x + 1, results);
                gen2(&mask[1..], addr & !(1 << x), x + 1, results);
            }
            '0' => {
                gen2(&mask[1..], addr, x + 1, results);
            },
            '1' => {
                gen2(&mask[1..], addr | (1 << x), x + 1, results);
            },
            _ => {}
        }
    }
}

fn gen(mask: &Vec<char>, addr: u64) -> Vec<u64> {
    let mut r = vec![];
    gen2(&mask, addr, 0, &mut r);
    r
}

pub fn main() {
    let text = read_to_string("inputs/day14.in").unwrap();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut memory2: HashMap<u64, u64> = HashMap::new();

    let mut mask_and: u64 = 0xFFFFFFFFF;
    let mut mask_or = 0;
    let mut mask_floating = vec![];

    for line in text.trim().split('\n') {
        if line.starts_with("mask = ") {
            let poop = line.split(" = ").nth(1).unwrap();

            mask_and = poop.chars().fold(0, |acc, n: char| acc * 2 + (n != '0') as u64);
            mask_or = poop.chars().fold(0, |acc, n: char| acc * 2 + (n == '1') as u64);
            mask_floating = poop.chars().collect::<Vec<_>>();
            mask_floating.reverse();
        }

        if line.starts_with("mem[") {
            let mut pieces = line[4..].split("] = ");

            let addr = pieces.next().unwrap().parse::<u64>().unwrap();
            let value = pieces.next().unwrap().parse::<u64>().unwrap();
            let mvalue = (value & mask_and) | mask_or;

            memory.insert(addr, mvalue);

            for addr2 in gen(&mask_floating, addr) {
                memory2.insert(addr2, value);
            }
        }
    }

    println!("{:?}", memory.iter().fold(0, |acc, (_,v)| acc + v));
    println!("{:?}", memory2.iter().fold(0, |acc, (_,v)| acc + v));
}
