use std::collections::VecDeque;

use super::instruction::Instruction;

pub fn solve(input: &str) -> String {
    let instructions = input
        .lines()
        .flat_map(str::parse::<Box<dyn Instruction>>)
        .collect::<VecDeque<_>>();

    let mut circuit = circuit::Circuit::new(instructions);
    circuit.run_instructions();

    circuit.get_crt_output()
}

mod circuit {
    use std::collections::VecDeque;

    use super::super::instruction::Instruction;

    pub struct Circuit {
        register: VecDeque<Box<dyn Instruction>>,
        x: isize,

        cur_instruction: Box<dyn Instruction>,
        cur_instruction_cycle: usize,

        crt: [bool; 240],
    }

    impl Circuit {
        pub fn new(mut instructions: VecDeque<Box<dyn Instruction>>) -> Self {
            let first_instruction = instructions.pop_front().unwrap();
            Self {
                register: instructions,
                x: 1,
                cur_instruction: first_instruction,
                cur_instruction_cycle: 0,

                crt: [false; 240],
            }
        }

        pub fn get_crt_output(&mut self) -> String {
            let mut output = String::new();
            for (i, &crt) in self.crt.iter().enumerate() {
                if i % 40 == 0 {
                    output.push('\n');
                }
                if crt {
                    output.push('#');
                } else {
                    output.push('.');
                }
            }
            output
        }

        pub fn run_instructions(&mut self) {
            let mut cycle = 0;
            loop {
                let register_empty = self.cycle(cycle).is_none();
                if register_empty {
                    break;
                }
                cycle += 1;
            }
        }

        fn cycle(&mut self, cycle: usize) -> Option<()> {
            self.draw(cycle);

            if self.cur_instruction.is_finished() {
                self.cur_instruction = self.register.pop_front()?;
                self.cur_instruction_cycle = 0;
            }

            self.cur_instruction
                .run(self.cur_instruction_cycle, &mut self.x);
            self.cur_instruction_cycle += 1;

            Some(())
        }

        fn draw(&mut self, cycle: usize) {
            let sprite_range = self.x - 1..self.x + 2;
            if sprite_range.contains(&((cycle as isize) % 40)) {
                self.crt[cycle] = true;
            }
        }
    }
}
