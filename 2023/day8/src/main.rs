use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

fn a() {
    let (directions, network) = include_str!("input").split_once("\n\n").unwrap();
    let re = Regex::new(r"[A-Z]+").unwrap();
    let mut map = HashMap::new();

    for l in network.lines() {
        let mut ms = re.find_iter(l);
        map.insert(
            ms.next().unwrap().as_str(),
            (ms.next().unwrap().as_str(), ms.next().unwrap().as_str()),
        );
    }

    let mut sum = 0;
    let mut element = "AAA";

    for dir in directions.chars().cycle() {
        if dir == 'L' {
            element = map.get(element).unwrap().0
        } else {
            element = map.get(element).unwrap().1
        }

        sum += 1;

        if element == "ZZZ" {
            break;
        }
    }

    println!("{}", sum);
}

fn b() {
    let (directions, network) = include_str!("input").split_once("\n\n").unwrap();
    let re = Regex::new(r"[A-Z]+").unwrap();
    let mut map = HashMap::new();

    let mut elements = Vec::new();

    for l in network.lines() {
        let mut ms = re.find_iter(l);

        let first = ms.next().unwrap().as_str();
        if first.ends_with('A') {
            elements.push(first);
        }

        map.insert(
            first,
            (ms.next().unwrap().as_str(), ms.next().unwrap().as_str()),
        );
    }

    println!(
        "{}",
        elements
            .iter()
            .map(|&first| {
                let mut elements = vec![first];
                let mut count = 0;
                let mut z_count = 0;

                for dir in directions.chars().cycle() {
                    let element = if dir == 'L' {
                        map.get(elements.last().unwrap()).unwrap().0
                    } else {
                        map.get(elements.last().unwrap()).unwrap().1
                    };

                    count += 1;

                    if element.contains('Z') {
                        z_count += 1;
                    }

                    if z_count >= 2 {
                        if let Some(i) = elements.iter().rposition(|&e| e.contains('Z')) {
                            count -= i as u64;
                            break;
                        }
                    }

                    elements.push(element);
                }

                count
            })
            .reduce(lcm)
            .unwrap()
    );
}

fn main() {
    a();
    b();
}
