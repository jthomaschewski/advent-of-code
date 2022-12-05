use crate::Solution;

pub fn solve(input: &str) -> Solution {
    let groups = input.split("\n\n");

    let mut group_sums = groups.map(sum_food).collect::<Vec<u32>>();
    group_sums.sort();

    let top_3_sum = group_sums.iter().rev().take(3).sum();
    Solution::Number(top_3_sum)
}

fn sum_food(elf_str: &str) -> u32 {
    elf_str.lines().flat_map(str::parse::<u32>).sum()
}
