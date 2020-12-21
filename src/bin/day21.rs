#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(exclusive_range_pattern)]
#![feature(iterator_fold_self)]
#![feature(test)]
extern crate test;

use std::collections::*;

pub fn day21(input: String) -> (usize, String) {
    let mut map: HashMap<String, HashSet<_>> = HashMap::new();
    let mut total_ingredients = vec![];

    for line in input.lines() {
        let mut x = line[0..line.len() - 1].split(" (contains ");
        let ingredients = x.next().unwrap().split(' ').map(|x| x.to_string()).collect::<HashSet<_>>();
        let allergens = x.next().unwrap().split(", ").collect::<Vec<_>>();

        for ingredient in ingredients.iter() {
            total_ingredients.push(ingredient.to_string());
        }

        for &allergen in allergens.iter() {
            if !map.contains_key(&allergen.to_string()) {
                map.insert(allergen.to_string(), ingredients.clone());
            } else {
                let intersection = map[allergen].intersection(&ingredients).map(|x| x.to_string()).collect::<HashSet<_>>();
                map.insert(allergen.to_string(), intersection);
            }
        }
    }

    let mut dangerous_ingredients: HashSet<String> = HashSet::new();
    let mut i2a: HashMap<String, String> = HashMap::new();
    loop {
        let m = map.iter().filter(|&(k,v)| v.len() == 1 && !dangerous_ingredients.contains(k)).next();
        if m.is_none() {
            break;
        }

        let (allergen, ingredients) = m.unwrap();

        let ingredient = ingredients.iter().next().unwrap().clone();

        dangerous_ingredients.insert(ingredient.clone());
        i2a.insert(ingredient.clone(), allergen.clone());

        for (_,v) in map.iter_mut() {
            v.remove(&ingredient);
        }
    }

    let mut total = 0;
    for ingredient in total_ingredients.iter() {
        if !dangerous_ingredients.contains(ingredient) {
            total += 1;
        }
    }

    let mut dangerous_ingredients = dangerous_ingredients.iter().map(|x|x.to_string()).collect::<Vec<_>>();
    dangerous_ingredients.sort_by_key(|k| i2a[k].to_string());

    (total, dangerous_ingredients.join(","))
}

aoc2020::day!(day21, "day21.in", bench_day21);
