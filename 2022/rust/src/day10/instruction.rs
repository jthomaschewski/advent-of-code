use anyhow::{bail, Context};
use std::str::FromStr;

pub trait Instruction {
    fn run(&mut self, cycle: usize, x: &mut isize);
    fn is_finished(&self) -> bool;
}

struct AddxInstruction {
    value: isize,
    finished: bool,
}
struct NoopInstruction {}

impl AddxInstruction {
    fn new(value: isize) -> Self {
        Self {
            value,
            finished: false,
        }
    }
}

impl Instruction for AddxInstruction {
    fn run(&mut self, cycle: usize, x: &mut isize) {
        // do nothing in first, finish in second cycle
        if cycle == 1 {
            *x += self.value;
            self.finished = true;
        }
    }

    fn is_finished(&self) -> bool {
        self.finished
    }
}

impl Instruction for NoopInstruction {
    fn run(&mut self, _: usize, _: &mut isize) {}
    fn is_finished(&self) -> bool {
        true
    }
}

impl FromStr for Box<dyn Instruction> {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.split_whitespace();
        let instruction = parts.next().context("no instruction")?;
        match instruction {
            "addx" => {
                let value = parts.next().context("no value")?.parse::<isize>()?;
                Ok(Box::new(AddxInstruction::new(value)))
            }
            "noop" => Ok(Box::new(NoopInstruction {})),
            _ => bail!("Unknown instruction: {}", line),
        }
    }
}
