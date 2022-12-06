use std::collections::HashSet;

pub fn find_marker(input: &str, num_unique_chars: usize) -> Option<usize> {
    let vec = input.chars().enumerate().collect::<Vec<(usize, char)>>();
    let result_window = vec.windows(num_unique_chars).find(|window| {
        // put all chars into a hash map to check for uniqueness
        let hs: HashSet<&char> = HashSet::from_iter(window.iter().map(|(_, c)| c));
        // expect all chars to be unique to be a marker set
        hs.len() == num_unique_chars
    });

    match result_window {
        Some(window) => {
            let marker_index = window.last().unwrap().0;
            Some(marker_index + 1)
        }
        None => None,
    }
}
