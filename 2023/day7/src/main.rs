use std::collections::HashMap;

use itertools::Itertools;

const A_CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn a_score(a: &str) -> u64 {
    let mut counts = HashMap::new();

    for ac in a.chars() {
        counts
            .entry(ac)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut counts = counts.values().collect_vec();
    counts.sort_unstable_by_key(|x| std::cmp::Reverse(*x));

    10u64.pow(counts[0] + 10)
        + 10u64.pow(*counts.get(1).unwrap_or(&&0) + 9)
        + a.char_indices()
            .map(|(i, c)| {
                13usize.pow(5 - i as u32)
                    * A_CARDS.iter().rev().position(|&card| card == c).unwrap()
            })
            .sum::<usize>() as u64
}

fn a() {
    let mut input = include_str!("input")
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|l| (l.1.parse::<u32>().unwrap(), l.0))
        .collect_vec();

    input.sort_by_key(|(_, hand)| a_score(hand));

    println!(
        "{}",
        input
            .iter()
            .enumerate()
            .map(|(i, (bid, _))| (i + 1) * *bid as usize)
            .sum::<usize>()
    )
}

const B_CARDS: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn b_score(a: &str) -> u64 {
    let mut counts = HashMap::new();

    for ac in a.chars() {
        counts
            .entry(ac)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let joker_count = *counts.get(&'J').unwrap_or(&0);
    counts.remove(&'J');
    let mut counts = counts.values().collect_vec();
    counts.sort_unstable_by_key(|x| std::cmp::Reverse(*x));

    13u64.pow(*counts.first().unwrap_or(&&0) + 7 + joker_count)
        + 13u64.pow(*counts.get(1).unwrap_or(&&0) + 6)
        + a.char_indices()
            .map(|(i, c)| {
                13usize.pow(5 - i as u32)
                    * B_CARDS.iter().rev().position(|&card| card == c).unwrap()
            })
            .sum::<usize>() as u64
}

fn b() {
    let mut input = include_str!("input")
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|l| (l.1.parse::<u32>().unwrap(), l.0))
        .collect_vec();

    input.sort_by_key(|(_, hand)| b_score(hand));

    println!(
        "{}",
        input
            .iter()
            .enumerate()
            .map(|(i, (bid, _))| (i + 1) * *bid as usize)
            .sum::<usize>()
    )
}

fn main() {
    a();
    b();
}
