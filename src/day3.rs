use std::fs::read_to_string;

fn hits(area: &str, height: usize, width: usize, delta: (usize, usize)) -> u32 {
    let mut position = (0, 0);
    let mut trees = 0;

    while position.1 < height {
        if area
            .chars()
            .nth(position.1 * (width + 1) + position.0)
            .unwrap()
            == '#'
        {
            trees += 1;
        }

        position.0 = (position.0 + delta.0) % width;
        position.1 += delta.1;
    }

    trees
}

pub fn main() {
    let area = read_to_string("inputs/day3.in").unwrap();

    let width = area.chars().position(|c| c == '\n').unwrap();
    let height = area.len() / (width + 1);

    println!("{}", hits(&area, height, width, (3, 1)));

    println!(
        "{}",
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .into_iter()
            .map(|delta| hits(&area, height, width, delta))
            .product::<u32>(),
    );
}
