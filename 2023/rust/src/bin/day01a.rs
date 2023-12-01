use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../inputs/day01.txt").expect("Failed to read input.txt");

    let sum: u32 = input
        .lines()
        .map(|line| str::chars(&line).collect::<Vec<char>>())
        .map(|line| {
            let first_digit = line
                .iter()
                .find(|c| char::is_ascii_digit(&c))
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last_digit = &line
                .iter()
                .rfind(|c| char::is_ascii_digit(&c))
                .unwrap()
                .to_digit(10)
                .unwrap();
            return first_digit * 10 + last_digit;
        })
        .sum();

    println!("{}", sum)
}
