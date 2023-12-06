use itertools::Itertools;
use std::collections::HashMap;

fn a() {
    let (seeds, maps) = include_str!("input").split_once("\n\n").unwrap();
    let seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap());

    let mut hashmaps = vec![HashMap::new(); 7];

    maps.split("\n\n")
        .map(|m| m.split_once(":\n").unwrap().1.lines())
        .enumerate()
        .for_each(|(i, m)| {
            for l in m {
                let (destination, source, length) = l
                    .split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap();

                hashmaps[i].insert(
                    source..(source + length),
                    destination - source,
                );
            }
        });

    println!(
        "{}",
        seeds
            .map(|mut s| {
                hashmaps.iter().for_each(|map| {
                    s = map
                        .iter()
                        .find_map(|(source, destination)| {
                            if source.contains(&s) {
                                Some(s + destination)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(s)
                });
                s
            })
            .min()
            .unwrap()
    );
}

fn b() {
    let (seeds, maps) = include_str!("input").split_once("\n\n").unwrap();
    let mut seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .tuples()
        .map(|(start, len)| start..(start + len))
        .collect_vec();

    let mut hashmaps = vec![HashMap::new(); 7];

    maps.split("\n\n")
        .map(|m| m.split_once(":\n").unwrap().1.lines())
        .enumerate()
        .for_each(|(i, m)| {
            for l in m {
                let (destination, source, length) = l
                    .split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap();

                hashmaps[i].insert(
                    source..(source + length),
                    destination - source,
                );
            }
        });

    for map in hashmaps {
        // let mut temp = seeds.clone();
        // temp.sort_by_key(|r| r.start);
        // dbg!(&temp);

        let mut after_transform = Vec::new();

        for seed_range in seeds {
            let mut split_ranges = vec![seed_range.clone()];
            let mut last_ranges = Vec::new();

            while !split_ranges.is_empty() && split_ranges != last_ranges {
                let iterable = split_ranges.clone();
                last_ranges = split_ranges.clone();
                for range in iterable {
                    for (source, &offset) in map.iter() {
                        if source.contains(&range.start) && source.contains(&range.end) {
                            split_ranges.remove(split_ranges.iter().position(|r| *r == range).unwrap());
                            after_transform.push((range.start + offset)..(range.end + offset));
                            break;
                        } else if source.contains(&range.start) {
                            after_transform.push((range.start + offset)..(source.end + offset));
                            split_ranges.remove(split_ranges.iter().position(|r| *r == range).unwrap());
                            split_ranges.push(source.end..range.end);
                            break;
                        } else if source.contains(&range.end) {
                            split_ranges.remove(split_ranges.iter().position(|r| *r == range).unwrap());
                            split_ranges.push(range.start..source.start);
                            after_transform.push((source.start + offset)..(range.end + offset));
                            break;
                        }
                    }
                }
            }

            after_transform.append(&mut split_ranges);
        }

        seeds = after_transform;
    }

    // let mut temp = seeds.clone();
    // temp.sort_by_key(|r| r.start);
    // dbg!(&temp);

    println!("{}", seeds.iter().map(|range| range.start).min().unwrap())
}

fn b_brute() {
    let (seeds, maps) = include_str!("input").split_once("\n\n").unwrap();
    let seeds = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .tuples()
        .map(|(start, len)| start..(start + len))
        .collect_vec();

    let mut hashmaps = vec![HashMap::new(); 7];

    maps.split("\n\n")
        .map(|m| m.split_once(":\n").unwrap().1.lines())
        .enumerate()
        .for_each(|(i, m)| {
            for l in m {
                let (destination, source, length) = l
                    .split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap();

                hashmaps[i].insert(
                    destination..(destination + length),
                    destination - source,
                );
            }
        });

    println!(
        "{}",
        (0..).find(|s| {
            let mut s = *s;
            hashmaps.iter().rev().for_each(|map| {
                s = map
                    .iter()
                    .find_map(|(destination, offset)| {
                        if destination.contains(&s) {
                            Some(s - offset)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(s)
            });
            seeds.iter().any(|r| r.contains(&s))
        }).unwrap()
    );
}

fn main() {
    a();
    b();
    b_brute();
}
