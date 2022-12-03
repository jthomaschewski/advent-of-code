pub fn solve(input: &str) -> u32 {
    let groups = input.split("\n\n");

    groups.map(sum_food).max().unwrap()
}

fn sum_food(elf_str: &str) -> u32 {
    elf_str
        .lines()
        .filter_map(|food_str| food_str.parse::<u32>().ok())
        .sum()
}
