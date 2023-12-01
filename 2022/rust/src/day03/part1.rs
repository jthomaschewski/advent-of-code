pub fn solve(input: &str) -> u32 {
    let rucksacks = input.lines().map(|line| line.split_at(line.len() / 2));

    let mut prio_sum = 0;
    for (compartment_a, compartment_b) in rucksacks {
        let common_item = get_common_item(compartment_a, compartment_b);
        prio_sum += get_priority(common_item) as u32;
    }

    prio_sum
}

fn get_common_item(compartment_a: &str, compartment_b: &str) -> char {
    let common_char_index = compartment_a
        .find(|char| compartment_b.contains(char))
        .expect("no common char found");

    compartment_a.chars().nth(common_char_index).unwrap()
}

fn get_priority(item: char) -> u8 {
    let code: u8 = item.try_into().expect("Unexpected char");
    if code >= 97 {
        code - 96
    } else {
        code - 38
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_item() {
        assert_eq!(get_common_item("abco", "yubl"), 'b');
    }

    #[test]
    fn priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }
}
