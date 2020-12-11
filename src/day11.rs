use std::fs::read_to_string;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Layout {
    state: Vec<char>,
    width: i32,
    height: i32,
}

fn print(layout: &Layout) {
    for i in 0..layout.height {
        let offset: usize = (i * layout.width) as usize;
        let end = offset + layout.width as usize;
        let line = layout.state[offset..end].iter().collect::<String>();
        println!("{}", line);
    }
}

fn adjacent_seats2(i: usize, layout: &Layout) -> (i32, i32) {
    let deltas: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1,  0),
        (-1,  1),
        ( 0, -1),
        ( 0,  1),
        ( 1, -1),
        ( 1,  0),
        ( 1,  1),
    ];

    let mut empty = 0;
    let mut occupied = 0;

    let ix = i as i32 % layout.width;
    let iy = i as i32 / layout.width;
    for delta in deltas {
        let mut px = ix;
        let mut py = iy;
        loop {
            px = px + delta.0;
            py = py + delta.1;
            if (0..layout.width).contains(&px) && (0..layout.height).contains(&py) {
                let j = px + py*layout.width;

                //println!("{} ({} x {}) -> {} ({} x {})", i, ix, iy, j, px, py);

                if layout.state[j as usize] == '#' {
                    occupied += 1;
                    break;
                }
                if layout.state[j as usize] == 'L' {
                    empty += 1;
                    break;
                }
            } else {
                break;
            }
        }
    }

    (empty, occupied)
}

fn adjacent_seats(i: usize, layout: &Layout) -> (i32, i32) {
    let deltas: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1,  0),
        (-1,  1),
        ( 0, -1),
        ( 0,  1),
        ( 1, -1),
        ( 1,  0),
        ( 1,  1),
    ];

    let mut empty = 0;
    let mut occupied = 0;

    let ix = i as i32 % layout.width;
    let iy = i as i32 / layout.width;
    for delta in deltas {
        let px = ix + delta.0;
        let py = iy + delta.1;
        if (0..layout.width).contains(&px) && (0..layout.height).contains(&py) {
            let j = px + py*layout.width;

            //println!("{} ({} x {}) -> {} ({} x {})", i, ix, iy, j, px, py);

            match layout.state[j as usize] {
                'L' => { empty += 1; },
                '#' => { occupied += 1; },
                _ => {},
            }
        }
    }

    (empty, occupied)
}

fn next(previous: &Layout) -> Layout {
    let mut layout = previous.clone();

    for i in 0..previous.state.len() {
        let (_, occupied) = adjacent_seats(i, &previous);
        if previous.state[i] == 'L' && occupied == 0 {
            layout.state[i] = '#';
        } else if previous.state[i] == '#' && occupied >= 4 {
            layout.state[i] = 'L';
        }
    }

    layout
}

fn next2(previous: &Layout) -> Layout {
    let mut layout = previous.clone();

    for i in 0..previous.state.len() {
        let (_, occupied) = adjacent_seats2(i, &previous);
        if previous.state[i] == 'L' && occupied == 0 {
            layout.state[i] = '#';
        } else if previous.state[i] == '#' && occupied >= 5 {
            layout.state[i] = 'L';
        }
    }

    layout
}

pub fn main() {
    let text = read_to_string("inputs/day11.in").unwrap();

    let width = text.chars().position(|c| c == '\n').unwrap();
    let height = text.len() / (width + 1);
    let state: Vec<_> = text.chars().filter(|&c| c != '\n').collect();

    let mut layout = Layout { state, width: width as i32, height: height as i32 };

    loop {
        //println!("iterating");
        let next = next2(&layout);
        //print(&next);
        //println!("---");

        if next == layout { break; }
        layout = next;
    }

    // print(&layout);

    println!("{}", layout.state.iter().filter(|&&c| c == '#').count());
}
