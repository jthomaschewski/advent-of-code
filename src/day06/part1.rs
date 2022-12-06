use super::shared::find_marker;

pub fn solve(input: &str) -> usize {
    find_marker(input, 4).unwrap()
}
