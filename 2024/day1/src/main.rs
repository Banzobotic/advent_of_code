fn main() {
    let input = include_str!("../input").trim();
    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();
        xs.push(split.next().unwrap().parse::<i32>().unwrap());
        ys.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    xs.sort_unstable();
    ys.sort_unstable();

    let mut sum = 0;

    for (x, y) in xs.iter().zip(ys.iter()) {
        sum += (x - y).abs()
    }
    
    println!("{sum}");

    let mut sum2 = 0;

    for x in xs.iter() {
        let mut count = 0;

        for y in ys.iter() {
            if x == y {
                count += 1
            }
        }

        sum2 += count * x;
    }

    println!("{sum2}");
}
