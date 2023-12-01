use aho_corasick::AhoCorasick;

fn searchf(s: &str) -> char {
    let num_pos = s.chars().position(|c| c.is_numeric()).unwrap_or(10000);
    let word_pos = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
        .iter()
        .map(|num| s.match_indices(num).next().unwrap_or((10000, "nope")))
        .min_by_key(|num| num.0)
        .unwrap();

    if num_pos < word_pos.0 {
        s.chars().find(|c| c.is_numeric()).unwrap()
    } else {
        match word_pos.1 {
            "one" => '1',
            "two" => '2',
            "three" => '3',
            "four" => '4',
            "five" => '5',
            "six" => '6',
            "seven" => '7',
            "eight" => '8',
            "nine" => '9',
            _ => {
                println!("{}", s);
                panic!();
            }
        }
    }
}

fn searchb(s: &str) -> char {
    let num_pos = s.len() - s.chars().rev().position(|c| c.is_numeric()).unwrap_or(0) - 1;
    let word_pos = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
        .iter()
        .map(|num| s.rmatch_indices(num).next().unwrap_or((0, "pain")))
        .max_by_key(|num| num.0)
        .unwrap();

    if num_pos >= word_pos.0 && s.chars().any(|c| c.is_numeric()) {
        s.chars().rev().find(|c| c.is_numeric()).unwrap()
    } else {
        match word_pos.1 {
            "one" => '1',
            "two" => '2',
            "three" => '3',
            "four" => '4',
            "five" => '5',
            "six" => '6',
            "seven" => '7',
            "eight" => '8',
            "nine" => '9',
            _ => {
                println!("{}", s);
                println!("{} {}", num_pos, word_pos.0);
                panic!();
            }
        }
    }
}

fn a() {
    println!(
        "{}",
        include_str!("input")
            .lines()
            .map(|l| format!(
                "{}{}",
                l.chars().find(|c| c.is_numeric()).unwrap(),
                l.chars().rev().find(|c| c.is_numeric()).unwrap()
            )
            .parse::<u32>()
            .unwrap())
            .sum::<u32>()
    );
}

fn b() {
    println!(
        "{}",
        include_str!("input")
            .lines()
            .map(|l| format!(
                "{}{}",
                searchf(l),
                searchb(l)
            )
            .parse::<u32>()
            .unwrap())
            .sum::<u32>()
    );
}

fn b_new() {
    println!(
        "{}",
        include_str!("input")
            .lines()
            .map(|l| {
                let search = AhoCorasick::new(["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]).unwrap();
                let expanded = search.replace_all(l, &["oonee", "ttwoo", "tthreee", "ffourr", "ffivee", "ssixx", "ssevenn", "eeightt", "nninee"]);
                search.replace_all(&expanded, &["1", "2", "3", "4", "5", "6", "7", "8", "9"])
            })
            .map(|l| format!(
                "{}{}",
                l.chars().find(|c| c.is_numeric()).unwrap(),
                l.chars().rev().find(|c| c.is_numeric()).unwrap()
            )
            .parse::<u32>()
            .unwrap())
            .sum::<u32>()
    );
}

fn main() {
    a();
    b();
    b_new();
}
