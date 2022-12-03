use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub enum Part {
    One = 1,
    Two = 2,
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
