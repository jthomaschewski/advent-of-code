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

const VALID_STR: [&str; 2] = ["XMAS", "SAMX"];

fn check_from_coord(input: &Vec<Vec<char>>, cords: (usize, usize)) -> u32 {
    let mut matches = 0;
    let line = &input[cords.0];

    // check horizontal
    if line.len() >= cords.1 + 4 {
        let slice = &line[cords.1..cords.1 + 4].iter().collect::<String>();
        if VALID_STR.contains(&slice.as_str()) {
            matches += 1;
        }
    }
    // check vertical
    if input.len() >= cords.0 + 4 {
        let slice = &input[cords.0..cords.0 + 4]
            .iter()
            .map(|row| row[cords.1])
            .collect::<String>();
        if VALID_STR.contains(&slice.as_str()) {
            matches += 1;
        }
    }
    // check diagonal down-left
    if input.len() >= cords.0 + 4 && line.len() >= cords.1 + 4 {
        let slice = &input[cords.0..cords.0 + 4]
            .iter()
            .enumerate()
            .map(|(i, row)| row[cords.1 + i])
            .collect::<String>();
        if VALID_STR.contains(&slice.as_str()) {
            matches += 1;
        }
    }
    // check diagnoal down-right
    if input.len() >= cords.0 + 4 && cords.1 >= 3 {
        let slice = &input[cords.0..cords.0 + 4]
            .iter()
            .enumerate()
            .map(|(i, row)| row[cords.1 - i])
            .collect::<String>();
        if VALID_STR.contains(&slice.as_str()) {
            matches += 1;
        }
    }

    return matches;
}
