use itertools::Itertools;

fn a() {
    println!(
        "{}",
        include_str!("input")
            .lines()
            .map(|l| l.split_once(": ").unwrap().1)
            .map(|l| l.split_once(" | ").unwrap())
            .map(|(ws, nums)| (
                ws.split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect_vec(),
                nums.split_whitespace().map(|n| n.parse::<u32>().unwrap())
            ))
            .map(|(ws, nums)| nums.filter(|n| ws.contains(n)).count())
            .map(|nums| if nums == 0 {
                0
            } else {
                2u32.pow(nums as u32 - 1)
            })
            .sum::<u32>()
    )
}

fn b() {
    let cards = include_str!("input")
        .lines()
        .map(|l| l.split_once(": ").unwrap().1)
        .map(|l| l.split_once(" | ").unwrap())
        .map(|(ws, nums)| {
            (
                ws.split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect_vec(),
                nums.split_whitespace().map(|n| n.parse::<u32>().unwrap()),
            )
        })
        .map(|(ws, nums)| nums.filter(|n| ws.contains(n)).count())
        .collect_vec();

    let mut counts = Vec::new();

    for i in 0..cards.len() {
        let mut total_count = 1;

        for (j, (card2, count)) in cards.iter().zip(counts.iter()).take(i).enumerate() {
            if i - j <= *card2 {
                total_count += count
            }
        }

        counts.push(total_count);
    }

    println!("{}", counts.iter().sum::<u32>());
}

fn main() {
    a();
    b();
}
