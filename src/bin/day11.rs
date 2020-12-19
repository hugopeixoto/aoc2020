#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(test)]
extern crate test;

const DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn next(a: &Vec<bool>, b: &mut Vec<bool>, neighbors: &Vec<usize>, thresh: usize) -> bool {
    let mut changed = false;
    for i in 1..a.len() {
        let occupied = neighbors[(i - 1) * 8..i * 8]
            .iter()
            .fold(0, |acc, j| acc + a[*j] as usize);

        let flip = if a[i] {
            occupied >= thresh
        } else {
            occupied == 0
        };

        b[i] = a[i] ^ flip;
        changed |= flip;
    }

    changed
}

fn neighbors(state: &Vec<char>, width: usize, _height: usize, indexes: &Vec<usize>) -> Vec<usize> {
    let mut ns: Vec<usize> = vec![];
    ns.resize(state.len() * 8, 0);

    let w1 = width as i32 + 1;
    let mut ni = 0;
    for i in 0..state.len() {
        if state[i] != 'L' {
            continue;
        }

        for delta in DELTAS.iter() {
            let j = (i as i32 + delta.0 + w1 * delta.1) as usize;
            if state[j] == 'L' {
                ns[ni] = indexes[j];
            }
            ni += 1;
        }
    }

    ns
}

fn neighbors2(state: &Vec<char>, width: usize, height: usize, indexes: &Vec<usize>) -> Vec<usize> {
    let mut ns: Vec<usize> = vec![];
    ns.resize(state.len() * 8, 0);

    let w1 = width as i32 + 1;
    let mut ni = 0;
    for i in 0..state.len() {
        if state[i] != 'L' {
            continue;
        }
        let ix = (i as i32 - 1) % w1;
        let iy = (i as i32 - 1) / w1 - 1;

        for delta in DELTAS.iter() {
            for m in 1.. {
                let px = ix as i32 + m as i32 * delta.0;
                let py = iy as i32 + m as i32 * delta.1;
                if (0..width as i32).contains(&px) && (0..height as i32).contains(&py) {
                    let j = (1 + px + py * w1 + w1) as usize;
                    if state[j] == 'L' {
                        ns[ni] = indexes[j];
                        break;
                    }
                } else {
                    break;
                }
            }
            ni += 1;
        }
    }

    ns
}

pub fn day11(input: String) -> (usize, usize) {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.len() / (width + 1);

    let mut expanded_state: Vec<char> = vec![];
    expanded_state.resize((width + 1) * (height + 2) + 1, '.');

    for (i, c) in input.chars().enumerate() {
        expanded_state[i + width + 1 + 1] = c;
    }

    let mut indexes = vec![];
    let mut i = 0;
    for &c in expanded_state.iter() {
        indexes.push(i + 1);
        if c == 'L' {
            i += 1;
        }
    }

    let mut compact_state = vec![];
    compact_state.resize(i + 1, false);

    let ns = neighbors(&expanded_state, width, height, &indexes);
    let ns2 = neighbors2(&expanded_state, width, height, &indexes);

    let mut a = &mut compact_state.clone();
    let mut b = &mut compact_state.clone();

    while next(a, b, &ns, 4) {
        (a, b) = (b, a);
    }

    let p1 = a.iter().filter(|&&c| c).count();

    let mut a = &mut compact_state.clone();
    let mut b = &mut compact_state.clone();

    while next(a, b, &ns2, 5) {
        (a, b) = (b, a);
    }

    let p2 = a.iter().filter(|&&c| c).count();

    (p1, p2)
}

aoc2020::day!(day11, "day11.in", bench_day11);
