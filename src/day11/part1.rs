use super::monkey::Monkey;

pub fn solve(input: &str) -> u64 {
    let mut puzzle = MonkeyPuzzle::new(input);
    puzzle.run(20);
    puzzle.get_monkey_business()
}

struct MonkeyPuzzle {
    monkeys: Vec<Monkey>,
}

impl MonkeyPuzzle {
    pub fn new(input: &str) -> Self {
        let monkeys: Vec<_> = input
            .split("\n\n")
            .map(|monkey| monkey.parse::<Monkey>().unwrap())
            .collect();
        Self { monkeys }
    }

    pub fn run(&mut self, rounds: u32) -> &Vec<Monkey> {
        for _ in 0..rounds {
            for i in 0..self.monkeys.len() {
                let monkey = &mut self.monkeys[i];
                let updated_items = monkey
                    .items
                    .drain(..)
                    .map(|item| monkey.operation.eval(item) / 3)
                    .collect::<Vec<_>>();
                monkey.items_inspected += updated_items.len() as u64;

                let Monkey {
                    divisor,
                    target_if_true: target_on_true,
                    target_if_false: target_on_false,
                    ..
                } = *monkey;

                for item in updated_items {
                    let target = if item % divisor == 0 {
                        target_on_true
                    } else {
                        target_on_false
                    };
                    self.monkeys[target].items.push(item);
                }
            }
        }
        &self.monkeys
    }

    pub fn get_monkey_business(&self) -> u64 {
        let mut inspections = self
            .monkeys
            .iter()
            .map(|monkey| monkey.items_inspected)
            .collect::<Vec<_>>();
        inspections.sort();
        inspections.iter().rev().take(2).product()
    }
}
