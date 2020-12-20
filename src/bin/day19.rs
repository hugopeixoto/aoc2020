#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

use std::collections::*;
use regex::Regex;

#[derive(Debug, Clone)]
enum Rule {
    Index(usize),
    Literal(String),
}

fn flatten(rules: &Vec<Vec<Vec<Rule>>>, idx: usize, cache: &mut HashMap<usize, String>, depth: usize) -> String {
    if cache.contains_key(&idx) {
        return cache[&idx].clone();
    }

    let v = rules[idx].iter()
            .map(|ruleset| ruleset.iter().map(|x| {
                match x {
                    Rule::Index(sidx) => flatten(rules, *sidx, cache, depth + 1),
                    Rule::Literal(s) => s.clone().replace("\"", ""),
                }
            }).fold_first(|acc, x| acc + &x).unwrap()
             )
            .fold_first(|acc, x| format!("({}|{})", acc, x) ).unwrap();

    cache.insert(idx, v.to_string());
    v
}


pub fn day19(input: String) -> (usize, usize) {
    let mut parts = input.split("\n\n");

    let rulesstr = parts.next().unwrap();
    let messagesstr = parts.next().unwrap();

    let mut rules = vec![];
    rules.resize(rulesstr.lines().count().max(43), vec![]);
    for rule in rulesstr.lines() {
        let idx = rule.split(": ").nth(0).unwrap().parse::<usize>().unwrap();
        let x = rule.split(": ").nth(1).unwrap();

        let mut ors = vec![];

        for or in x.split(" | ") {
            ors.push(or.split(" ").map(|x| x.parse::<usize>().map(|y| Rule::Index(y)).unwrap_or(Rule::Literal(x.to_string()))).collect::<Vec<_>>());
        }

        rules[idx] = ors;
    }

    let mut cache: HashMap<usize, String> = HashMap::new();

    flatten(&rules, 0, &mut cache, 0);

    let re = Regex::new(&format!("^{}$", &cache[&0])).unwrap();
    let re8 = Regex::new(&format!("^{}+$", &cache[&8])).unwrap();

    let mut p1 = 0;
    for message in messagesstr.lines() {
        if re.is_match(message) {
            p1 += 1;
        }
    }

    let mut p2 = 0;
    for message in messagesstr.lines() {
        'x: for i in 1..message.len() {
            let p8 = &message[0..i];
            if re8.is_match(p8) {
                let p11 = &message[i..];
                for k in 1..=p11.len()/2 {
                    let re11 = Regex::new(&format!(
                            "^({}){{{}}}({}){{{}}}$",
                            &cache[&42], k, &cache[&31], k)).unwrap();

                    if re11.is_match(p11) {
                        p2 += 1;
                        break 'x;
                    }
                }
            }
        }
    }

    (p1, p2)
}

aoc2020::day!(day19, "day19.in", bench_day19);
