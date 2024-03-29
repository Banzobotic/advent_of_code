use itertools::Itertools;
use regex::Regex;

fn a() {
    let input: Vec<Vec<_>> = include_str!("input")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut sum = 0;
    let adjacent_offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    for (row, line) in input.iter().enumerate() {
        let mut buffer = Vec::new();
        let mut symbol = false;

        for (i, c) in line.iter().enumerate() {
            if c.is_numeric() {
                for (row_offset, i_offset) in adjacent_offsets.iter() {
                    let row_check = row as i32 + row_offset;
                    let i_check = i as i32 + i_offset;
                    if (0..input.len() as i32).contains(&row_check)
                        && (0..line.len() as i32).contains(&i_check)
                    {
                        symbol = match input[row_check as usize][i_check as usize] {
                            '0'..='9' | '.' => symbol,
                            _ => true,
                        };
                    }
                }

                buffer.push(c);
            } else {
                if symbol && !buffer.is_empty() {
                    let num: String = buffer.clone().into_iter().collect();
                    sum += num.parse::<i32>().unwrap();
                }

                buffer.clear();
                symbol = false;
            }
        }

        if symbol && !buffer.is_empty() {
            let num: String = buffer.clone().into_iter().collect();
            sum += num.parse::<i32>().unwrap();
        }
    }

    println!("{}", sum);
}

fn b() {
    let input: Vec<Vec<_>> = include_str!("input")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut sum = 0;
    let adjacent_offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    for (row, line) in input.iter().enumerate() {
        for (i, c) in line.iter().enumerate() {
            if *c != '*' {
                continue;
            }

            let mut contiguous = false;
            let mut nums = Vec::new();

            for (row_offset, i_offset) in adjacent_offsets.iter() {
                let row_check = row as i32 + row_offset;
                let i_check = i as i32 + i_offset;
                if (0..input.len() as i32).contains(&row_check)
                    && (0..line.len() as i32).contains(&i_check)
                {
                    let c_check = input[row_check as usize][i_check as usize];

                    if (!contiguous || *row_offset == 0) && c_check.is_numeric() {
                        let mut offset = 0;
                        let mut digit = input[row_check as usize][i_check as usize];
                        let mut buffer = Vec::new();

                        loop {
                            buffer.insert(0, digit);

                            offset -= 1;
                            digit = input[row_check as usize][(i_check + offset) as usize];

                            if !digit.is_numeric() {
                                break;
                            }

                            if i_check + offset == 0 {
                                buffer.insert(0, digit);
                                break;
                            }
                        }

                        offset = 1;
                        digit = input[row_check as usize][(i_check + offset) as usize];

                        while digit.is_numeric() {
                            buffer.push(digit);

                            offset += 1;
                            digit = input[row_check as usize][(i_check + offset) as usize];

                            if !digit.is_numeric() {
                                break;
                            }

                            if i_check + offset == line.len() as i32 - 1 {
                                buffer.push(digit);
                                break;
                            }
                        }

                        let num: String = buffer.iter().collect();
                        nums.push(num.parse().unwrap());
                    }

                    if *row_offset == 0 {
                        contiguous = false;
                    } else {
                        contiguous = c_check.is_numeric();
                    }
                }
            }

            if nums.len() == 2 {
                sum += nums.iter().product::<u32>();
            }
        }
    }

    println!("{}", sum);
}

fn a_new() {
    let input = include_str!("input").lines().collect_vec();
    let re = Regex::new(r"\d+").unwrap();

    println!(
        "{}",
        input
            .windows(3)
            .flat_map(|lines| lines[1]
                .char_indices()
                .filter(|(_, c)| !matches!(c, '0'..='9' | '.'))
                .map(|(i, _)| lines
                    .iter()
                    .flat_map(|l| re
                        .find_iter(l)
                        .filter(|m| i <= m.end() && m.start() <= i + 1)
                        .map(|m| m.as_str().parse::<u32>().unwrap()))
                    .sum::<u32>()))
            .sum::<u32>()
    );
}

fn b_new() {
    let input = include_str!("input").lines().collect_vec();
    let re = Regex::new(r"\d+").unwrap();

    println!(
        "{}",
        input
            .windows(3)
            .flat_map(|lines| lines[1]
                .char_indices()
                .filter(|(_, c)| *c == '*')
                .map(|(i, _)| lines
                    .iter()
                    .flat_map(|l| re
                        .find_iter(l)
                        .filter(|m| i <= m.end() && m.start() <= i + 1)
                        .map(|m| m.as_str().parse::<u32>().unwrap()))
                    .fold((0, 1), |(count, product), x| (count + 1, product * x))))
            .filter(|(count, _)| *count == 2)
            .fold(0, |sum, (_, x)| sum + x)
    );
}

fn main() {
    a();
    b();
    a_new();
    b_new();
}
