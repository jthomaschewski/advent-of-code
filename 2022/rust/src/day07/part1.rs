use super::shared::get_dirsizes;

pub fn solve(input: &str) -> u32 {
    let dir_sizes = get_dirsizes(input);

    dir_sizes.values().filter(|&&size| size <= 100_000).sum()
}
