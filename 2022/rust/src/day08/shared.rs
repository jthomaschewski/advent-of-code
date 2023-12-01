use std::{fmt::Display, ops::Deref, str::FromStr};

pub struct Matrix(Vec<Vec<u32>>);

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Matrix {
    pub fn dimensions(&self) -> (usize, usize) {
        (self.len(), self[0].len())
    }

    pub fn get(&self, point: &Point) -> u32 {
        self[point.y][point.x]
    }

    pub fn left_from(&self, point: &Point) -> impl Iterator<Item = u32> + '_ {
        let Point { x, y } = point;
        self[*y].iter().take(*x).copied().rev()
    }

    pub fn right_from(&self, point: &Point) -> impl Iterator<Item = u32> + '_ {
        let Point { x, y } = point;
        self[*y].iter().skip(x + 1).copied()
    }

    pub fn top_from(&self, point: &Point) -> impl Iterator<Item = u32> + '_ {
        let Point { x, y } = point;
        self.col(*x).take(*y).rev()
    }

    pub fn bottom_from(&self, point: &Point) -> impl Iterator<Item = u32> + '_ {
        let Point { x, y } = point;
        self.col(*x).skip(y + 1)
    }

    pub fn col(&self, x: usize) -> impl DoubleEndedIterator<Item = u32> + ExactSizeIterator + '_ {
        self.iter().map(move |row| row[x])
    }
}

impl FromStr for Matrix {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let matrix = input
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|c| c.to_digit(10))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(Matrix(matrix))
    }
}

impl Deref for Matrix {
    type Target = Vec<Vec<u32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter() {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}
