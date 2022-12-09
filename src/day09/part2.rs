use super::shared::{Move, Rope};

pub fn solve(input: &str) -> usize {
    let moves = input
        .lines()
        .flat_map(|line| line.parse::<Move>())
        .collect::<Vec<_>>();

    let mut rope = Rope::new(10);
    for m in moves {
        rope.apply_move(&m);
    }
    rope.num_visited()
}
