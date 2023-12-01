use crate::{Part, Solution};

pub mod math;
pub mod monkey;
pub mod part1;
pub mod part2;

pub fn get_input() -> Result<String, std::io::Error> {
    std::fs::read_to_string("src/day11/input/input.txt")
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
    use crate::Part;

    #[test]
    fn part1_works() {
        assert_eq!(run(Part::One), 54054_u64.into());
    }

    #[test]
    fn part2_works() {
        assert_eq!(run(Part::Two), 14314925001_u64.into());
    }
}
