pub fn solve(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let group_chunks = lines.chunks(3);

    let mut prio_sum = 0;

    for chunks in group_chunks {
        let common_item = get_common_item(chunks);
        prio_sum += get_priority(common_item) as u32;
    }

    prio_sum
}

fn get_common_item(groups: &[&str]) -> char {
    for char in groups[0].chars() {
        let mut found_in_nr = 1;

        for group in &groups[1..] {
            for c in group.chars() {
                if c == char {
                    found_in_nr += 1;
                    break;
                }
            }
        }
        // dbg!(char, found_in_nr, groups);
        if found_in_nr == groups.len() {
            return char;
        }
    }
    panic!("no common item found");
}

fn get_priority(item: char) -> u8 {
    let char_ord: u8 = item.try_into().expect("Unexpected char");
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
        assert_eq!(get_common_item(&["abco", "yubl"]), 'b');
        assert_eq!(get_common_item(&["abac", "dfgc", "yuic"]), 'c');
        assert_eq!(get_common_item(&["abaac", "daafgc", "yuic"]), 'c');
    }

    #[test]
    fn priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }
}
