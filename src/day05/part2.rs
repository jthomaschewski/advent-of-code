use super::shared::{Move, Stacks};
use crate::Solution;

pub fn solve(input: &str) -> Solution {
    if let [stacks_input, moves_input] = input.split("\n\n").collect::<Vec<&str>>()[..2] {
        let mut stacks: Stacks = stacks_input.into();
        let moves = Move::vec_from_input(moves_input);

        // stacks.print();
        process_moves(&moves, &mut stacks);
        // stacks.print();

        return stacks.get_top().into();
    }
    panic!("Invalid input");
}

fn process_moves(moves: &Vec<Move>, stacks: &mut Stacks) {
    for m in moves {
        // collecting moved items in a temporary vec first
        // this vec is then reversed and pushed to the target stack
        let mut tmp_stack = vec![];
        for _ in 0..m.amount {
            if let Some(item) = stacks[m.from - 1].pop() {
                tmp_stack.push(item);
            }
        }
        for item in tmp_stack.iter().rev() {
            stacks[m.to - 1].push(*item);
        }
    }
}
