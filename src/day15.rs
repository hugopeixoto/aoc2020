use std::fs::read_to_string;

pub fn main() {
    let text = read_to_string("inputs/day15.in").unwrap();

    let mut indexes = vec![];
    indexes.resize(30000000, -1);

    let mut count = 0;
    for (i, c) in text.trim().split(',').enumerate() {
        let n = c.parse::<usize>().unwrap();
        indexes[n] = i as i32;
        count += 1;
    }

    let mut next = 0;

    while count < 2020 - 1 {
        let n = next;

        next = if indexes[n] < 0 { 0 } else { count - indexes[n] as usize };
        indexes[n] = count as i32;
        count += 1;
    }

    println!("{}", next);

    while count < 30000000 - 1 {
        let n = next;

        next = if indexes[n] < 0 { 0 } else { count - indexes[n] as usize };
        indexes[n] = count as i32;
        count += 1;
    }

    println!("{}", next);
}
