use crate::Solution;

pub fn solve(input: &str) -> Solution {
    let lines = input.lines().collect::<Vec<_>>();
    let group_chunks = lines.chunks(3);

    let mut prio_sum = 0;

    for chunks in group_chunks {
        let common_item = get_common_item(chunks).unwrap();
        prio_sum += get_priority(common_item) as u32;
    }

    prio_sum.into()
}

fn get_common_item(groups: &[&str]) -> Option<char> {
    let first_group = &groups[0];
    let other_groups = &groups[1..];

    first_group.chars().find(|item| {
        other_groups
            .iter()
            .all(|group| group.chars().any(|other_item| other_item == *item))
    })
}

fn get_priority(item: char) -> u32 {
    let char_ord: u32 = item.try_into().expect("Unexpected char");
    if char_ord >= 97 {
        char_ord - 96
    } else {
        char_ord - 38
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_item() {
        assert_eq!(get_common_item(&["abco", "yubl"]), Some('b'));
        assert_eq!(get_common_item(&["abac", "dfgc", "yuic"]), Some('c'));
        assert_eq!(get_common_item(&["abaac", "daafgc", "yuic"]), Some('c'));
    }

    #[test]
    fn priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }
}
