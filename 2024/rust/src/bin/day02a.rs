use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../inputs/day02.txt").expect("Failed to read input.txt");

    let mut safe = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|item| item.parse::<i32>().ok())
            .collect();

        if check_line(&nums) {
            safe += 1;
        }
    }

    println!("result: {safe}")
}

fn check_line(line: &[i32]) -> bool {
    let is_increase = line[0] < line[1];

    for (i, cur) in line[1..].iter().enumerate() {
        let prev = line[i];

        if !is_safe(is_increase, *cur, prev) {
            return false;
        }
    }
    return true;
}

fn is_safe(is_increase: bool, cur: i32, prev: i32) -> bool {
    if is_increase && cur < prev {
        return false;
    }
    if !is_increase && cur > prev {
        return false;
    }
    let diff = cur.abs_diff(prev);
    if diff < 1 || diff > 3 {
        return false;
    }
    return true;
}
