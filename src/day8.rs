use regex::Regex;
use std::collections::*;
use std::fs::read_to_string;

#[derive(Debug)]
struct Instruction<'a> {
    instruction: &'a str,
    arg1: i32,
}

fn execute(instructions: &Vec<Instruction>) -> (i32, usize) {
    let mut counter = 0;
    let mut ip = 0;
    let mut visited: HashSet<usize> = HashSet::new();
    while ip < instructions.len() {
        if visited.contains(&ip) {
            return (counter, ip);
        }

        visited.insert(ip);
        match instructions[ip].instruction {
            "nop" => { ip += 1; }
            "jmp" => { ip = (ip as i32 + instructions[ip].arg1) as usize; }
            "acc" => { counter += instructions[ip].arg1; ip += 1; }
            _ => { println!("invalid instruction: {:?}", instructions[ip]); panic!(); }
        }
    }

    return (counter, ip);
}


pub fn main() {
    let text = read_to_string("inputs/day8.in").unwrap();
    let re = Regex::new(r"(nop|acc|jmp) ([-+\d]+)").unwrap();

    let mut instructions: Vec<_> = text
        .trim()
        .split("\n")
        .map(|x| {
            let matches = re.captures(&x).unwrap();

            Instruction {
                instruction: &matches.get(1).unwrap().as_str(),
                arg1: i32::from_str_radix(&matches[2], 10).unwrap(),
            }
        })
        .collect();


    println!("{:?}", execute(&instructions).0);

    for i in 0..instructions.len() {
        let result = match instructions[i].instruction {
            "nop" => {
                instructions[i].instruction = "jmp";
                let r = execute(&instructions);
                instructions[i].instruction = "nop";
                Some(r)
            },
            "jmp" => {
                instructions[i].instruction = "nop";
                let r = execute(&instructions);
                instructions[i].instruction = "jmp";
                Some(r)
            },
            _ => None,
        };

        if result.is_some() && result.unwrap().1 == instructions.len() {
            println!("{:?}", result.unwrap().0);
        }
    }
}
