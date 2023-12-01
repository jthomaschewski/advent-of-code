use super::shared::{Matrix, Point};

pub fn solve(input: &str) -> Option<usize> {
    let matrix = input.parse::<Matrix>().ok()?;
    Some(matrix.highest_scenic_score())
}

impl Matrix {
    fn highest_scenic_score(&self) -> usize {
        let (height, width) = self.dimensions();
        let mut highest_scenic = 0;

        for y in 0..height {
            for x in 0..width {
                let point = Point { x, y };
                let scenic_score = self.scenic_score(&point);
                if scenic_score > highest_scenic {
                    highest_scenic = scenic_score;
                }
            }
        }
        highest_scenic
    }

    fn scenic_score(&self, point: &Point) -> usize {
        let val = &self.get(point);

        let visible_left = Matrix::count_visible(val, self.left_from(point));
        let visible_right = Matrix::count_visible(val, self.right_from(point));
        let visible_top = Matrix::count_visible(val, self.top_from(point));
        let visible_bottom = Matrix::count_visible(val, self.bottom_from(point));

        visible_left * visible_right * visible_top * visible_bottom
    }

    fn count_visible(val: &u32, iter: impl Iterator<Item = u32>) -> usize {
        let mut n_trees = 0;
        for n in iter {
            n_trees += 1;
            if n >= *val {
                break;
            }
        }
        n_trees
    }
}
