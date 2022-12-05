use crate::Solution;

const SCORE_LOST: u32 = 0;
const SCORE_DRAW: u32 = 3;
const SCORE_WIN: u32 = 6;

const SCORE_ROCK: u32 = 1;
const SCORE_PAPER: u32 = 2;
const SCORE_SCISSORS: u32 = 3;

pub fn solve(input: &str) -> Solution {
    Solution::Number(input.lines().map(get_score).sum())
}

fn get_score(line: &str) -> u32 {
    match line {
        // A => rock B => paper C => scissors
        // X => loose Y => draw Z => win
        "A X" => SCORE_LOST + SCORE_SCISSORS,
        "A Y" => SCORE_DRAW + SCORE_ROCK,
        "A Z" => SCORE_WIN + SCORE_PAPER,
        "B X" => SCORE_LOST + SCORE_ROCK,
        "B Y" => SCORE_DRAW + SCORE_PAPER,
        "B Z" => SCORE_WIN + SCORE_SCISSORS,
        "C X" => SCORE_LOST + SCORE_PAPER,
        "C Y" => SCORE_DRAW + SCORE_SCISSORS,
        "C Z" => SCORE_WIN + SCORE_ROCK,
        _ => panic!("unexpected input line: {line}"),
    }
}
