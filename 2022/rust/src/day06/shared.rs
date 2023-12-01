use std::collections::HashSet;

pub fn find_marker(input: &str, num_unique_chars: usize) -> Option<usize> {
    let marker_index = input
        .as_bytes()
        .windows(num_unique_chars)
        .position(|window| {
            // put all chars into a set to check if all n items are distinct
            window.iter().collect::<HashSet<_>>().len() == num_unique_chars
        });

    let marker_index = marker_index? + num_unique_chars;
    Some(marker_index)
}
