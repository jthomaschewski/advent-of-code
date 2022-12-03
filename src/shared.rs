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
