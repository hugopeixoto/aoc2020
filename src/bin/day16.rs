#![feature(destructuring_assignment)]
#![feature(or_patterns)]
#![feature(test)]
extern crate test;

fn gen(mapping: &mut Vec<usize>, validities: &Vec<(usize, usize)>, index: usize) -> bool {
    if index == mapping.len() {
        return true;
    } else {
        for i in 0..30 {
            if (validities[index].1 & 1 << i) != 0 && !mapping[0..index].contains(&i) {
                mapping[index] = i;
                if gen(mapping, validities, index + 1) {
                    return true;
                }
            }
        }
    }

    false
}

fn bits(mut n: usize) -> usize {
    let mut b = 0;
    while n > 0 {
        b += n&1;
        n /= 2;
    }
    b
}

pub fn day16(input: String) -> (usize, usize) {
    let mut parts = input.trim().split("\n\n");

    let ranges = parts.next().unwrap();
    let yourticket = parts.next().unwrap();
    let nearbytickets = parts.next().unwrap();

    let mut rules = vec![];
    for range in ranges.trim().split('\n') {
        let mut rangeparts = range.split(": ");
        let name = rangeparts.next().unwrap();
        let rulestr = rangeparts.next().unwrap();

        let mut rulevec = vec![];
        for rule in rulestr.split(" or ") {
            let mut rangepartpart = rule.split('-');
            let min = rangepartpart.next().unwrap().parse::<usize>().unwrap();
            let max = rangepartpart.next().unwrap().parse::<usize>().unwrap();
            rulevec.push(min..=max);
        }

        rules.push((name, rulevec));
    }

    let myticket = yourticket.trim().split('\n').last().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut othertickets = nearbytickets.trim().split('\n');
    othertickets.next().unwrap();

    let mut othervalidticketsets = vec![];

    let mut errorrate = 0;
    for ticket in othertickets {
        let mut validities: Vec<usize> = vec![];

        let mut invalid = false;
        for strvalue in ticket.split(',') {
            let value = strvalue.parse::<usize>().unwrap();

            let v: usize = rules
                .iter()
                .enumerate()
                .fold(0, |acc, (i, rule)| acc + ((rule.1.iter().any(|range| range.contains(&value)) as usize) << i));

            if v == 0 {
                errorrate += value;
                invalid = true;
            }

            validities.push(v);
        }

        if !invalid {
            othervalidticketsets.push(validities);
        }
    }

    let mut ticketvalidities: Vec<(usize, usize)> = vec![];
    for (idx, &value) in myticket.iter().enumerate() {
        let v: usize = rules
            .iter()
            .enumerate()
            .fold(0, |acc, (i, rule)| acc + ((rule.1.iter().any(|range| range.contains(&value)) as usize) << i));

        ticketvalidities.push((idx, v));
    }

    for tv in othervalidticketsets.iter() {
        for (i, tvs) in tv.iter().enumerate() {
            ticketvalidities[i].1 = ticketvalidities[i].1 & tvs;
        }
    }

    ticketvalidities.sort_by_key(|&(_, v)| bits(v));

    let mut mapping = vec![];
    mapping.resize(myticket.len(), 0);

    gen(&mut mapping, &ticketvalidities, 0);

    let mut values = 1;
    for (i, m) in mapping.iter().enumerate() {
        if rules[*m].0.starts_with("departure") {
            values *= myticket[ticketvalidities[i].0];
        }
    }

    (errorrate, values)
}

aoc2020::day!(day16, "day16.in", bench_day16);
