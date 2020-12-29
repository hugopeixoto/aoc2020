#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

use std::collections::*;

#[derive(Default, Clone, Hash, PartialEq, Eq)]
struct Deck {
    a: u128,
    b: u128,
    c: u128,
}

impl Deck {
    fn get(&self, index: usize) -> u8 {
        match index {
             0..21 => { (self.a >> ((index -  0) * 6) & 0x3F) as u8 },
            21..42 => { (self.b >> ((index - 21) * 6) & 0x3F) as u8 },
            42..50 => { (self.c >> ((index - 42) * 6) & 0x3F) as u8 },
            _ => { 0 },
        }
    }

    fn set(&mut self, index: usize, n: u8) {
        match index {
             0..21 => { self.a &= !(0x3F << (index - 0) * 6); self.a |= (n as u128) << ((index -  0) * 6); },
            21..42 => { self.b &= !(0x3F << (index - 21) * 6); self.b |= (n as u128) << ((index - 21) * 6); },
            42..50 => { self.c &= !(0x3F << (index - 42) * 6); self.c |= (n as u128) << ((index - 42) * 6); },
            _ => { },
        }
    }

    fn len1(&self) -> usize {
        ((self.c >> (8*6)) & 0x3F) as usize
    }

    fn len2(&self) -> usize {
        ((self.c >> (9*6)) & 0x3F) as usize
    }

    fn set_len1(&mut self, n: usize) {
        self.c &= !(63 << (8*6));
        self.c |= (n as u128) << (8*6);
    }

    fn set_len2(&mut self, n: usize) {
        self.c &= !(63 << (9*6));
        self.c |= (n as u128) << (9*6);
    }

    fn rotate1(&mut self) {
        let h = self.a & 0x3F;

        let c1 = self.len1();

        match c1 {
            1..=21 => {
                let keepmask = u128::MAX << (c1 * 6);
                self.a = (self.a & keepmask) | ((self.a & !keepmask) >> 6) | (h << ((c1-1)*6));
            },
            22..=42 => {
                let keepmask = u128::MAX << ((c1 - 21) * 6);
                self.a = (self.a >> 6) | ((self.b & 0x3F) << ((21 - 1)*6));
                self.b = (self.b & keepmask) | ((self.b & !keepmask) >> 6) | (h << ((c1 - 21 - 1)*6));
            },
            43..=50 => {
                let keepmask = u128::MAX << ((c1 - 42) * 6);
                self.a = (self.a >> 6) | ((self.b & 0x3F) << ((21 - 1)*6));
                self.b = (self.b >> 6) | ((self.c & 0x3F) << ((21 - 1)*6));
                self.c = (self.c & keepmask) | ((self.c & !keepmask) >> 6) | (h << ((c1 - 42 - 1)*6));
            },
            _ => {}
        }

        self.set_len1(c1 + 1);
        self.set_len2(self.len2() - 1);
    }

    fn rotate2(&mut self) {
        let c1 = self.len1();
        let c2 = self.len2();

        let h1 = (self.a & 0x3F) as u8;
        let h2 = self.get(c1);

        match c1 {
            1..=21 => {
                let keepmask = u128::MAX << ((c1 - 1) * 6);
                let keepmask3 = u128::MAX << (8 * 6);
                self.a = (((self.b & 0xFFF) << ((21 - 2)*6)) & keepmask) | ((self.a >> 12) & keepmask) | ((self.a >> 6) & !keepmask);
                self.b = ((self.c & 0xFFF) << ((21 - 2)*6)) | (self.b >> 12);
                self.c = (self.c & keepmask3) | ((self.c & !keepmask3) >> 12);
            },
            22..=42 => {
                let keepmask = u128::MAX << ((c1 - 21 - 1) * 6);
                let keepmask3 = u128::MAX << (8 * 6);

                self.a = (self.a >> 6) | ((self.b & 0x3F) << ((21 - 1)*6));
                self.b = (((self.c & 0xFFF) << ((21 - 2)*6)) & keepmask) | ((self.b >> 12) & keepmask) | ((self.b >> 6) & !keepmask);
                self.c = (self.c & keepmask3) | ((self.c & !keepmask3) >> 12);
            },
            43..=50 => {
                let keepmask = u128::MAX << ((c1 - 42 - 1) * 6);
                let keepmask3 = u128::MAX << (8 * 6);

                self.a = (self.a >> 6) | ((self.b & 0x3F) << ((21 - 1)*6));
                self.b = (self.b >> 6) | ((self.c & 0x3F) << ((21 - 1)*6));
                self.c = (self.c & keepmask3) | (((self.c & !keepmask3) >> 12) & keepmask) | (((self.c & !keepmask3) >> 6) & !keepmask);
            },
            _ => {}
        }

        self.set(c1 + c2 - 2, h2);
        self.set(c1 + c2 - 1, h1);

        self.set_len1(c1 - 1);
        self.set_len2(c2 + 1);
    }

    fn subdeck(&self) -> Self {
        let mut sd = Deck::default();
        let mut sd2 = Deck::default();

        let c1 = self.len1();
        let c2 = self.len2();

        let h1 = (self.a & 0x3F) as u8;
        let h2 = self.get(c1);

        let drops = 1 + c1 - h1 as usize;

        //match c1 {
        //    1..=21 => {
        //        let h1mask = !(u128::MAX << (h1 * 6));
        //        let h1h2mask = !(u128::MAX << ((h1 + h2) * 6));
        //        let carrymask = !(u128::MAX << (drops*6));
        //        let keepmask3 = u128::MAX << (8 * 6);

        //        sd2.a =
        //            (((self.b & carrymask) << ((21 - drops)*6)) & !h1mask & h1h2mask) |
        //            ((self.a >> (drops * 6)) & !h1mask & h1h2mask) |
        //            ((self.a >> 6) & h1mask);

        //        sd2.b =
        //            ((self.c & carrymask) << ((21 - drops)*6)) |
        //            (self.b >> (drops * 6));

        //        sd2.c = (self.c & keepmask3) | ((self.c & !keepmask3) >> (drops * 6));
        //    },
        //    //22..=42 => {
        //    _ => {},
        //    //_ => {
        //        //let keepmask = u128::MAX << (h1 * 6);
        //        //let carrymask = !(u128::MAX << (drops*6));
        //        //let keepmask3 = u128::MAX << (8 * 6);

        //        //println!("keepmask: {:0128b}", !keepmask);
        //        //sd.a =
        //        //    (((self.b & carrymask) << ((21 - drops)*6)) & keepmask) |
        //        //    ((self.a >> (drops * 6)) & keepmask) |
        //        //    ((self.a >> 6) & !keepmask);
        //        //sd.b = ((self.c & carrymask) << ((21 - drops)*6)) | (self.b >> (drops * 6));
        //        //sd.c = (self.c & keepmask3) | ((self.c & !keepmask3) >> (drops * 6));
        //    //},
        //    //43..=50 => {},
        //    //_ => { panic!(); },
        //}

        //sd2.set_len1(h1 as usize);
        //sd2.set_len2(h2 as usize);

        for i in 0..h1 as usize {
            sd.set(i, self.get(i + 1));
        }

        for i in 0..h2 as usize {
            sd.set(i + h1 as usize, self.get(c1 + i + 1));
        }

        sd.set_len1(h1 as usize);
        sd.set_len2(h2 as usize);

        //if c1 <= 21 {
        //    if sd != sd2 {
        //        println!("before:");
        //        println!("{:0128b}", self.a);
        //        println!("{:0128b}", self.b);
        //        println!("{:0128b}", self.c);
        //        println!("{} {}", self.len1(), self.len2());

        //        println!("sd:");
        //        println!("{:0128b}", sd.a);
        //        println!("{:0128b}", sd.b);
        //        println!("{:0128b}", sd.c);
        //        println!("{} {}", sd.len1(), sd.len2());

        //        println!("sd2:");
        //        println!("{:0128b}", sd2.a);
        //        println!("{:0128b}", sd2.b);
        //        println!("{:0128b}", sd2.c);
        //        println!("{} {}", sd2.len1(), sd2.len2());

        //        panic!();
        //    }
        //}

        sd
    }
}

impl From<(Vec<u8>, Vec<u8>)> for Deck {
    fn from(decks: (Vec<u8>, Vec<u8>)) -> Self {
        let mut deck = Deck::default();

        let c1 = decks.0.len();
        let c2 = decks.1.len();

        deck.set_len1(c1);
        deck.set_len2(c2);

        for i in 0..c1 {
            deck.set(i, decks.0[i]);
        }

        for i in 0..c2 {
            deck.set(i + c1, decks.1[i]);
        }

        deck
    }
}

impl core::fmt::Debug for Deck {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str("Deck([")?;

        let c1 = self.len1();
        let c2 = self.len2();

        for i in 0..c1 {
            write!(formatter, "{}{}", self.get(i), if i == c1-1 { "" } else { ", " })?;
        }

        formatter.write_str("], [")?;

        for i in 0..c2 {
            write!(formatter, "{}{}", self.get(i + c1), if i == c2-1 { "" } else { ", " })?;
        }

        formatter.write_str("])")
    }
}

fn play(decks: &mut Deck) -> usize {
    while decks.len1() > 0 && decks.len2() > 0 {
        let c1 = decks.get(0);
        let c2 = decks.get(decks.len1());

        let p1_wins_round = c1 > c2;
        if p1_wins_round {
            decks.rotate1();
        } else {
            decks.rotate2();
        }
    }

    if decks.len1() > 0 {
        1
    } else {
        2
    }
}

#[derive(Default, Debug)]
struct Stats {
    depth: usize,
    maxdepth: usize,
    maxlength: usize,
    breaks: usize,
    insertions: usize,
    subgames: usize,
    subdeck1: usize,
    subdeck2: usize,
    subdeck3: usize,
    shortcircuit_hits: usize,
    shortcircuit_misses: usize,
}

fn play_recursive(decks: &mut Deck, stats: &mut Stats) -> usize {
    let mut seen = HashSet::new();

    stats.subgames += 1;

    if stats.depth > stats.maxdepth {
        stats.maxdepth = stats.depth;
    }

    let mut shortcircuit = false;
    if stats.depth != 0 {
        let l1 = decks.len1();
        let m1 = (0..l1).map(|i| decks.get(i)).max().unwrap();
        let m2 = (0..decks.len2()).map(|i| decks.get(i + l1)).max().unwrap();

        shortcircuit = m1 > m2;
        if shortcircuit {
            stats.shortcircuit_hits += 1;
            return 1;
        } else {
            stats.shortcircuit_misses += 1;
        }
    }

    let mut length = 0;
    while decks.len1() > 0 && decks.len2() > 0 {
        length += 1;
        if seen.contains(decks) {
            stats.breaks += 1;
            break;
        }

        seen.insert(decks.clone());
        stats.insertions += 1;

        let c1 = decks.get(0);
        let c2 = decks.get(decks.len1());

        if c1 <= 21 { stats.subdeck1 += 1; }
        else if c1 <= 42 { stats.subdeck2 += 1; }
        else { stats.subdeck3 += 1; }

        stats.depth += 1;
        let p1_wins_round = if decks.len1() > c1 as usize && decks.len2() > c2 as usize {
            play_recursive(&mut decks.subdeck(), stats) == 1
        } else {
            c1 > c2
        };
        stats.depth -= 1;

        if p1_wins_round {
            decks.rotate1();
        } else {
            decks.rotate2();
        }
    }

    if length > stats.maxlength {
        stats.maxlength = length;
    }

    if shortcircuit {
        if decks.len1() == 0 {
            panic!();
        }
    }

    if decks.len1() > 0 {
        1
    } else {
        2
    }
}

fn score2(decks: &Deck, winner: usize) -> usize {
    match winner {
        1 => { (0..decks.len1()).map(|i| (decks.len1() - i) * decks.get(i) as usize).sum::<usize>() },
        2 => { (0..decks.len2()).map(|i| (decks.len2() - i) * decks.get(decks.len1() + i) as usize).sum::<usize>() },
        _ => { panic!(); },
    }
}

pub fn day22(input: String) -> (usize, usize) {
    let mut decks = input.split("\n\n");

    let mut stats = Stats::default();

    let deck1 = decks.next().unwrap()[9..].trim().split('\n').map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>();
    let deck2 = decks.next().unwrap()[9..].trim().split('\n').map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>();

    let mut d1 = Deck::from((deck1.clone(), deck2.clone()));
    let winner1 = play(&mut d1);
    let p1 = score2(&d1, winner1);

    let mut d2 = Deck::from((deck1.clone(), deck2.clone()));
    let winner2 = play_recursive(&mut d2, &mut stats);
    let p2 = score2(&d2, winner2);

    println!("{:?}", stats);

    (p1, p2)
}

aoc2020::day!(day22, "day22.in", bench_day22);
