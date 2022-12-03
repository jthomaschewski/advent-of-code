use crate::Part;

pub mod part1;
pub mod part2;

pub fn get_input() -> &'static str {
    include_str!("./input.txt")
}

pub fn run(part: Part) -> u32 {
    let input = get_input();
    match part {
        Part::One => part1::solve(input),
        Part::Two => part2::solve(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(run(Part::One), 11666);
    }

    #[test]
    fn part2_works() {
        assert_eq!(run(Part::Two), 12767);
    }
}
