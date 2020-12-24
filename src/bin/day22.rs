#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

use std::collections::*;

fn play(deck1: &mut VecDeque<u8>, deck2: &mut VecDeque<u8>) -> usize {
    if deck1.iter().max().unwrap() > deck2.iter().max().unwrap() {
        //return 1;
    }

    let mut seen = HashSet::new();
    while deck1.len() > 0 && deck2.len() > 0 {
        let state = (deck1.clone(), deck2.clone());

        if seen.contains(&state) {
            break;
        }

        seen.insert(state);

        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        let p1_wins_round = if deck1.len() >= c1 as usize && deck2.len() >= c2 as usize {
            let mut n1 = deck1.clone();
            let mut n2 = deck2.clone();

            while n1.len() > c1 as usize { n1.pop_back(); }
            while n2.len() > c2 as usize { n2.pop_back(); }

            play(&mut n1, &mut n2) == 1
        } else {
            c1 > c2
        };

        if p1_wins_round {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }

    if deck1.len() > 0 {
        1
    } else {
        2
    }
}

pub fn day22(input: String) -> (usize, usize) {
    let mut decks = input.split("\n\n");

    let mut deck1 = decks.next().unwrap()[9..].trim().split('\n').map(|x| x.parse::<u8>().unwrap()).collect::<VecDeque<_>>();
    let mut deck2 = decks.next().unwrap()[9..].trim().split('\n').map(|x| x.parse::<u8>().unwrap()).collect::<VecDeque<_>>();

    let mut deck1p2 = deck1.clone();
    let mut deck2p2 = deck2.clone();

    while deck1.len() > 0 && deck2.len() > 0 {
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        if c1 > c2 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }

    let s1 = deck1.iter().enumerate().map(|(i, &c)| (deck1.len() - i) * c as usize).sum::<usize>();
    let s2 = deck2.iter().enumerate().map(|(i, &c)| (deck2.len() - i) * c as usize).sum::<usize>();

    let winner = play(&mut deck1p2, &mut deck2p2);

    let score = if winner == 1 {
        deck1p2.iter().enumerate().map(|(i, &c)| (deck1p2.len() - i) * c as usize).sum::<usize>()
    } else {
        deck2p2.iter().enumerate().map(|(i, &c)| (deck2p2.len() - i) * c as usize).sum::<usize>()
    };

    (s1 + s2, score)
}

aoc2020::day!(day22, "day22.in", bench_day22);
