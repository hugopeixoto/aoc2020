#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

use std::collections::*;

pub fn day21(input: String) -> (usize, String) {
    let mut a2is: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut total_ingredients = vec![];

    for line in input.lines() {
        let mut x = line[0..line.len() - 1].split(" (contains ");
        let ingredients = x.next().unwrap().split(' ').collect::<HashSet<&str>>();

        for allergen in x.next().unwrap().split(", ") {
            match a2is.get_mut(allergen) {
                Some(i) => { i.retain(|e| ingredients.contains(e)); },
                None => { a2is.insert(allergen, ingredients.clone()); },
            }
        }

        for ingredient in ingredients {
            total_ingredients.push(ingredient);
        }
    }

    let mut dangerous_ingredients: HashSet<&str> = HashSet::new();
    let mut i2a: HashMap<&str, &str> = HashMap::new();
    loop {
        match a2is.iter().find(|(_, v)| v.len() == 1) {
            None => {
                break;
            },
            Some((&allergen, ingredients)) => {
                let &ingredient = ingredients.iter().next().unwrap();

                dangerous_ingredients.insert(ingredient);
                i2a.insert(ingredient, allergen);

                for v in a2is.values_mut() {
                    v.remove(ingredient);
                }
            }
        }
    }

    let mut total = 0;
    for ingredient in total_ingredients {
        if !dangerous_ingredients.contains(ingredient) {
            total += 1;
        }
    }

    let mut dangerous_ingredients = dangerous_ingredients.into_iter().collect::<Vec<_>>();
    dangerous_ingredients.sort_by_key(|k| &i2a[k]);

    (total, dangerous_ingredients.join(","))
}

aoc2020::day!(day21, "day21.in", bench_day21);
