#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

use std::collections::*;

fn flipbits64(mut n: u64) -> u64 {
    let mut f = 0;

    let mut i = 0;
    while i < 64 {
        f = f << 1 | (n & 1);
        n >>= 1;
        i += 1;
    }

    f
}

fn flipbits(mut n: usize) -> usize {
    let mut f = 0;

    let mut i = 0;
    while i < 10 {
        f = f << 1 | (n & 1);
        n >>= 1;
        i += 1;
    }

    f
}

fn rotate(tile: u64, rotation: usize) -> u64 {
    match rotation {
        0 => tile,
        1 => {
            (0..8).map(|i| 1&(tile >> ((i+1)*8 - 1))).fold(0, |acc, d| acc * 2 + d) << (64-1*8) |
            (0..8).map(|i| 1&(tile >> ((i+1)*8 - 2))).fold(0, |acc, d| acc * 2 + d) << (64-2*8) |
            (0..8).map(|i| 1&(tile >> ((i+1)*8 - 3))).fold(0, |acc, d| acc * 2 + d) << (64-3*8) |
            (0..8).map(|i| 1&(tile >> ((i+1)*8 - 4))).fold(0, |acc, d| acc * 2 + d) << (64-4*8) |
            (0..8).map(|i| 1&(tile >> ((i+1)*8 - 5))).fold(0, |acc, d| acc * 2 + d) << (64-5*8) |
            (0..8).map(|i| 1&(tile >> ((i+1)*8 - 6))).fold(0, |acc, d| acc * 2 + d) << (64-6*8) |
            (0..8).map(|i| 1&(tile >> ((i+1)*8 - 7))).fold(0, |acc, d| acc * 2 + d) << (64-7*8) |
            (0..8).map(|i| 1&(tile >> ((i+1)*8 - 8))).fold(0, |acc, d| acc * 2 + d) << (64-8*8)
        },
        2 => flipbits64(tile),
        3 => flipbits64(rotate(tile, 1)),
        4 => {
            (0..8).map(|i| 1&(tile >> (56 + i))).fold(0, |acc, d| acc * 2 + d) << (64-1*8) |
            (0..8).map(|i| 1&(tile >> (48 + i))).fold(0, |acc, d| acc * 2 + d) << (64-2*8) |
            (0..8).map(|i| 1&(tile >> (40 + i))).fold(0, |acc, d| acc * 2 + d) << (64-3*8) |
            (0..8).map(|i| 1&(tile >> (32 + i))).fold(0, |acc, d| acc * 2 + d) << (64-4*8) |
            (0..8).map(|i| 1&(tile >> (24 + i))).fold(0, |acc, d| acc * 2 + d) << (64-5*8) |
            (0..8).map(|i| 1&(tile >> (16 + i))).fold(0, |acc, d| acc * 2 + d) << (64-6*8) |
            (0..8).map(|i| 1&(tile >> ( 8 + i))).fold(0, |acc, d| acc * 2 + d) << (64-7*8) |
            (0..8).map(|i| 1&(tile >> ( 0 + i))).fold(0, |acc, d| acc * 2 + d) << (64-8*8)
        },
        5 => rotate(rotate(tile, 4), 1),
        6 => rotate(rotate(tile, 4), 2),
        7 => rotate(rotate(tile, 4), 3),
        _ => { panic!(); },
    }
}

fn flipimage(image: &Vec<bool>, rotation: usize, w: usize) -> Vec<bool> {
    match rotation {
        0 => image.clone(),
        1 => (0..w*w).map(|i| {
            let x = i%w;
            let y = i/w;

            image[(w-1)*w+y - w*x]
        }).collect(),
        2 => flipimage(&flipimage(image, 1, w), 1, w),
        3 => flipimage(&flipimage(image, 2, w), 1, w),
        4 => (0..w*w).map(|i| {
            let x = i%w;
            let y = i/w;

            image[y*w + (w - 1 - x)]
        }).collect(),
        5 => flipimage(&flipimage(image, 4, w), 1, w),
        6 => flipimage(&flipimage(image, 4, w), 2, w),
        7 => flipimage(&flipimage(image, 4, w), 3, w),
        _ => { panic!(); }
    }
}

fn parse_numbers(input: &str) -> Vec<usize> {
    let top = input[0..10].chars().fold(0, |acc, d| acc * 2 + match d { '#' => 1, _ => 0 });
    let right = input[0..109].chars().enumerate().filter(|(i, _)| i % 11 == 9).fold(0, |acc, (_, d)| acc * 2 + match d { '#' => 1, _ => 0 });
    let bottom = input[99..109].chars().fold(0, |acc, d| acc * 2 + match d { '#' => 1, _ => 0 });
    let left = input[0..109].chars().enumerate().filter(|(i, _)| i % 11 == 0).fold(0, |acc, (_, d)| acc * 2 + match d { '#' => 1, _ => 0 });

    vec![top, right, flipbits(bottom), flipbits(left)]
}

fn parse_image(input: &str) -> u64 {
  (0..64)
    .map(|i| input.chars().nth(((i/8)+1)*11 + (i%8 + 1)).unwrap() == '#')
    .fold(0, |acc, d| acc * 2 + (d as u64))
}

fn get(tile: &(usize, Vec<usize>, u64), rotation: usize, side: usize) -> usize {
    let (side, flip) = match (rotation, side) {
        (0, 0) => (0, false),
        (0, 1) => (1, false),
        (0, 2) => (2, false),
        (0, 3) => (3, false),

        (1, 0) => (3, false),
        (1, 1) => (0, false),
        (1, 2) => (1, false),
        (1, 3) => (2, false),

        (2, 0) => (2, false),
        (2, 1) => (3, false),
        (2, 2) => (0, false),
        (2, 3) => (1, false),

        (3, 0) => (1, false),
        (3, 1) => (2, false),
        (3, 2) => (3, false),
        (3, 3) => (0, false),

        (4, 0) => (0, true),
        (4, 1) => (3, true),
        (4, 2) => (2, true),
        (4, 3) => (1, true),

        (5, 0) => (1, true),
        (5, 1) => (0, true),
        (5, 2) => (3, true),
        (5, 3) => (2, true),

        (6, 0) => (2, true),
        (6, 1) => (1, true),
        (6, 2) => (0, true),
        (6, 3) => (3, true),

        (7, 0) => (3, true),
        (7, 1) => (2, true),
        (7, 2) => (1, true),
        (7, 3) => (0, true),

        _ => { panic!(); },
    };

    let r = tile.1[side];
    if flip {
        flipbits(r)
    } else {
        r
    }
}

fn gen(mapping: &mut Vec<usize>, rotations: &mut Vec<usize>, idx: usize, tiles: &Vec<(usize, Vec<usize>, u64)>, w: usize) -> bool {
    if idx == mapping.len() {
        true
    } else {
        for i in 0..tiles.len() {
            if !mapping[0..idx].contains(&i) {
                mapping[idx] = i;
                for r in 0..8 {
                    rotations[idx] = r;

                    if idx % w == 0 || get(&tiles[mapping[idx - 1]], rotations[idx - 1], 1) == flipbits(get(&tiles[mapping[idx]], rotations[idx], 3)) {
                        if idx / w == 0 || get(&tiles[mapping[idx - w]], rotations[idx - w], 2) == flipbits(get(&tiles[mapping[idx]], rotations[idx], 0)) {
                            if gen(mapping, rotations, idx + 1, tiles, w) {
                                return true;
                            }
                        }
                    }

                }
            }
        }

        false
    }
}

pub fn day20(input: String) -> (usize, usize) {
    let tiles = input.split("\n\n").map(|t| {
       let id = t[5..9].parse::<usize>().unwrap();

        let numbers = parse_numbers(&t[11..]);
        let image = parse_image(&t[11..]);

       (id, numbers, image)
    }).collect::<Vec<_>>();

    println!("{:?}", tiles.len());
    let w = if tiles.len() == 144 { 12 } else { 3 };

    let mut mapping = vec![];
    mapping.resize(tiles.len(), 0);
    let mut rotations = vec![];
    rotations.resize(tiles.len(), 0);

    let ret = gen(&mut mapping, &mut rotations, 0, &tiles, w);

    println!("success: {}", ret);

    for i in 0..mapping.len() {
        print!("{}({}) {:064b}", tiles[mapping[i]].0, rotations[i], tiles[mapping[i]].2);
        if i % w == w - 1 { println!(""); }
    }

    let total =
        tiles[mapping[0]].0 *
        tiles[mapping[w - 1]].0 *
        tiles[mapping[w * (w - 1)]].0 *
        tiles[mapping[w * w - 1]].0;

    let p1 = total;

    let mut image: Vec<bool> = vec![];

    for y in 0..w * 8 {
        for x in 0..w {
            let r = rotate(tiles[mapping[y/8*w+x]].2, rotations[y/8*w+x]);
            for i in 0..8 {
                image.push( ((r >> ((7-y%8)*8)) >> (7-i) ) & 1 == 1);
            }
        }
    }

    for y in 0..w * 8 {
        for x in 0..w * 8 {
            print!("{}", if image[y*w*8 + x] { '#' } else { '.' });
        }
        println!("");
    }

    let mut total = 0;
    let mut monsters = 0;

    let spots = [
        (0, 18),
        (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18), (1, 19),
        (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16),
    ];

    let mut marks = HashSet::new();
    for r in 0..8 {
        println!("flip {}", r);
        let rimage = flipimage(&image, r, 8*w);
        let px = |y: usize, x: usize| rimage[(y*w*8)+(x)];

        for y in 0 .. w*8 - 2 {
            for x in 0 .. w*8 - 20 {
                if spots.iter().all(|(dy, dx)| px(y+dy, x+dx)) {
                    println!("monster @ {} {}", x, y);
                    for (dy, dx) in spots.iter() {
                        marks.insert((y+dy, x+dx));
                    }
                    monsters += 1;
                }
            }
        }

    }

    total += image.iter().filter(|&&c| c).count() - marks.len();

    let p2 = total;

    (p1, p2)
}

aoc2020::day!(day20, "day20.in", bench_day20);
