use std::fs::read_to_string;

use regex::Regex;

fn main() {
    let input = read_to_string("../inputs/day03.txt").expect("Failed to read input.txt");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    let sum: i32 = re
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [left, right])| {
            let left: i32 = left.parse().expect(&format!("not a number {left}"));
            let right: i32 = right.parse().expect(&format!("not a number {right}"));
            return left * right;
        })
        .sum();

    println!("{:?}", sum);
}
