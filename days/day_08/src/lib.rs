#[must_use]
pub fn calc_a(input: &str) -> String {
    let matrix = Matrix::from(input);
    matrix.visible_count().to_string()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    let matrix = Matrix::from(input);
    matrix.max_scenic_score().to_string()
}

struct Matrix(Box<[Box<[u32]>]>);

impl Matrix {
    fn visible_count(&self) -> usize {
        let (rows, cols) = self.size();
        let mut count = 0;
        for row in 0..rows {
            for col in 0..cols {
                let v = self.0[row][col];
                count += usize::from(
                    row == 0
                        || col == 0
                        || row == rows - 1
                        || col == cols - 1
                        || self.row_slice_is_less(row, v, 0, col)
                        || self.row_slice_is_less(row, v, col + 1, cols)
                        || self.get_col_iter(col, v, 0, row)
                        || self.get_col_iter(col, v, row + 1, rows),
                );
            }
        }
        count
    }

    fn max_scenic_score(&self) -> usize {
        let (rows, cols) = self.size();
        let mut max_score = 0;
        for row in 0..rows {
            for col in 0..cols {
                if row == 0 || col == 0 || row == rows - 1 || col == cols - 1 {
                    continue;
                }

                let v = self.0[row][col];
                let mut score = 1;
                score *= self.row_reach(row, v, 0, col, true);
                score *= self.row_reach(row, v, col + 1, cols, false);
                score *= self.col_reach(col, v, 0, row, true);
                score *= self.col_reach(col, v, row + 1, rows, false);

                if score > max_score {
                    max_score = score;
                }
            }
        }
        max_score
    }

    fn size(&self) -> (usize, usize) {
        (self.0.len(), self.0.first().map_or(0, |row| row.len()))
    }

    fn row_slice_is_less(&self, row: usize, value: u32, start: usize, end: usize) -> bool {
        self.0[row][start..end].iter().all(|&x| x < value)
    }

    fn row_reach(&self, row: usize, value: u32, start: usize, end: usize, reverse: bool) -> usize {
        let slice = self.0[row][start..end].iter();
        get_reach(
            value,
            if reverse {
                slice.rev().collect()
            } else {
                slice.collect()
            },
        )
    }

    fn get_col_iter(&self, col: usize, value: u32, start: usize, end: usize) -> bool {
        self.0[start..end]
            .iter()
            .map(move |row| &row[col])
            .all(|&x| x < value)
    }

    fn col_reach(&self, col: usize, value: u32, start: usize, end: usize, reverse: bool) -> usize {
        let slice = self.0[start..end].iter().map(move |row| &row[col]);
        get_reach(
            value,
            if reverse {
                slice.rev().collect()
            } else {
                slice.collect()
            },
        )
    }
}

fn get_reach(value: u32, arr: Vec<&u32>) -> usize {
    let mut count = 0;
    for v in arr {
        count += 1;

        if *v >= value {
            break;
        }
    }
    count
}

impl From<&str> for Matrix {
    fn from(input: &str) -> Self {
        let matrix = input
            .lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();
        Self(matrix)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calc_b() {
        let input = r#"30373
25512
65332
33549
35390"#;
        assert_eq!(calc_b(input), "8");
    }
}
