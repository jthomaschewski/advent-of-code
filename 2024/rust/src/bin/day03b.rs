use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../inputs/day03.txt").expect("Failed to read input.txt");

    let re = Regex::new(r"(?P<mul>mul\((\d+),(\d+)\))|(?P<do>do\(\))|(?P<dont>don\'t\(\))")
        .expect("Invalid regex");

    let mut do_mul = true;
    let sum: i32 = re
        .captures_iter(&input)
        .map(|capture| {
            if capture.name("mul").is_some() {
                if !do_mul {
                    return 0;
                }
                let left = capture.get(2).expect("invalid mul").as_str();
                let left: i32 = left.parse().expect("invalid mul parse");
                let right = capture.get(3).expect("invalid mul").as_str();
                let right: i32 = right.parse().expect("invalid mul parse");

                if do_mul {
                    return left * right;
                }
                println!("{:?}", (left, right));
            } else if capture.name("do").is_some() {
                do_mul = true;
            } else if capture.name("dont").is_some() {
                do_mul = false;
            }
            return 0;
        })
        .sum();

    println!("{:?}", sum);
}
