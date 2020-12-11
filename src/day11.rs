use std::fs::read_to_string;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Layout {
    state: Vec<char>,
    width: i32,
    height: i32,
}

const DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1,  0),
    (-1,  1),
    ( 0, -1),
    ( 0,  1),
    ( 1, -1),
    ( 1,  0),
    ( 1,  1),
];

fn next(a: &Layout, b: &mut Layout, neighbors: &Vec<usize>, thresh: usize) -> bool {
    let mut changed = false;
    for i in 0..a.state.len() - 1 {
        let occupied = neighbors[i*8..(i+1)*8]
            .iter()
            .filter(|&j| a.state[*j] == '#')
            .count();

        if a.state[i] == 'L' && occupied == 0 {
            b.state[i] = '#';
            changed = true;
        } else if a.state[i] == '#' && occupied >= thresh {
            b.state[i] = 'L';
            changed = true;
        } else {
            b.state[i] = a.state[i];
        }
    }

    changed
}

fn neighbors(layout: &Layout, thresh: usize) -> Vec<usize> {
    let len = layout.state.len() - 1;
    let mut ns: Vec<usize> = (0..len*8).collect();

    for i in 0..len {
        let ix = i as i32 % layout.width;
        let iy = i as i32 / layout.width;

        for (di, delta) in DELTAS.iter().enumerate() {
            ns[i * 8 + di] = len;
            for m in 1i32..=(thresh as i32) {
                let px = ix + m * delta.0;
                let py = iy + m * delta.1;
                if (0..layout.width).contains(&px) && (0..layout.height).contains(&py) {
                    let j = (px + py * layout.width) as usize;

                    if layout.state[j] != '.' {
                        ns[i * 8 + di] = j;
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    ns
}

pub fn main() {
    let text = read_to_string("inputs/day11.in").unwrap();

    let width = text.chars().position(|c| c == '\n').unwrap();
    let height = text.len() / (width + 1);

    let mut state: Vec<_> = text.chars().filter(|&c| c != '\n').collect();
    state.push('.');

    let layout = Layout {
        state,
        width: width as i32,
        height: height as i32,
    };

    let ns = neighbors(&layout, 1);
    let mut a = &mut layout.clone();
    let mut b = &mut layout.clone();

    loop {
        if !next(a, b, &ns, 4) { break; }
        let c = b;
        b = a;
        a = c;
    }

    println!("{}", a.state.iter().filter(|&&c| c == '#').count());

    let ns = neighbors(&layout, layout.state.len());
    let mut a = &mut layout.clone();
    let mut b = &mut layout.clone();

    loop {
        if !next(a, b, &ns, 5) { break; }

        let c = b;
        b = a;
        a = c;
    }

    println!("{}", a.state.iter().filter(|&&c| c == '#').count());
}
