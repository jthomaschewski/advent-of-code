use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../inputs/day04.txt").expect("Failed to read input");
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;
    let line_len = input[0].len();

    for i in 0..input.len() {
        for j in 0..line_len {
            let result = check_from_coord(&input, (i, j));
            sum += result;
        }
    }

    println!("{:?}", sum);
}

const VALID_STR: [&str; 2] = ["MAS", "SAM"];

fn check_from_coord(input: &Vec<Vec<char>>, cords: (usize, usize)) -> u32 {
    let mut matches = 0;

    if input.len() >= cords.0 + 3 && cords.1 >= 2 {
        let (downright, downleft) = &input[cords.0..cords.0 + 3]
            .iter()
            .enumerate()
            .map(|(i, row)| (row[cords.1 - i], row[cords.1 - 2 + i]))
            .collect::<(String, String)>();

        if VALID_STR.contains(&downleft.as_str()) && VALID_STR.contains(&downright.as_str()) {
            matches += 1;
        }
    }

    return matches;
}
