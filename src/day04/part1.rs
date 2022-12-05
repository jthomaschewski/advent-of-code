use crate::Solution;

pub fn solve(input: &str) -> Solution {
    let overlaps = input
        .lines()
        .map(|line| line.split(&['-', ',']).flat_map(str::parse::<u32>))
        .map(|ranges| ranges.collect::<Vec<u32>>())
        .fold(0, |sum, ranges| {
            if let [r1_from, r1_to, r2_from, r2_to] = ranges[..] {
                let range2_overlaps_range1 = r1_from <= r2_from && r1_to >= r2_to;
                let range1_overlaps_range2 = r2_from <= r1_from && r2_to >= r1_to;

                if range2_overlaps_range1 || range1_overlaps_range2 {
                    return sum + 1;
                }
            }
            sum
        });
    overlaps.into()
}
