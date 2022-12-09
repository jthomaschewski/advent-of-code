use super::shared::{Matrix, Point};

pub fn solve(input: &str) -> usize {
    let matrix = input.parse::<Matrix>().unwrap();
    matrix.num_visible()
}

impl Matrix {
    fn num_visible(&self) -> usize {
        let (height, width) = self.dimensions();
        let mut n_visible = (width - 1) * 2 + (height - 1) * 2;

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let point = Point { x, y };
                if self.is_visible(&point) {
                    n_visible += 1;
                }
            }
        }
        n_visible
    }

    fn is_visible(&self, point: &Point) -> bool {
        let val = self.get(point);

        let blocked_left = self.left_from(point).any(|n| n >= val);
        let blocked_right = self.right_from(point).any(|n| n >= val);
        let blocked_top = self.top_from(point).any(|n| n >= val);
        let blocked_bottom = self.bottom_from(point).any(|n| n >= val);

        let blocked = blocked_left && blocked_right && blocked_top && blocked_bottom;
        !blocked
    }
}
