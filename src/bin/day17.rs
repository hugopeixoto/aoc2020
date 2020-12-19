#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(test)]
extern crate test;

pub fn index4(x: usize, y: usize, z: i32, w: i32, width: usize, height: usize, depth: usize, _wefth: usize) -> usize {
    x + y * width + z.abs() as usize * width * height + w.abs() as usize * width * height * depth
}

pub fn in_range4(x: i32, y: i32, z: i32, w: i32, width: usize, height: usize, depth: usize, wefth: usize) -> bool {
    (0..width as i32).contains(&x) &&
    (0..height as i32).contains(&y) &&
    (0..depth as i32).contains(&z.abs()) &&
    (0..wefth as i32).contains(&w.abs())
}

const DELTAS4: [(i32, i32, i32, i32); 80] = [
    (-1, -1, -1, -1), ( 0, -1, -1, -1), ( 1, -1, -1, -1),
    (-1,  0, -1, -1), ( 0,  0, -1, -1), ( 1,  0, -1, -1),
    (-1,  1, -1, -1), ( 0,  1, -1, -1), ( 1,  1, -1, -1),

    (-1, -1,  0, -1), ( 0, -1,  0, -1), ( 1, -1,  0, -1),
    (-1,  0,  0, -1), ( 0,  0,  0, -1), ( 1,  0,  0, -1),
    (-1,  1,  0, -1), ( 0,  1,  0, -1), ( 1,  1,  0, -1),

    (-1, -1,  1, -1), ( 0, -1,  1, -1), ( 1, -1,  1, -1),
    (-1,  0,  1, -1), ( 0,  0,  1, -1), ( 1,  0,  1, -1),
    (-1,  1,  1, -1), ( 0,  1,  1, -1), ( 1,  1,  1, -1),

    (-1, -1, -1,  0), ( 0, -1, -1,  0), ( 1, -1, -1,  0),
    (-1,  0, -1,  0), ( 0,  0, -1,  0), ( 1,  0, -1,  0),
    (-1,  1, -1,  0), ( 0,  1, -1,  0), ( 1,  1, -1,  0),

    (-1, -1,  0,  0), ( 0, -1,  0,  0), ( 1, -1,  0,  0),
    (-1,  0,  0,  0),                   ( 1,  0,  0,  0),
    (-1,  1,  0,  0), ( 0,  1,  0,  0), ( 1,  1,  0,  0),

    (-1, -1,  1,  0), ( 0, -1,  1,  0), ( 1, -1,  1,  0),
    (-1,  0,  1,  0), ( 0,  0,  1,  0), ( 1,  0,  1,  0),
    (-1,  1,  1,  0), ( 0,  1,  1,  0), ( 1,  1,  1,  0),

    (-1, -1, -1,  1), ( 0, -1, -1,  1), ( 1, -1, -1,  1),
    (-1,  0, -1,  1), ( 0,  0, -1,  1), ( 1,  0, -1,  1),
    (-1,  1, -1,  1), ( 0,  1, -1,  1), ( 1,  1, -1,  1),

    (-1, -1,  0,  1), ( 0, -1,  0,  1), ( 1, -1,  0,  1),
    (-1,  0,  0,  1), ( 0,  0,  0,  1), ( 1,  0,  0,  1),
    (-1,  1,  0,  1), ( 0,  1,  0,  1), ( 1,  1,  0,  1),

    (-1, -1,  1,  1), ( 0, -1,  1,  1), ( 1, -1,  1,  1),
    (-1,  0,  1,  1), ( 0,  0,  1,  1), ( 1,  0,  1,  1),
    (-1,  1,  1,  1), ( 0,  1,  1,  1), ( 1,  1,  1,  1),
];

pub fn next4(a: &mut Vec<bool>, b: &mut Vec<bool>, width: usize, height: usize, depth: usize, wefth: usize, t: usize) {
    let deltas = if wefth == 1 {
        &DELTAS4[27..54]
    } else {
        &DELTAS4[..]
    };

    for i in t..=(width - t) {
        for j in t..=(height - t) {
            for k in 0..=(depth - t) as i32 {
                for l in 0..=(wefth - t.min(wefth)) as i32 {
                    let neighbors = deltas
                        .iter()
                        .map(|&(dx,dy,dz,dw)| (i as i32 + dx, j as i32 + dy, k as i32 + dz, l as i32 + dw))
                        .filter(|&(x,y,z,w)| in_range4(x, y, z, w, width, height, depth, wefth))
                        .filter(|&(x,y,z,w)| a[index4(x as usize,y as usize, z, w, width,height,depth,wefth)])
                        .count();

                    let idx = index4(i,j,k,l,width,height,depth,wefth);
                    b[idx] = match (a[idx], neighbors) {
                        (true, 2..=3) => true,
                        (false, 3) => true,
                        _ => false,
                    };
                }
            }
        }
    }
}

pub fn count4(a: &mut Vec<bool>, width: usize, height: usize, depth: usize, wefth: usize) -> usize {
    let mut t = 0;
    for i in 0..width {
        for j in 0..height {
            for k in (1 - depth as i32)..depth as i32 {
                for l in (1 - wefth as i32)..wefth as i32 {
                    if a[index4(i,j,k,l,width,height,depth,wefth)] {
                        t += 1;
                    }
                }
            }
        }
    }

    t
}

pub fn day17(input: String) -> (usize, usize) {
    let z0width = input.lines().next().unwrap().len();
    let z0height = input.len() / (z0width + 1);

    let turns = 6;
    let width = z0width + 2*turns;
    let height = z0height + 2*turns;
    let depth = 1 + turns;
    let wefth = 1;

    let mut state: Vec<bool> = vec![];
    state.resize(width * height * depth * wefth, false);

    for (i, c) in input.chars().enumerate() {
        if c != '\n' {
            let x = i % (z0width + 1);
            let y = i / (z0width + 1);
            state[index4(x+turns, y+turns, 0, 0, width, height, depth, wefth)] = c == '#';
        }
    }

    let mut a = &mut state.clone();
    let mut b = &mut state.clone();

    for t in 0..turns {
        next4(a, b, width, height, depth, wefth, turns - t);
        (a, b) = (b, a);
    }

    let p1 = count4(a, width, height, depth, wefth);

    let wefth = 1 + turns;

    let mut state: Vec<bool> = vec![];
    state.resize(width * height * depth * wefth, false);

    for (i, c) in input.chars().enumerate() {
        if c != '\n' {
            let x = i % (z0width + 1);
            let y = i / (z0width + 1);
            state[index4(x+turns, y+turns, 0, 0, width, height, depth, wefth)] = c == '#';
        }
    }

    let mut a = &mut state.clone();
    let mut b = &mut state.clone();

    for t in 0..turns {
        next4(a, b, width, height, depth, wefth, turns - t);
        (a, b) = (b, a);
    }

    let p2 = count4(a, width, height, depth, wefth);

    (p1, p2)
}

aoc2020::day!(day17, "day17.in", bench_day17);
