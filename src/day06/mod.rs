use crate::{Part, Solution};

pub mod part1;
pub mod part2;
pub mod shared;

pub fn get_input() -> Result<String, std::io::Error> {
    std::fs::read_to_string("src/day06/input/input.txt")
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
        assert_eq!(part1::solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1::solve("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

        assert_eq!(run(Part::One), 1155u32.into());
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2::solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2::solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2::solve("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2::solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2::solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);

        assert_eq!(run(Part::Two), 2789u32.into());
    }
}
