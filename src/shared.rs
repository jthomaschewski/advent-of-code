use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub enum Part {
    One = 1,
    Two = 2,
}

#[derive(Debug)]
pub enum Solution {
    Number(u32),
    SignedNumber(i32),
    String(String),
}

/// makes it possible to use "Solution" instance in print/format
impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::Number(n) => write!(f, "{}", n),
            Solution::SignedNumber(s) => write!(f, "{}", s),
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
impl From<String> for Solution {
    fn from(str: String) -> Self {
        Solution::String(str)
    }
}
impl From<u32> for Solution {
    fn from(n: u32) -> Self {
        Solution::Number(n)
    }
}
impl From<i32> for Solution {
    fn from(n: i32) -> Self {
        Solution::SignedNumber(n)
    }
}
impl From<usize> for Solution {
    fn from(n: usize) -> Self {
        Solution::Number(n as u32)
    }
}
// Support any Result<T, _> as Solution, for any T, which implements From<T> (see above, e.g. Result<u32, Err>)
impl<T, E> From<Result<T, E>> for Solution
where
    T: std::convert::Into<Solution>,
    E: std::fmt::Debug,
{
    fn from(n: Result<T, E>) -> Self {
        n.expect("Got error instead of solution...").into()
    }
}
// Support any Option<T> as Solution, for any T, which implements From<T> (see above, e.g. Option<u32>)
impl<T> From<Option<T>> for Solution
where
    T: std::convert::Into<Solution>,
{
    fn from(n: Option<T>) -> Self {
        n.expect("Got None instead of solution...").into()
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
