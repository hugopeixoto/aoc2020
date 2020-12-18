use std::fs::read_to_string;
use std::collections::*;

fn gen(mapping: &mut Vec<usize>, validities: &Vec<(usize, Vec<usize>)>, index: usize) -> bool {
    if index == mapping.len() {
        return true;
    } else {
        for i in validities[index].1.iter() {
            if !mapping[0..index].contains(&i) {
                mapping[index] = *i;
                if gen(mapping, validities, index + 1) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn main() {
    let text = read_to_string("inputs/day16.in").unwrap();

    let mut parts = text.trim().split("\n\n");

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

    let mut yourticketsplit = yourticket.trim().split('\n');
    yourticketsplit.next().unwrap();
    let myticket = yourticketsplit.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut othertickets = nearbytickets.trim().split('\n');
    othertickets.next().unwrap();

    let mut othervalidticketsets = vec![];

    let mut errorrate = 0;
    for ticket in othertickets {
        let values = ticket.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let mut validities: Vec<HashSet<usize>> = vec![];

        let mut invalid = false;
        for value in values.iter() {
            let v: HashSet<_> = rules
                .iter()
                .enumerate()
                .filter(|(_index, rule)| rule.1.iter().any(|range| range.contains(&value)))
                .map(|(index, _)| index)
                .collect();

            if v.is_empty() {
                errorrate += value;
                invalid = true;
            }

            validities.push(v);
        }

        if !invalid {
            othervalidticketsets.push(validities);
        }
    }

    let mut ticketvalidities: Vec<HashSet<usize>> = vec![];
    for &value in myticket.iter() {
        let v = rules.iter()
            .enumerate()
            .filter(|(_index, rule)| rule.1.iter().any(|range| range.contains(&value)))
            .map(|(index, _)| index)
            .collect();

        ticketvalidities.push(v);
    }

    for tv in othervalidticketsets.iter() {
        for (i, tvs) in tv.iter().enumerate() {
            ticketvalidities[i] = ticketvalidities[i].intersection(tvs).map(|&v| v).collect();
        }
    }

    let mut tvvec: Vec<(usize, Vec<usize>)> = ticketvalidities.iter().map(|tvs| tvs.iter().map(|&v| v).collect()).enumerate().collect();

    tvvec.sort_by_key(|(_, v)| v.len());

    println!("{:?}", ticketvalidities);

    let mut mapping = vec![];
    mapping.resize(myticket.len(), 0);

    gen(&mut mapping, &tvvec, 0);

    let mut values = 1;
    for (i, m) in mapping.iter().enumerate() {
        if rules[*m].0.starts_with("departure") {
            values *= myticket[tvvec[i].0];
        }
    }

    println!("{}", errorrate);
    println!("{}", values);
}
