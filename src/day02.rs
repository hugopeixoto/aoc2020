use std::fs::read_to_string;

pub fn main() {
    let input = read_to_string("inputs/day2.in").unwrap();

    let c = input
        .trim()
        .split("\n")
        .map(|x| {
            let (low, high, character, password) =
                scan_fmt!(&x, "{}-{} {}: {}", i32, i32, char, String).unwrap();
            (
                (low..=high)
                    .contains(&(password.chars().filter(|x| x == &character).count() as i32)),
                (password.chars().nth((low - 1) as usize).unwrap() == character)
                    != (password.chars().nth((high - 1) as usize).unwrap() == character),
            )
        })
        .fold((0, 0), |a, b| {
            (a.0 + if b.0 { 1 } else { 0 }, a.1 + if b.1 { 1 } else { 0 })
        });

    println!("{:?}", c.0);
    println!("{:?}", c.1);
}
