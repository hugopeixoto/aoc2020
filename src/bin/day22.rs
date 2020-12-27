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

fn play_recursive(deck1: &mut VecDeque<u8>, deck2: &mut VecDeque<u8>) -> usize {
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

            n1.truncate(c1 as usize);
            n2.truncate(c2 as usize);

            play_recursive(&mut n1, &mut n2) == 1
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

fn score(deck: &VecDeque<u8>) -> usize {
    deck.iter().enumerate().map(|(i, &c)| (deck.len() - i) * c as usize).sum::<usize>()
}

pub fn day22(input: String) -> (usize, usize) {
    let mut decks = input.split("\n\n");

    let mut deck1 = decks.next().unwrap()[9..].trim().split('\n').map(|x| x.parse::<u8>().unwrap()).collect::<VecDeque<_>>();
    let mut deck2 = decks.next().unwrap()[9..].trim().split('\n').map(|x| x.parse::<u8>().unwrap()).collect::<VecDeque<_>>();

    let mut deck1p2 = deck1.clone();
    let mut deck2p2 = deck2.clone();

    let winner1 = play(&mut deck1, &mut deck2);
    let p1 = score(if winner1 == 1 { &deck1 } else { &deck2 });

    let winner2 = play_recursive(&mut deck1p2, &mut deck2p2);
    let p2 = score(if winner2 == 1 { &deck1p2 } else { &deck2p2 });

    (p1, p2)
}

aoc2020::day!(day22, "day22.in", bench_day22);
