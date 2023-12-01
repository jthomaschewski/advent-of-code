const SCORE_LOST: u32 = 0;
const SCORE_DRAW: u32 = 3;
const SCORE_WIN: u32 = 6;

const SCORE_ROCK: u32 = 1;
const SCORE_PAPER: u32 = 2;
const SCORE_SCISSORS: u32 = 3;

pub fn solve(input: &str) -> u32 {
    input.lines().map(get_score).sum()
}

fn get_score(line: &str) -> u32 {
    match line {
        // A/X => rock B/Y => paper C/Z => scissors
        "A X" => SCORE_DRAW + SCORE_ROCK,
        "B X" => SCORE_LOST + SCORE_ROCK,
        "C X" => SCORE_WIN + SCORE_ROCK,
        "A Y" => SCORE_WIN + SCORE_PAPER,
        "B Y" => SCORE_DRAW + SCORE_PAPER,
        "C Y" => SCORE_LOST + SCORE_PAPER,
        "A Z" => SCORE_LOST + SCORE_SCISSORS,
        "B Z" => SCORE_WIN + SCORE_SCISSORS,
        "C Z" => SCORE_DRAW + SCORE_SCISSORS,
        _ => panic!("unexpected input line: {line}"),
    }
}
