use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::collections::*;

#[derive(Debug)]
struct SubBags {
    count: i32,
    color: String,
}

pub fn main() {
    let file = File::open("inputs/day7.in").unwrap();
    let re = Regex::new(r"(.*) bag(s?) contain(s?) (.*)").unwrap();
    let re2 = Regex::new(r"(\d) (.*) bag(s?)").unwrap();

    let lines: HashMap<_, _> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
             let matches = re.captures(&x).unwrap();
             let parts = matches.get(4).unwrap().as_str().split(", ");

             let herp = parts.filter_map(|p| re2.captures(p)).
                 map(|subbag| SubBags {
                     count: i32::from_str_radix(subbag.get(1).unwrap().as_str(), 10).unwrap(),
                     color: subbag.get(2).unwrap().as_str().to_string(),
                 }).
                 collect::<Vec<_>>();

             (
                 matches.get(1).unwrap().as_str().to_string(),
                 herp,
             )
        })
        .collect();

    let mut has_shiny_gold : HashSet<String> = HashSet::new();
    has_shiny_gold.insert("shiny gold".to_string());

    loop {
        let mut insertions = false;
        for (color, contents) in lines.iter() {
            if !has_shiny_gold.contains(color) {
                if contents.iter().filter(|subbag| has_shiny_gold.contains(&subbag.color)).count() > 0 {
                    has_shiny_gold.insert(color.to_string());
                    insertions = true;
                }
            }
        }

        if !insertions {
            break;
        }
    }

    println!("{:?}", has_shiny_gold.len() - 1);

    let mut subbags: HashMap<String, i32> = HashMap::new();

    let counts = visit(&"shiny gold".to_string(), &lines, &mut subbags) - 1;

    println!("{:?}", counts);
}

fn visit(color: &String, subbags: &HashMap<String, Vec<SubBags>>, counts: &mut HashMap<String, i32>) -> i32 {
    if counts.contains_key(color) {
        return *counts.get(color).unwrap();
    }

    let mut count = 1;
    for subbag in subbags.get(color).unwrap().iter() {
        println!("{} contains {} * {}", color, subbag.count, subbag.color);
        count += subbag.count * visit(&subbag.color, subbags, counts);
    }

    println!("{} ontains {}", color, count);

    counts.insert(color.to_string(), count);
    count
}
