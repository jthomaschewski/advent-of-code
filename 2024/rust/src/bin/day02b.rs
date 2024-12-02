use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../inputs/day02.txt").expect("Failed to read input.txt");

    let mut safe = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|item| item.parse::<i32>().ok())
            .collect();

        if check_line(&nums, true) || check_line(&nums, false) {
            safe += 1;
        } else {
            // this can likely be optimized
            for i in 0..nums.len() {
                let mut reduced_nums = nums.clone();
                reduced_nums.remove(i);

                if check_line(&reduced_nums, true) || check_line(&reduced_nums, false) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    println!("result: {safe}")
}

fn check_line(line: &[i32], is_increase: bool) -> bool {
    for (i, cur) in line[1..].iter().enumerate() {
        let prev = line[i];

        if !is_safe(is_increase, *cur, prev) {
            return false;
        }
    }
    return true;
}

fn is_safe(is_increase: bool, prev: i32, cur: i32) -> bool {
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
