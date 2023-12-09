use itertools::Itertools;

fn a() {
    fn get_next(nums: Vec<i32>) -> i32 {
        match nums.iter().all(|&n| n == 0) {
            true => 0,
            false => {
                nums.last().unwrap()
                    + get_next(nums.iter().tuple_windows().map(|(a, b)| b - a).collect())
            }
        }
    }
    println!(
        "{}",
        include_str!("input")
            .lines()
            .map(|l| l
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect())
            .map(|l| get_next(l))
            .sum::<i32>()
    );
}

fn b() {
    fn get_prev(nums: Vec<i32>) -> i32 {
        match nums.iter().all(|&n| n == 0) {
            true => 0,
            false => {
                nums[0]
                    - get_prev(nums.iter().tuple_windows().map(|(a, b)| b - a).collect())
            }
        }
    }
    println!(
        "{}",
        include_str!("input")
            .lines()
            .map(|l| l
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect())
            .map(|l| get_prev(l))
            .sum::<i32>()
    );
}

fn main() {
    a();
    b();
}
