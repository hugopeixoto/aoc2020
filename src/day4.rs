use regex::Regex;
use std::fs::read_to_string;

struct Validation {
    hcl: regex::Regex,
    hgt: regex::Regex,
    pid: regex::Regex,
    ecl: regex::Regex,
    eyr: regex::Regex,
    iyr: regex::Regex,
    byr: regex::Regex,
}

fn is_valid(passport: &str, validation: &Validation) -> bool {
    validation.hcl.is_match(passport)
        && validation.hgt.is_match(passport)
        && validation.pid.is_match(passport)
        && validation.ecl.is_match(passport)
        && validation.eyr.is_match(passport)
        && validation.iyr.is_match(passport)
        && validation.byr.is_match(passport)
}

pub fn main() {
    let text = read_to_string("inputs/day4.in").unwrap();

    let p1 = Validation {
        hcl: Regex::new(r"(?m)\bhcl:").unwrap(),
        hgt: Regex::new(r"(?m)\bhgt:").unwrap(),
        pid: Regex::new(r"(?m)\bpid:").unwrap(),
        ecl: Regex::new(r"(?m)\becl:").unwrap(),
        eyr: Regex::new(r"(?m)\beyr:").unwrap(),
        iyr: Regex::new(r"(?m)\biyr:").unwrap(),
        byr: Regex::new(r"(?m)\bbyr:").unwrap(),
    };

    let p2 = Validation {
        hcl: Regex::new(r"(?m)\bhcl:#[0-9a-f]{6}\b").unwrap(),
        hgt: Regex::new(r"(?m)\bhgt:(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)\b").unwrap(),
        pid: Regex::new(r"(?m)\bpid:\d{9}\b").unwrap(),
        ecl: Regex::new(r"(?m)\becl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap(),
        eyr: Regex::new(r"(?m)\beyr:20(2\d|30)\b").unwrap(),
        iyr: Regex::new(r"(?m)\biyr:20(1\d|20)\b").unwrap(),
        byr: Regex::new(r"(?m)\bbyr:(19[2-9]\d|200[0-2])\b").unwrap(),
    };

    println!(
        "{}",
        text.split("\n\n")
            .filter(|passport| is_valid(passport, &p1))
            .count()
    );

    println!(
        "{}",
        text.split("\n\n")
            .filter(|passport| is_valid(passport, &p2))
            .count()
    );
}
