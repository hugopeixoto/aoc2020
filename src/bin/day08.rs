#![feature(or_patterns)]
#![feature(test)]
extern crate test;

#[macro_use]
extern crate scan_fmt;

use std::collections::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    Nop,
    Jmp,
    Acc,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    op: Op,
    arg1: i32,
}

#[derive(Debug, Default)]
struct State {
    ip: usize,
    counter: i32,
}

impl State {
    fn eval(self: &mut State, inst: &Instruction) {
        self.ip = inst.next(self.ip);

        if inst.op == Op::Acc {
            self.counter += inst.arg1;
        }
    }
}

impl Instruction {
    fn next(self: &Instruction, index: usize) -> usize {
        match self.op {
            Op::Jmp => (index as i32 + self.arg1) as usize,
            Op::Acc => index + 1,
            Op::Nop => index + 1,
        }
    }

    fn flip(self: &Instruction) -> Instruction {
        match self.op {
            Op::Jmp => Instruction {
                op: Op::Nop,
                arg1: self.arg1,
            },
            Op::Nop => Instruction {
                op: Op::Jmp,
                arg1: self.arg1,
            },
            Op::Acc => Instruction {
                op: Op::Acc,
                arg1: self.arg1,
            },
        }
    }
}

fn execute(instructions: &Vec<Instruction>) -> State {
    let mut state = State::default();
    let mut visited: HashSet<usize> = HashSet::new();
    while state.ip < instructions.len() {
        let inst = &instructions[state.ip];
        if visited.contains(&state.ip) {
            return state;
        }

        visited.insert(state.ip);
        state.eval(inst);
    }

    return state;
}

fn dfs(index: usize, edges: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    if visited[index] {
        return;
    }
    visited[index] = true;

    for edge in edges[index].iter() {
        dfs(*edge, edges, visited);
    }
}

pub fn day08(input: String) -> (usize, usize) {
    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let (opstr, arg1) = scan_fmt!(&line, "{} {}", String, i32).unwrap();

            let op = match opstr.as_str() {
                "jmp" => Op::Jmp,
                "acc" => Op::Acc,
                "nop" => Op::Nop,
                _ => panic!(),
            };

            Instruction {
                op,
                arg1,
            }
        })
        .collect();

    let p1 = execute(&instructions).counter;

    let mut reverse: Vec<Vec<usize>> = Vec::new();
    reverse.resize_with(instructions.len() + 1, Default::default);

    for (index, inst) in instructions.iter().enumerate() {
        reverse[inst.next(index)].push(index);
    }

    let mut visited = vec![];
    visited.resize(1 + instructions.len(), false);
    dfs(instructions.len(), &reverse, &mut visited);

    let mut state = State::default();
    let mut flipped = false;
    while state.ip < instructions.len() {
        let mut inst = instructions[state.ip];

        if !flipped && visited[inst.flip().next(state.ip)] {
            inst = inst.flip();
            flipped = true;
        }

        state.eval(&inst);
    }

    let p2 = state.counter;

    (p1 as usize, p2 as usize)
}

aoc2020::day!(day08, "day08.in", bench_day08);
