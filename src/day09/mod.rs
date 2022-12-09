use crate::{Part, Solution};

pub mod part1;
pub mod part2;
pub mod shared;

pub fn get_input() -> Result<String, std::io::Error> {
    std::fs::read_to_string("src/day09/input/input.txt")
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
    fn part1_example_works() {
        let input = r#"R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2"#;
        assert_eq!(part1::solve(input), 13);
    }

    #[test]
    fn part1_works() {
        assert_eq!(run(Part::One), Solution::Number(6354));
    }

    #[test]
    fn part2_example_1_works() {
        let input = r#"R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2"#;
        assert_eq!(part2::solve(input), 1);
    }

    #[test]
    fn part2_example_2_works() {
        let input = r#"R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20"#;
        assert_eq!(part2::solve(input), 36);
    }

    #[test]
    fn part2_works() {
        assert_eq!(run(Part::Two), Solution::Number(2651));
    }
}
