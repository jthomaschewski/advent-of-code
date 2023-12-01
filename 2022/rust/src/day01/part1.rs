use crate::Solution;

pub fn solve(input: &str) -> Solution {
    let groups = input.split("\n\n");

    groups.map(sum_food).max().unwrap().into()
}

fn sum_food(elf_str: &str) -> u32 {
    elf_str.lines().flat_map(str::parse::<u32>).sum()
}
