#![feature(or_patterns)]
#![feature(test)]
extern crate test;

use std::collections::*;

#[derive(Debug, Clone, Copy)]
pub struct SubBags {
    count: usize,
    color: usize,
}

pub fn parse_subbags(input: &str, color_index: &HashMap<String, usize>) -> Vec<SubBags> {
    let mut r = vec![];

    let mut state = 0;
    let mut count = 0;
    let mut color: Vec<char> = vec![];
    for c in input.chars() {
        match (c, state) {
            ('0'..='9', 0) => {
                count = c as usize - '0' as usize;
                state = 1;
            },
            ('a'..='z', 2 | 3) => {
                color.push(c);
            },
            (' ', 2) => {
                color.push(c);
                state += 1;
            },
            (' ', 1 | 3) => {
                state += 1;
            },
            (',' | '.', 4) => {
                r.push(SubBags {
                    count,
                    color: color_index[&color.iter().collect::<String>()],
                });
                color.clear();
                state = 0;
            },
            _ => {},
        }
    }

    r
}

pub fn day07(input: String) -> (usize, usize) {
    let mut statements = input
        .lines()
        .map(|x| x.split(" bags contain "))
        .collect::<Vec<_>>();

    let mut color_index = HashMap::with_capacity(1_000);

    for (i, line) in statements.iter_mut().enumerate() {
        color_index.insert(line.next().unwrap().to_string(), i);
    }

    let shiny_gold = color_index["shiny gold"];

    let lines: Vec<_> = statements
        .iter_mut()
        .map(|parts| parse_subbags(parts.next().unwrap(), &color_index))
        .collect();

    let mut has_shiny_gold: HashMap<usize, bool> = HashMap::new();
    has_shiny_gold.insert(shiny_gold, true);
    let p1 = (0..lines.len())
        .filter(|color| visit1(*color, &lines, &mut has_shiny_gold))
        .count()
        - 1;

    let p2 = visit2(shiny_gold, &lines, &mut HashMap::new()) - 1;

    (p1, p2)
}

fn visit1<'a>(
    color: usize,
    subbags: &Vec<Vec<SubBags>>,
    cache: &mut HashMap<usize, bool>,
) -> bool {
    let x = cache.get(&color);
    if let Some(&v) = x {
        return v;
    }

    let r = subbags[color]
        .iter()
        .any(|subbag| visit1(subbag.color, subbags, cache));

    cache.insert(color, r);
    r
}

fn visit2<'a>(
    color: usize,
    subbags: &Vec<Vec<SubBags>>,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    let x = cache.get(&color);
    if let Some(&v) = x {
        return v;
    }

    let r = 1 + subbags[color]
        .iter()
        .fold(0, |acc, subbag| acc + subbag.count * visit2(subbag.color, subbags, cache));

    cache.insert(color, r);
    r
}

aoc2020::day!(day07, "day07.in", bench_day07);
