use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../inputs/day01.txt").expect("Failed to read input.txt");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let (l, r) = line.split_once(char::is_whitespace).unwrap();
        let l = l.parse().expect("unable to parse int");
        let r = r.trim().parse().expect("unable to parse int");

        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    for (i, l) in left.iter().enumerate() {
        let distance = (right[i] - *l).abs();
        sum += distance;
    }
    println!("result: {sum}")
}
