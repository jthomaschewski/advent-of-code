use crate::Part;

pub mod part1;
pub mod part2;

pub fn run(part: Part) -> u32 {
    match part {
        Part::One => part1::solve(),
        Part::Two => part2::solve(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(run(Part::One), 66306);
    }

    #[test]
    fn part2_works() {
        assert_eq!(run(Part::Two), 195292);
    }
}
