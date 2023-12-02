fn a() {
    println!(
        "{}",
        include_str!("input")
            .lines()
            .map(|l| l.split_once(": ").unwrap().1)
            .map(|l| l
                .split("; ")
                .flat_map(|s| s.split(", "))
                .map(|cube| cube.split_once(' ').unwrap())
                .map(|(count, colour)| (count.parse::<u32>().unwrap(), colour))
                .map(|(count, colour)| match colour {
                    "blue" => count <= 14,
                    "green" => count <= 13,
                    "red" => count <= 12,
                    _ => panic!(),
                })
                .all(|b| b))
            .enumerate()
            .filter_map(|(i, b)| if b { Some(i + 1) } else { None })
            .sum::<usize>()
    );
}

fn b() {
    println!(
        "{}",
        include_str!("input")
            .lines()
            .map(|l| l.split_once(": ").unwrap().1)
            .map(|l| l
                .split("; ")
                .flat_map(|s| s.split(", "))
                .map(|cube| cube.split_once(' ').unwrap())
                .map(|(count, colour)| (count.parse::<u32>().unwrap(), colour))
                .fold((0, 0, 0), |(r, g, b), (count, colour)| match colour {
                    "red" => (r.max(count), g, b),
                    "green" => (r, g.max(count), b),
                    "blue" => (r, g, b.max(count)),
                    _ => panic!(),
                }))
            .map(|(r, g, b)| r * g * b)
            .sum::<u32>()
    );
}

fn main() {
    a();
    b();
}
