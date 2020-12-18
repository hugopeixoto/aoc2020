use std::fs::read_to_string;

pub fn main() {
    let text = read_to_string("inputs/day13.in").unwrap();

    let mut lines = text.trim().split('\n');

    let start_time = lines.next().unwrap().parse::<u64>().unwrap();

    let ids = lines.next().unwrap().split(',').collect::<Vec<_>>();

    let mut minimum = start_time;
    let mut minimal = 0;

    let mut delta = 0;
    let mut base = 0;
    let mut multiplier = 1;

    for i in ids.iter() {
        if *i != "x" {
            let curr = i.parse::<u64>().unwrap();
            for k in 0..curr {
                if (base + multiplier * k + delta) % curr == 0 {
                    base += multiplier * k;
                    multiplier *= curr;
                    break;
                }
            }

            let v = (curr - (start_time % curr)) % curr;
            if v < minimum {
                minimum = v;
                minimal = curr;
            }
        }
        delta += 1;
    }

    println!("{}", minimum * minimal);
    println!("{}", base);
}
