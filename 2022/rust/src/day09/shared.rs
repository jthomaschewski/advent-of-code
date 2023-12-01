use std::{collections::HashSet, str::FromStr};

use anyhow::{bail, Context, Result};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Move {
    pub direction: Direction,
    pub distance: usize,
}

type Pos = (i32, i32);

#[derive(Debug)]
pub struct Rope {
    knots: Vec<Pos>,
    pub visited: HashSet<Pos>,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut split = line.split_whitespace();

        let direction = match split.next() {
            Some("U") => Direction::Up,
            Some("D") => Direction::Down,
            Some("L") => Direction::Left,
            Some("R") => Direction::Right,
            _ => bail!("invalid direction line {}", line),
        };
        let distance = split.next().context("distance missing")?.parse::<usize>()?;

        Ok(Move {
            direction,
            distance,
        })
    }
}

impl Rope {
    pub fn new(size: usize) -> Self {
        Rope {
            knots: vec![(0, 0); size],
            visited: [(0, 0)].into(),
        }
    }

    pub fn num_visited(&self) -> usize {
        self.visited.len()
    }

    pub fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.distance {
            self.move_to(&m.direction);
        }
    }

    fn move_to(&mut self, direction: &Direction) {
        // move head
        let (head_x, head_y) = &mut self.knots[0];
        match direction {
            Direction::Up => *head_y += 1,
            Direction::Down => *head_y -= 1,
            Direction::Left => *head_x -= 1,
            Direction::Right => *head_x += 1,
        }

        // check if knots need to move
        for i in 1..self.knots.len() {
            let (next_x, next_y) = self.knots[i - 1];
            let (cur_x, cur_y) = &mut self.knots[i];

            let distance_x = *cur_x - next_x;
            let distance_y = *cur_y - next_y;

            // in same row => move in x-axis
            if distance_x.abs() == 2 && distance_y.abs() == 0 {
                // needs move in x-axis
                *cur_x -= distance_x.signum();
            }
            // in same col => move in y-axis
            else if distance_y.abs() == 2 && distance_x.abs() == 0 {
                *cur_y -= distance_y.signum();
            }
            // diagonal => move in both axis
            else if (distance_x.abs() == 2 && distance_y.abs() >= 1)
                || (distance_x.abs() >= 1 && distance_y.abs() == 2)
            {
                *cur_x -= distance_x.signum();
                *cur_y -= distance_y.signum();
            }
        }
        // store visited num of tail
        let tail = self.knots.last().unwrap();
        self.visited.insert(*tail);
    }
}
