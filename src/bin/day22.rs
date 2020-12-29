#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

use std::collections::*;

fn play(deck1: &mut VecDeque<u8>, deck2: &mut VecDeque<u8>) -> usize {
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

    if deck1.len() > 0 {
        1
    } else {
        2
    }
}

fn play_recursive(decks: &mut [(VecDeque<u8>, VecDeque<u8>)]) -> usize {
    let mut seen = HashSet::new();

    while decks[0].0.len() > 0 && decks[0].1.len() > 0 {
        let state = (
            decks[0].0.clone(),
            decks[0].1.clone(),
        );

        if seen.contains(&state) {
            break;
        }

        seen.insert(state);

        let c1 = decks[0].0.pop_front().unwrap();
        let c2 = decks[0].1.pop_front().unwrap();

        let p1_wins_round = if decks[0].0.len() >= c1 as usize && decks[0].1.len() >= c2 as usize {
            decks[1].0.clear();
            decks[1].1.clear();

            for i in 0..c1 as usize { let e = decks[0].0[i]; decks[1].0.push_back(e); }
            for i in 0..c2 as usize { let e = decks[0].1[i]; decks[1].1.push_back(e); }

            play_recursive(&mut decks[1..]) == 1
        } else {
            c1 > c2
        };

        if p1_wins_round {
            decks[0].0.push_back(c1);
            decks[0].0.push_back(c2);
        } else {
            decks[0].1.push_back(c2);
            decks[0].1.push_back(c1);
        }
    }

    if decks[0].0.len() > 0 {
        1
    } else {
        2
    }
}

fn score(deck: &VecDeque<u8>) -> usize {
    deck.iter().enumerate().map(|(i, &c)| (deck.len() - i) * c as usize).sum::<usize>()
}

pub fn day22(input: String) -> (usize, usize) {
    let mut decks = input.split("\n\n");

    let mut deck1 = decks.next().unwrap()[9..].trim().split('\n').map(|x| x.parse::<u8>().unwrap()).collect::<VecDeque<_>>();
    let mut deck2 = decks.next().unwrap()[9..].trim().split('\n').map(|x| x.parse::<u8>().unwrap()).collect::<VecDeque<_>>();

    let deck1p2 = deck1.clone();
    let deck2p2 = deck2.clone();

    let winner1 = play(&mut deck1, &mut deck2);
    let p1 = score(if winner1 == 1 { &deck1 } else { &deck2 });

    let mut nestlings = (0..26)
        .map(|_| (VecDeque::with_capacity(50), VecDeque::with_capacity(50)))
        .collect::<Vec<_>>();

    nestlings[0].0 = deck1p2;
    nestlings[0].1 = deck2p2;

    let winner2 = play_recursive(&mut nestlings);
    let p2 = score(if winner2 == 1 { &nestlings[0].0 } else { &nestlings[0].1 });

    (p1, p2)
}

aoc2020::day!(day22, "day22.in", bench_day22);
