use anyhow::{bail, Context};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum MathOperation {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
pub struct MathExpression {
    left: u64,
    right: u64,
    operation: MathOperation,
}

impl MathExpression {
    pub fn new(left: u64, right: u64, operation: MathOperation) -> Self {
        Self {
            left,
            right,
            operation,
        }
    }

    pub fn eval(&self, old: u64) -> u64 {
        let mut left = self.left;
        let mut right = self.right;
        if self.left == 0 {
            left = old;
        }
        if self.right == 0 {
            right = old;
        }
        match self.operation {
            MathOperation::Add => left + right,
            MathOperation::Multiply => left * right,
        }
    }
}

impl FromStr for MathExpression {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split_whitespace();

        let left_str = parts.next().context("invalid left expr")?;
        let operation = match parts.next() {
            Some("+") => MathOperation::Add,
            Some("*") => MathOperation::Multiply,
            _ => bail!("invalid operation"),
        };
        let right_str = parts.next().context("invalid right expr")?;

        let mut right: u64 = 0;
        let mut left: u64 = 0;

        if left_str != "old" {
            left = left_str.parse()?;
        }
        if right_str != "old" {
            right = right_str.parse()?;
        }

        Ok(MathExpression::new(left, right, operation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_add() {
        assert_eq!(MathExpression::new(12, 2, MathOperation::Add).eval(99), 14);
        assert_eq!(
            MathExpression::new(12, 2, MathOperation::Multiply).eval(99),
            24
        );

        assert_eq!(MathExpression::new(0, 2, MathOperation::Add).eval(10), 12);
        assert_eq!(MathExpression::new(2, 0, MathOperation::Add).eval(10), 12);
    }

    #[test]
    fn math_from_str() {
        assert_eq!("12 + 2".parse::<MathExpression>().unwrap().eval(99), 14);
        assert_eq!("12 * 2".parse::<MathExpression>().unwrap().eval(99), 24);

        assert_eq!("old * 2".parse::<MathExpression>().unwrap().eval(10), 20);
        assert_eq!("2 + old".parse::<MathExpression>().unwrap().eval(10), 12);
    }
}
