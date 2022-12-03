pub fn solve() -> u32 {
    let mut sums = get_elf_sums();
    sums.sort();
    sums.iter().rev().take(3).sum()
}

fn get_elf_sums() -> Vec<u32> {
    let str = include_str!("./input/input.txt");

    str.split("\n\n").map(get_elf_food_sum).collect()
}

fn get_elf_food_sum(elf_str: &str) -> u32 {
    elf_str
        .lines()
        .filter_map(|food_str| food_str.parse::<u32>().ok())
        .sum()
}
