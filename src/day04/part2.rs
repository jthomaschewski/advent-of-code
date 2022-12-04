pub fn solve(input: &str) -> u32 {
    let overlaps = input
        .lines()
        .map(|line| line.split(&['-', ',']).flat_map(str::parse::<u32>))
        .map(|ranges| ranges.collect::<Vec<u32>>())
        .fold(0, |sum, ranges| {
            if let [r1_from, r1_to, r2_from, r2_to] = ranges[..] {
                let range2_overlaps_range1 = r1_from <= r2_from && r1_to >= r2_from;
                let range1_overlaps_range2 = r2_from <= r1_from && r2_to >= r1_from;

                if range2_overlaps_range1 || range1_overlaps_range2 {
                    return sum + 1;
                }
            }
            sum
        });
    overlaps
}
