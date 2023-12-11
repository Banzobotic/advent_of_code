use std::collections::HashSet;

use itertools::Itertools;

fn ab() {
    let input = include_str!("input");
    let mut non_empty_columns = HashSet::new();
    let mut empty_lines = Vec::new();
    let mut empty_columns = Vec::new();

    for (i, line) in input.lines().enumerate() {
        if line.chars().all(|c| c == '.') {
            empty_lines.push(i);
        }

        for (j, c) in line.char_indices() {
            if c == '#' {
                non_empty_columns.insert(j);
            }
        }
    }

    for j in 0..input.lines().next().unwrap().len() {
        if !non_empty_columns.contains(&j) {
            empty_columns.push(j)
        }
    }

    println!(
        "{:?}",
        input
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                l.char_indices()
                    .flat_map(move |(j, c)| if c == '#' { Some((i, j)) } else { None })
            })
            .tuple_combinations()
            .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2)
                + y1.abs_diff(y2)
                + (x1.min(x2)..x1.max(x2)).filter(|i| empty_lines.contains(i)).count()
                + (y1.min(y2)..=y1.max(y2)).filter(|i| empty_columns.contains(i)).count())
            .sum::<usize>()
    );

    println!(
        "{:?}",
        input
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                l.char_indices()
                    .flat_map(move |(j, c)| if c == '#' { Some((i, j)) } else { None })
            })
            .tuple_combinations()
            .map(|((x1, y1), (x2, y2))| x1.abs_diff(x2)
                + y1.abs_diff(y2)
                + (x1.min(x2)..x1.max(x2)).filter(|i| empty_lines.contains(i)).count() * 999_999
                + (y1.min(y2)..=y1.max(y2)).filter(|i| empty_columns.contains(i)).count() * 999_999)
            .sum::<usize>()
    );
}

fn main() {
    ab();
}
