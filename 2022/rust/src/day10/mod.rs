use crate::{Part, Solution};

pub mod instruction;
pub mod part1;
pub mod part2;

pub fn get_input() -> Result<String, std::io::Error> {
    std::fs::read_to_string("src/day10/input/input.txt")
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
        assert_eq!(run(Part::One), 14780.into());
    }

    #[test]
    fn part2_works() {
        // answer is visible in image: ELPLZGZL
        let exp_image = r#"
            ####.#....###..#....####..##..####.#....
            #....#....#..#.#.......#.#..#....#.#....
            ###..#....#..#.#......#..#......#..#....
            #....#....###..#.....#...#.##..#...#....
            #....#....#....#....#....#..#.#....#....
            ####.####.#....####.####..###.####.####."#
            .replace(' ', "");

        assert_eq!(run(Part::Two), exp_image.into());
    }
}
