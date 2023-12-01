use crate::{Part, Solution};

pub mod part1;
pub mod part2;
pub mod shared;

pub fn get_input() -> Result<String, std::io::Error> {
    std::fs::read_to_string("src/day08/input/input.txt")
}

pub fn run(part: Part) -> Solution {
    let input = get_input().unwrap();
    match part {
        Part::One => part1::solve(input.as_str()).into(),
        Part::Two => part2::solve(input.as_str()).into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(run(Part::One), 1693_u32.into());
    }

    #[test]
    fn part2_works() {
        assert_eq!(run(Part::Two), 422059_u32.into());
    }
}
