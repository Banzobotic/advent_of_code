use itertools::Itertools;

fn a() {
    let (times, distance) = include_str!("input")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .map(|n| n.parse::<u32>().unwrap())
        })
        .collect_tuple()
        .unwrap();

    println!(
        "{}",
        times
            .zip(distance)
            .map(|(t, d)| (1..t).map(|s| (s * (t - s) > d) as u32).sum::<u32>())
            .product::<u32>()
    )
}

fn b() {
    let (t, d) = include_str!("input")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<u128>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    println!(
        "{}",
        t as usize - (1..t).rev().position(|s| s * (t - s) > d).unwrap()
            - 1
            - (1..t).position(|s| s * (t - s) > d).unwrap()
    )
}

fn b_maths() {
    let (t, d) = include_str!("input")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<f64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    println!(
        "{:.0}",
        ((-t - (t * t - 4.0 * -1.0 * -d).sqrt()) / -2.0)
            - ((-t + (t * t - 4.0 * -1.0 * -d).sqrt()) / -2.0)
    );
}

fn main() {
    a();
    b();
    b_maths();
}
