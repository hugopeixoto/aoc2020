#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

use std::collections::*;

const DELTAS: [(i32, i32); 6] = [
      (-1, -1), (0, -1),
    (-1,  0),     (1,  0),
       (0,  1), (1,  1),
];

fn next(a: &mut HashSet<(i32, i32)>, b: &mut HashSet<(i32, i32)>) {
    let xmin = a.iter().map(|&(x,_)| x).min().unwrap();
    let ymin = a.iter().map(|&(_,y)| y).min().unwrap();
    let xmax = a.iter().map(|&(x,_)| x).max().unwrap();
    let ymax = a.iter().map(|&(_,y)| y).max().unwrap();

    for i in xmin-2..=xmax+2 {
        for j in ymin-2..=ymax+2 {
            let p = (i, j);
            let is_black = a.contains(&p);
            let black_neighbors = DELTAS.iter().filter(|d| a.contains(&(i+d.0, j+d.1))).count();

            match (is_black, black_neighbors) {
                (true, 0) | (true, 3..) => { b.remove(&p); },
                (true, _) => { b.insert(p); }
                (false, 2) => { b.insert(p); }
                (false, _) => { b.remove(&p); }
            }
        }
    }
}

pub fn day24(input: String) -> (usize, usize) {
    let mut state = 0;
    let mut position = (0i32, 0i32);

    let mut colors: HashSet<(i32, i32)> = HashSet::new();
    for c in input.chars() {
        match (c, state) {
            ('\n', _) => {
                let x = colors.get(&position);
                match x {
                    Some(_) => { colors.remove(&position); },
                    None => { colors.insert(position); }
                }

                state = 0;
                position = (0, 0);
            },
            ('e', 0) => { // east
                position.0 += 1;
            },
            ('w', 0) => { // west
                position.0 -= 1;
            },
            ('n', 0) => {
                state = -1;
            },
            ('s', 0) => {
                state = 1;
            },
            ('e', -1) => { // north east
                position.1 -= 1;
                state = 0;
            },
            ('e', 1) => { // south east
                position.0 += 1;
                position.1 += 1;
                state = 0;
            },
            ('w', -1) => { // north west
                position.0 -= 1;
                position.1 -= 1;
                state = 0;
            },
            ('w', 1) => { // south west
                position.1 += 1;
                state = 0;
            },
            _ => { panic!("unknown state {}, {}", c, state); }
        }
    }

    let p1 = colors.len();

    let mut a = &mut colors.clone();
    let mut b = &mut colors.clone();

    for i in 0..100 {
        next(a, b);
        (a,b) = (b, a);
    }

    (p1, a.len())
}

aoc2020::day!(day24, "day24.in", bench_day24);
