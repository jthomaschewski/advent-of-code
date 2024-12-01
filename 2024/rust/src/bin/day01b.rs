use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("../inputs/day01.txt").expect("Failed to read input.txt");

    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let (l, r) = line.split_once(char::is_whitespace).unwrap();
        let l = l.parse().expect("unable to parse int");
        let r = r.trim().parse().expect("unable to parse int");

        if let Some(v) = right.get(&r) {
            right.insert(r, v + 1);
        } else {
            right.insert(r, 1);
        }
        left.push(l);
    }

    let mut sum = 0;

    for l in left.iter() {
        if let Some(r) = right.get(&l) {
            sum += r * l;
        }
    }
    println!("result: {sum}")
}
