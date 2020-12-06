use regex::Regex;
use std::fs::read_to_string;

use std::fs::File;
use std::io::{self, BufRead};

pub fn main() {
    let text = read_to_string("inputs/day6.in").unwrap();
    let mut count = 0;
    let mut part2 = 0;
    for entry in text.split("\n\n") {
        let people = entry.trim().split("\n").count();

        println!("people: {:?}", people);

        let mut chars = entry.replace("\n", "").chars().collect::<Vec<_>>();

        chars.sort();
        chars.dedup();

        count += chars.len();

        let mut count2 = 0;
        for c in 'a'..='z' {
            let answers = entry.chars().filter(|x| *x == c).count() == people;

            if answers { count2 += 1; }
        }

        println!("c2: {:?}", count2);
        part2 += count2;
    }
    println!("p1: {:?}", count);
    println!("p1: {:?}", part2);
}
