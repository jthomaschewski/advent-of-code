#![allow(dead_code)]
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Move {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}
pub struct Stacks(Vec<Vec<char>>);

impl Move {
    pub fn vec_from_input(input: &str) -> Vec<Move> {
        input.lines().map(|move_line| move_line.into()).collect()
    }
}

impl From<&str> for Move {
    /// Converts a string into a Move struct
    /// Can be used like this: let m: Move = "move 1 from 1 to 2".into()
    fn from(input_line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let captures = RE.captures(input_line).unwrap();
        Move {
            amount: captures[1].parse::<usize>().unwrap(),
            from: captures[2].parse::<usize>().unwrap(),
            to: captures[3].parse::<usize>().unwrap(),
        }
    }
}

impl From<&str> for Stacks {
    /// Converts a string into a Stacks struct
    /// Can be used like this: let s: Stacks = "[1] [2] ...".into()
    fn from(input_line: &str) -> Self {
        let num_stacks = (input_line.lines().next().unwrap().len() + 1) / 4;

        let mut stacks = vec![vec![]; num_stacks];

        input_line.lines().rev().skip(1).for_each(|line| {
            line.chars()
                .enumerate()
                .filter(|&(i, _)| (i + 3) % 4 == 0)
                .filter(|&(_, item)| item != ' ')
                .for_each(|(index, item)| {
                    let target_stack_idx: usize = index / 4;
                    stacks[target_stack_idx].push(item);
                })
        });

        Stacks(stacks)
    }
}

impl Stacks {
    /// Print visualization of the stacks to stdout
    pub fn print(&self) {
        self.iter().enumerate().for_each(|(index, stack)| {
            println!("stack #{}: {:?}", index, stack);
        });
    }

    pub fn get_top(&self) -> String {
        self.iter()
            .flat_map(|stack| stack.last())
            .fold(String::new(), |mut str, item| {
                str.push(*item);
                str
            })
    }
}

impl Deref for Stacks {
    type Target = Vec<Vec<char>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
