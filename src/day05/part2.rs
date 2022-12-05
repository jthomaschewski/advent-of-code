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
        // split off the top "move.amount" items from the "move.from" stack
        // then push those onto the "move.to" stack
        let from_stack = &mut stacks[m.from - 1];
        let drained_items = from_stack.split_off(from_stack.len() - m.amount);

        let to_stack = &mut stacks[m.to - 1];
        to_stack.extend(drained_items);
    }
}
