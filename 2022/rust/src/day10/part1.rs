use std::collections::VecDeque;

use super::instruction::Instruction;

pub fn solve(input: &str) -> isize {
    let instructions = input
        .lines()
        .flat_map(str::parse::<Box<dyn Instruction>>)
        .collect::<VecDeque<_>>();
    let circuit = Circuit::new(instructions);

    let sum_cycles: [usize; 6] = [20, 60, 100, 140, 180, 220];

    circuit
        .enumerate()
        .filter(|&(cycle, _)| sum_cycles.contains(&(cycle + 1)))
        .map(|(cycle, x)| x * (cycle + 1) as isize)
        .sum()
}

struct Circuit {
    register: VecDeque<Box<dyn Instruction>>,
    x: isize,

    cur_instruction: Box<dyn Instruction>,
    cur_instruction_cycle: usize,
}

impl Circuit {
    fn new(mut instructions: VecDeque<Box<dyn Instruction>>) -> Self {
        let first_instruction = instructions.pop_front().unwrap();
        Self {
            register: instructions,
            x: 1,
            cur_instruction: first_instruction,
            cur_instruction_cycle: 0,
        }
    }
}

impl Iterator for Circuit {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        let res = self.x;

        if self.cur_instruction.is_finished() {
            self.cur_instruction = self.register.pop_front()?;
            self.cur_instruction_cycle = 0;
        }

        self.cur_instruction
            .run(self.cur_instruction_cycle, &mut self.x);
        self.cur_instruction_cycle += 1;

        Some(res)
    }
}
