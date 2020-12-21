#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

fn flipbits(n: usize) -> usize {
    n.reverse_bits() >> (64 - 10)
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
        2 => tile.reverse_bits(),
        3 => rotate(tile, 1).reverse_bits(),
        4 => tile.reverse_bits().swap_bytes(),
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
    let mut n = 0;
    for (i, c) in input[11..11*9].chars().enumerate() {
        if i % 11 != 0 && i % 11 < 9 {
            n = n * 2 + (c == '#') as u64;
        }
    }

    n
}

fn get(tile: &(usize, Vec<usize>, u64), rotation: usize, side: usize) -> usize {
    if rotation < 4 {
        tile.1[(4 + side - rotation) % 4]
    } else {
        flipbits(tile.1[(rotation - side) % 4])
    }
}

fn gen(mapping: &mut Vec<usize>, rotations: &mut Vec<usize>, used: &mut Vec<bool>, idx: usize, tiles: &Vec<(usize, Vec<usize>, u64)>, x: &Vec<Vec<usize>>, w: usize) -> bool {
    if idx == mapping.len() {
        true
    } else {
        if idx == 0 {
            for i in 0..tiles.len() {
                if !used[i] {
                    mapping[idx] = i;
                    used[i] = true;
                    for r in 0..8 {
                        rotations[idx] = r;

                        if gen(mapping, rotations, used, idx + 1, tiles, x, w) {
                            return true;
                        }
                    }
                    used[i] = false;
                }
            }
        } else if idx % w == 0 {
            let p = flipbits(get(&tiles[mapping[idx - w]], rotations[idx - w], 2));
            for &i in x[p].iter() {
                if !used[i] {
                    mapping[idx] = i;
                    used[i] = true;
                    'l1: for r in 0..8 {
                        rotations[idx] = r;

                        if p == get(&tiles[mapping[idx]], rotations[idx], 0) {
                            if gen(mapping, rotations, used, idx + 1, tiles, x, w) {
                                return true;
                            }

                            break 'l1;
                        }
                    }
                    used[i] = false;
                }
            }
        } else {
            let p = flipbits(get(&tiles[mapping[idx - 1]], rotations[idx - 1], 1));
            for &i in x[p].iter() {
                if !used[i] {
                    mapping[idx] = i;
                    used[i] = true;
                    'l2: for r in 0..8 {
                        rotations[idx] = r;

                        if p == get(&tiles[mapping[idx]], rotations[idx], 3) {
                            if idx / w == 0 || flipbits(get(&tiles[mapping[idx - w]], rotations[idx - w], 2)) == get(&tiles[mapping[idx]], rotations[idx], 0) {
                                if gen(mapping, rotations, used, idx + 1, tiles, x, w) {
                                    return true;
                                }
                            }

                            break 'l2;
                        }
                    }
                    used[i] = false;
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

    let w = if tiles.len() == 144 { 12 } else { 3 };

    let mut mapping = vec![];
    mapping.resize(tiles.len(), 0);

    let mut rotations = vec![];
    rotations.resize(tiles.len(), 0);

    let mut used = vec![];
    used.resize(tiles.len(), false);

    let mut x: Vec<Vec<_>> = vec![];
    x.resize(1024, vec![]);
    for (idx, tile) in tiles.iter().enumerate() {
        for &v in tile.1.iter() {
            x[v].push(idx);
            x[flipbits(v)].push(idx);
        }
    }

    let ret = gen(&mut mapping, &mut rotations, &mut used, 0, &tiles, &x, w);

    if !ret { return (0, 0); }

    let p1 =
        tiles[mapping[0]].0 *
        tiles[mapping[w - 1]].0 *
        tiles[mapping[w * (w - 1)]].0 *
        tiles[mapping[w * w - 1]].0;

    let mut image: Vec<bool> = vec![];

    for y in 0..w * 8 {
        for x in 0..w {
            let r = rotate(tiles[mapping[y/8*w+x]].2, rotations[y/8*w+x]);
            for i in 0..8 {
                image.push( ((r >> ((7-y%8)*8)) >> (7-i) ) & 1 == 1);
            }
        }
    }

    let mut p2 = 0;
    let spots = [
        (0, 18),
        (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18), (1, 19),
        (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16),
    ];

    for r in 0..8 {
        let rimage = flipimage(&image, r, 8*w);
        let px = |y: usize, x: usize| rimage[(y*w*8)+(x)];
        let mut monsters = 0;

        for y in 0 .. w*8 - 2 {
            for x in 0 .. w*8 - 20 {
                if spots.iter().all(|(dy, dx)| px(y+dy, x+dx)) {
                    monsters += 1;
                }
            }
        }

        if monsters > 0 {
            p2 = image.iter().filter(|&&c| c).count() - monsters*spots.len();
            break;
        }
    }

    (p1, p2)
}

aoc2020::day!(day20, "day20.in", bench_day20);
