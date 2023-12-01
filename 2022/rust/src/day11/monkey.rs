use anyhow::Context;
use std::str::FromStr;

use super::math::MathExpression;

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: Vec<u64>,
    pub items_inspected: u64,
    pub operation: MathExpression,
    pub divisor: u64,
    pub target_if_true: usize,
    pub target_if_false: usize,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkey = Monkey {
            items: vec![],
            operation: "0 + 0".parse::<MathExpression>()?,
            divisor: 0,
            target_if_true: 0,
            target_if_false: 0,
            items_inspected: 0,
        };

        for line in s.trim().lines() {
            let line = line.trim();
            if line.starts_with("Starting items") {
                monkey.items = line
                    .split_whitespace()
                    .skip(2)
                    .flat_map(|item| item.replace(',', "").parse())
                    .collect();
            } else if line.starts_with("Operation") {
                monkey.operation = line
                    .split(" = ")
                    .nth(1)
                    .context("missing operation")?
                    .parse::<MathExpression>()?;
            } else if line.starts_with("Test:") {
                monkey.divisor = line.split(" by ").nth(1).context("invalid test")?.parse()?;
            } else if line.starts_with("If true") {
                monkey.target_if_true = line
                    .split("throw to monkey ")
                    .nth(1)
                    .context("invalid target")?
                    .parse::<usize>()?;
            } else if line.starts_with("If false") {
                monkey.target_if_false = line
                    .split("throw to monkey ")
                    .nth(1)
                    .context("invalid target")?
                    .parse::<usize>()?;
            }
        }

        Ok(monkey)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monkey_from_str() {
        let input = r#"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3
        "#;

        let monkey = input.parse::<Monkey>().unwrap();
        assert_eq!(monkey.items, vec![79, 98]);
        assert_eq!(monkey.operation.eval(2), 38);
        assert_eq!(monkey.divisor, 23);
        assert_eq!(monkey.target_if_true, 2);
        assert_eq!(monkey.target_if_false, 3);
    }
}
