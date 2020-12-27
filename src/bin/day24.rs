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

struct Bounds {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

fn next(a: &mut Vec<bool>, b: &mut Vec<bool>, boundsn: &Bounds, boundsi: &Bounds) {
    let width = boundsn.xmax - boundsn.xmin;

    for x in boundsi.xmin..=boundsi.xmax {
        for y in boundsi.ymin..=boundsi.ymax {
            let idx = ((y - boundsn.ymin) * width + (x - boundsn.xmin)) as usize;
            let is_black = a[idx];
            let black_neighbors = DELTAS.iter().filter(|d| {
                a[(idx as i32 + d.1 * width + d.0) as usize]
            }).count();

            b[idx] = match (is_black, black_neighbors) {
                (true, 0 | 3..) => { false },
                (true, _) => { true }
                (false, 2) => { true }
                (false, _) => { false }
            };
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

    let bounds0 = Bounds {
        xmin: *colors.iter().map(|(x,_)| x).min().unwrap(),
        ymin: *colors.iter().map(|(_,y)| y).min().unwrap(),
        xmax: *colors.iter().map(|(x,_)| x).max().unwrap(),
        ymax: *colors.iter().map(|(_,y)| y).max().unwrap(),
    };

    let boundsn = Bounds {
        xmin: bounds0.xmin - 104,
        ymin: bounds0.ymin - 104,
        xmax: bounds0.xmax + 104,
        ymax: bounds0.ymax + 104,
    };

    let width = boundsn.xmax - boundsn.xmin;
    let height = boundsn.ymax - boundsn.ymin;

    let n = (width * height) as usize;

    let mut floor = vec![];
    floor.resize(n, false);
    for &(x, y) in colors.iter() {
        floor[((y - boundsn.ymin) * width + (x - boundsn.xmin)) as usize] = true;
    }

    let mut a = &mut floor.clone();
    let mut b = &mut floor.clone();

    for i in 0..100 {
        next(a, b, &boundsn, &Bounds {
            xmin: bounds0.xmin - i - 2,
            ymin: bounds0.ymin - i - 2,
            xmax: bounds0.xmax + i + 2,
            ymax: bounds0.ymax + i + 2,
        });

        (a,b) = (b, a);
    }

    let p1 = colors.len();
    let p2 = a.iter().filter(|&&x| x).count();

    (p1, p2)
}

aoc2020::day!(day24, "day24.in", bench_day24);
