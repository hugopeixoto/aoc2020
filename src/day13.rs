use std::fs::read_to_string;

pub fn main() {
    let text = read_to_string("inputs/day13.in").unwrap();

    let mut lines = text.trim().split("\n");

    let k = lines.next().unwrap().parse::<u64>().unwrap();

    let poops = lines.next().unwrap().split(",").collect::<Vec<_>>();

    let buses = poops.iter().filter(|&x| *x != "x").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let mut minimum = k;
    let mut minimal = 0;

    for x in buses.iter() {
        let v = (x - (k%x))%x;
        if v < minimum {
            minimum = v;
            minimal = *x;
        }
    }

    println!("{}", minimum * minimal);

    let mut delta = 0;
    let mut base = 0;
    let mut multiplier = 1;
    for i in poops.iter() {
        if *i != "x" {
            let curr = i.parse::<u64>().unwrap();
            for k in 0..curr {
                if (base + multiplier*k + delta) % curr == 0 {
                    base += multiplier * k;
                    multiplier *= curr;
                    break;
                }
            }
        }
        delta += 1;
    }

    println!("{} {}", base, multiplier);
}
