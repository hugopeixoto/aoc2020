use regex::Regex;
use std::collections::*;
use std::fs::read_to_string;

#[derive(Debug)]
struct SubBags {
    count: i32,
    color: usize,
}

pub fn main() {
    let text = read_to_string("inputs/day7.in").unwrap();
    let re = Regex::new(r"(.*) bag(s?) contain(s?) (.*)").unwrap();
    let re2 = Regex::new(r"(\d) (.*) bag(s?)").unwrap();

    let mut color_index: HashMap<String, usize> = HashMap::new();
    let mut get_index = |color: String| {
        if !color_index.contains_key(&color) {
            let r = color_index.len();
            color_index.insert(color, r);
            r
        } else {
            color_index[&color]
        }
    };

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
                    count: i32::from_str_radix(&subbag.get(1).unwrap().as_str(), 10).unwrap(),
                    color: get_index(subbag.get(2).unwrap().as_str().to_string()),
                })
                .collect::<Vec<_>>();

            (
                get_index(matches.get(1).unwrap().as_str().to_string()),
                subbags,
            )
        })
        .collect();

    let mut has_shiny_gold: HashMap<usize, bool> = HashMap::new();
    has_shiny_gold.insert(get_index("shiny gold".to_string()), true);
    let counts = lines
        .iter()
        .filter(|(color, _)| visit1(**color, &lines, &mut has_shiny_gold))
        .count()
        - 1;
    println!("{}", counts);

    let mut subbags: HashMap<usize, i32> = HashMap::new();
    let counts = visit2(get_index("shiny gold".to_string()), &lines, &mut subbags) - 1;
    println!("{:?}", counts);
}

fn visit1<'a>(
    color: usize,
    subbags: &HashMap<usize, Vec<SubBags>>,
    cache: &mut HashMap<usize, bool>,
) -> bool {
    if cache.contains_key(&color) {
        return *cache.get(&color).unwrap();
    }

    let r = subbags
        .get(&color)
        .unwrap()
        .iter()
        .any(|subbag| visit1(subbag.color, subbags, cache));

    cache.insert(color, r);
    r
}

fn visit2<'a>(
    color: usize,
    subbags: &HashMap<usize, Vec<SubBags>>,
    cache: &mut HashMap<usize, i32>,
) -> i32 {
    if cache.contains_key(&color) {
        return *cache.get(&color).unwrap();
    }

    let r = 1 + subbags[&color]
        .iter()
        .map(|subbag| subbag.count * visit2(subbag.color, subbags, cache))
        .sum::<i32>();

    cache.insert(color, r);
    r
}
