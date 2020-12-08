use regex::Regex;
use std::collections::*;
use std::fs::read_to_string;

#[derive(Debug)]
struct SubBags<'a> {
    count: i32,
    color: &'a str,
}

pub fn main() {
    let text = read_to_string("inputs/day7.in").unwrap();
    let re = Regex::new(r"(.*) bag(s?) contain(s?) (.*)").unwrap();
    let re2 = Regex::new(r"(\d) (.*) bag(s?)").unwrap();

    let lines: HashMap<_, _> = text
        .trim()
        .split("\n")
        .map(|x| {
            let matches = re.captures(&x).unwrap();
            let subbags = matches
                .get(4)
                .unwrap()
                .as_str()
                .split(", ")
                .filter_map(|p| re2.captures(p))
                .map(|subbag| SubBags {
                    count: i32::from_str_radix(&subbag[1], 10).unwrap(),
                    color: subbag.get(2).unwrap().as_str(),
                })
                .collect::<Vec<_>>();

            (matches[1].to_string(), subbags)
        })
        .collect();

    let mut has_shiny_gold: HashMap<String, bool> = HashMap::new();
    has_shiny_gold.insert("shiny gold".to_string(), true);
    let counts = lines
        .iter()
        .filter(|(color, _)| visit1(color, &lines, &mut has_shiny_gold))
        .count()
        - 1;
    println!("{}", counts);

    let mut subbags: HashMap<String, i32> = HashMap::new();
    let counts = visit2(&"shiny gold".to_string(), &lines, &mut subbags) - 1;
    println!("{:?}", counts);
}

fn visit1<'a>(
    color: &'a str,
    subbags: &HashMap<String, Vec<SubBags>>,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if cache.contains_key(color) {
        return *cache.get(color).unwrap();
    }

    let r = subbags
        .get(color)
        .unwrap()
        .iter()
        .any(|subbag| visit1(&subbag.color, subbags, cache));

    cache.insert(color.to_string(), r);
    r
}

fn visit2<'a>(
    color: &'a str,
    subbags: &HashMap<String, Vec<SubBags>>,
    cache: &mut HashMap<String, i32>,
) -> i32 {
    if cache.contains_key(color) {
        return *cache.get(color).unwrap();
    }

    let r = 1 + subbags
        .get(color)
        .unwrap()
        .iter()
        .map(|subbag| subbag.count * visit2(&subbag.color, subbags, cache))
        .sum::<i32>();

    cache.insert(color.to_string(), r);
    r
}
