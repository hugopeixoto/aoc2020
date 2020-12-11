use std::fs::read_to_string;

pub fn main() {
    let text = read_to_string("inputs/day6.in").unwrap();
    let mut p1 = 0;
    let mut p2 = 0;
    for entry in text.trim().split("\n\n") {
        let people = entry.split("\n").count();

        for c in 'a'..='z' {
            let answers = entry.chars().filter(|x| *x == c).count();

            if answers > 0 {
                p1 += 1;
            }
            if answers == people {
                p2 += 1;
            }
        }
    }
    println!("{}", p1);
    println!("{}", p2);
}
