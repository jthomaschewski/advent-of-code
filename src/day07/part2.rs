use super::shared::get_dirsizes;

pub fn solve(input: &str) -> Option<u32> {
    let dir_sizes = get_dirsizes(input);

    let total_space: u32 = 70_000_000;
    let required_space: u32 = 30_000_000;

    let space_used = total_space - (dir_sizes.get("/").unwrap_or(&0));
    let need_to_free = required_space - space_used;

    let smallest_candidate = dir_sizes
        .values()
        .filter(|&&size| size >= need_to_free)
        .min()?;

    Some(*smallest_candidate)
}
