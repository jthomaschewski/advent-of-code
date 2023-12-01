use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../inputs/day01.txt").expect("Failed to read input.txt");

    let sum: usize = input
        .lines()
        .map(|line| str::chars(&line).collect::<Vec<char>>())
        .map(|line| {
            let mut first_digit = 0;
            let mut last_digit = 0;

            for i in 0..line.len() {
                let digit = to_digit(&line[i..].iter().collect::<String>());
                if let Some(digit) = digit {
                    if first_digit == 0 {
                        first_digit = digit;
                    }
                    last_digit = digit;
                }
            }
            return first_digit * 10 + last_digit;
        })
        .sum();

    println!("{}", sum)
}

#[rustfmt::skip]
const DIGITS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn to_digit(slice: &str) -> Option<usize> {
    let res = DIGITS
        .iter()
        .enumerate()
        .find(|(_, val)| slice.starts_with(*val));

    if let Some((idx, _)) = res {
        return Some(idx % 9 + 1);
    }
    return None;
}
