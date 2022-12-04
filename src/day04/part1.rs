pub fn solve(input: &str) -> u32 {
    let pairs = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .flat_map(str::parse::<u32>)
                        .collect::<Vec<u32>>()
                })
                .map(|range| (range[0], range[1]))
                .collect::<Vec<(u32, u32)>>()
        })
        .map(|pair| (pair[0], pair[1]));

    let overlaps = pairs.fold(0, |sum, pair| {
        let (range1, range2) = pair;

        let range2_contains_range1 = range1.0 <= range2.0 && range1.1 >= range2.1;
        let range1_contains_range2 = range2.0 <= range1.0 && range2.1 >= range1.1;

        if range2_contains_range1 || range1_contains_range2 {
            return sum + 1;
        }
        sum
    });

    overlaps
}
