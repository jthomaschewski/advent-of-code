use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    str::FromStr,
};

fn main() {
    let input = read_to_string("../inputs/day05.txt").expect("Failed to read input");
    let manual: Manual = input.parse().expect("Unable to parse manual");

    println!("valid: {:?}\n", manual.valid_updates());
    println!("sum: {:?}", manual.get_middle_sum());
}

type Rules = HashMap<i32, Vec<i32>>;
type Update = Vec<i32>;

#[derive(Debug)]
struct Manual {
    rules: Rules,
    updates: Vec<Update>,
}

impl Manual {
    fn get_middle_sum(&self) -> i32 {
        return self
            .valid_updates()
            .iter()
            .map(|update| update[update.len() / 2])
            .sum();
    }

    fn valid_updates(&self) -> Vec<&Update> {
        let valid_updates: Vec<&Update> = self
            .updates
            .iter()
            .filter(|update| {
                let mut checked: HashSet<&i32> = HashSet::new();
                for page in update.iter() {
                    if let Some(rule) = self.rules.get(page) {
                        for prev_page in &checked {
                            if rule.contains(prev_page) {
                                return false;
                            }
                        }
                    }
                    checked.insert(page);
                }
                return true;
            })
            .collect();
        return valid_updates;
    }
}

impl FromStr for Manual {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (rule_str, updates) = input.split_once("\n\n").expect("invalid input");

        let mut rules = Rules::new();
        rule_str
            .lines()
            .map(|line| line.split_once('|').expect("invalid rule"))
            .for_each(|(l, r)| {
                let l: i32 = l.parse().expect("invalid num");
                let r: i32 = r.parse().expect("invalid num");

                if let Some(rule) = rules.get(&l) {
                    let mut new_rule = rule.clone();
                    new_rule.push(r);
                    rules.insert(l, new_rule);
                } else {
                    rules.insert(l, vec![r]);
                }
            });

        let updates: Vec<Update> = updates
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|c| c.parse().expect("invalid update item"))
                    .collect()
            })
            .collect();

        Ok(Manual { rules, updates })
    }
}
