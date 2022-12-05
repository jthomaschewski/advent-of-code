use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub enum Part {
    One = 1,
    Two = 2,
}

#[derive(Debug)]
pub enum Solution {
    Number(u32),
    String(String),
}

/// makes it possible to use "Solution" instance in print/format
impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Number(n) => write!(f, "{}", n),
            Solution::String(s) => write!(f, "{}", s),
        }
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(n1), Self::Number(n2)) => n1 == n2,
            (Self::String(s1), Self::String(s2)) => s1 == s2,
            _ => false,
        }
    }
}

impl From<&str> for Solution {
    fn from(str: &str) -> Self {
        Solution::String(str.to_string())
    }
}
impl From<u32> for Solution {
    fn from(n: u32) -> Self {
        Solution::Number(n)
    }
}

/// makes it possible to use "Part" instance in print/format
impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_owned() as u8)
    }
}
impl Part {
    pub fn from_arg(value: Option<u8>) -> Vec<Part> {
        match value {
            Some(1) => vec![Part::One],
            Some(2) => vec![Part::Two],
            None => vec![Part::One, Part::Two],
            Some(value) => panic!("Invalid part nr {value}"),
        }
    }
}
