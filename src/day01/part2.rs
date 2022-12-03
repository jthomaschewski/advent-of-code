pub fn solve(input: &str) -> u32 {
    let groups = input.split("\n\n");

    let mut group_sums = groups.map(sum_food).collect::<Vec<u32>>();
    group_sums.sort();

    let top_3_sum = group_sums.iter().rev().take(3).sum();
    top_3_sum
}

fn sum_food(elf_str: &str) -> u32 {
    elf_str
        .lines()
        .filter_map(|food_str| food_str.parse::<u32>().ok())
        .sum()
}
