#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(test)]
extern crate test;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Ops {
    Add,
    Mul,
    LParen,
    RParen,
    Value(u64),
}

fn same_precedence(op: &Ops) -> usize {
    match op {
        Ops::Add => 1,
        Ops::Mul => 1,
        _ => 0,
    }
}

fn add_higher_precedence(op: &Ops) -> usize {
    match op {
        Ops::Add => 2,
        Ops::Mul => 1,
        _ => 0,
    }
}

fn infix2postfix<F>(infix: &Vec<Ops>, precedence: F) -> Vec<Ops> where F: Fn(&Ops) -> usize {
    let mut stack = vec![];
    let mut ops = vec![];

    for &op in infix.iter() {
        match op {
            Ops::LParen => { ops.push(op); },
            Ops::Value(_) => { stack.push(op); },
            Ops::RParen => {
                while ops[ops.len() - 1] != Ops::LParen {
                    stack.push(ops.pop().unwrap());
                }
                ops.pop();
            },
            Ops::Add | Ops::Mul => {
                while !ops.is_empty() && precedence(&op) <= precedence(&ops[ops.len() - 1]) {
                    stack.push(ops.pop().unwrap());
                }
                ops.push(op);
            },
        }
    }

    while !ops.is_empty() {
        stack.push(ops.pop().unwrap());
    }

    stack
}

fn eval(postfix: &Vec<Ops>) -> u64 {
    let mut stack = vec![];
    for elem in postfix.iter() {
        match elem {
            Ops::Value(x) => { stack.push(*x); },
            Ops::Mul => { let op1 = stack.pop().unwrap(); let op2 = stack.pop().unwrap(); stack.push(op1 * op2); }
            Ops::Add => { let op1 = stack.pop().unwrap(); let op2 = stack.pop().unwrap(); stack.push(op1 + op2); }
            _ => {},
        }
    }

    stack[0]
}

pub fn day18(input: String) -> (u64, u64) {
    let expr = format!("({})", input.trim().replace("\n", ") + ("));

    let mut infix = Vec::with_capacity(expr.len());
    for c in expr.chars() {
        match c {
            '(' => { infix.push(Ops::LParen); },
            ')' => { infix.push(Ops::RParen); },
            '+' => { infix.push(Ops::Add); },
            '*' => { infix.push(Ops::Mul); },
            '0'..='9' => { infix.push(Ops::Value(c as u64 - '0' as u64)); },
            _ => {},
        }
    }

    let p1 = eval(&infix2postfix(&infix, same_precedence));
    let p2 = eval(&infix2postfix(&infix, add_higher_precedence));

    (p1, p2)
}

aoc2020::day!(day18, "day18.in", bench_day18);
