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
    (passport
        .split(|c| c == ' ' || c == '\n')
        .map(|field| {
            if validation.ecl.is_match(field) {
                1 << 0
            } else if validation.pid.is_match(field) {
                1 << 1
            } else if validation.eyr.is_match(field) {
                1 << 2
            } else if validation.hcl.is_match(field) {
                1 << 3
            } else if validation.byr.is_match(field) {
                1 << 4
            } else if validation.iyr.is_match(field) {
                1 << 5
            } else if field.starts_with("cid:") {
                1 << 6
            } else if validation.hgt.is_match(field) {
                1 << 7
            } else {
                0
            }
        })
        .sum::<u64>()
        & 0b10111111)
        == 0b10111111
}

pub fn main() {
    let text = read_to_string("inputs/day4.in").unwrap();
    let passports = text.split("\n\n").collect::<Vec<_>>();

    let validation = Validation {
        hcl: Regex::new(r"^hcl:").unwrap(),
        hgt: Regex::new(r"^hgt:").unwrap(),
        pid: Regex::new(r"^pid:").unwrap(),
        ecl: Regex::new(r"^ecl:").unwrap(),
        eyr: Regex::new(r"^eyr:").unwrap(),
        iyr: Regex::new(r"^iyr:").unwrap(),
        byr: Regex::new(r"^byr:").unwrap(),
    };

    println!(
        "{}",
        passports
            .iter()
            .filter(|passport| is_valid(passport, &validation))
            .count()
    );

    let validation = Validation {
        hcl: Regex::new(r"^hcl:#[0-9a-f]{6}$").unwrap(),
        hgt: Regex::new(r"^hgt:(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)$").unwrap(),
        pid: Regex::new(r"^pid:\d{9}$").unwrap(),
        ecl: Regex::new(r"^ecl:(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
        eyr: Regex::new(r"^eyr:20(2\d|30)$").unwrap(),
        iyr: Regex::new(r"^iyr:20(1\d|20)$").unwrap(),
        byr: Regex::new(r"^byr:(19[2-9]\d|200[0-2])$").unwrap(),
    };

    println!(
        "{}",
        passports
            .iter()
            .filter(|passport| is_valid(passport, &validation))
            .count()
    );
}
