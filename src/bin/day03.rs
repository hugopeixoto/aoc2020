#![feature(test)]
extern crate test;

fn hits(area: &Vec<char>, height: usize, width: usize, delta: (usize, usize)) -> u32 {
    let mut position = (0, 0);
    let mut trees = 0;

    while position.1 < height {
        if area[position.1 * (width + 1) + position.0] == '#' {
            trees += 1;
        }

        position.0 = (position.0 + delta.0) % width;
        position.1 += delta.1;
    }

    trees
}

pub fn day03(input: String) -> (u32, u32) {
    let area: Vec<_> = input.chars().collect();

    let width = area.iter().position(|c| *c == '\n').unwrap();
    let height = area.len() / (width + 1);

    let p1 = hits(&area, height, width, (3, 1));

    let p2 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .into_iter()
            .map(|delta| hits(&area, height, width, delta))
            .product::<u32>();

    (p1, p2)
}

aoc2020::day!(day03, "day03.in", bench_day03);
