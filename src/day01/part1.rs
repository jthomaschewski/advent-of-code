pub fn solve() -> u32 {
    let sums = get_elf_sums();
    sums.iter().max().unwrap().clone()
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
