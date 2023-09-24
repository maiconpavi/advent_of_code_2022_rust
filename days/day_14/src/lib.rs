#![allow(dead_code, clippy::unwrap_used)]

use std::{fmt::Display, ops::RangeInclusive};

#[must_use]
pub fn calc_a(input: &str) -> String {
    let mut grid = Grid::new(input, (500, 0));
    grid.run().to_string()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    let mut grid = Grid::new(input, (500, 0));
    grid.set_floor(grid.size().1 + 1);
    grid.run().to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Air,
    Sand,
    Rock,
}

struct Grid {
    matrix: Matrix,
    sand_source: (usize, usize),
    floor: Option<usize>,
}

struct Matrix {
    data: Vec<Vec<Tile>>,
    x_size: usize,
    y_size: usize,
}

impl Grid {
    fn new(input: &str, sand_source: (usize, usize)) -> Self {
        Self {
            matrix: Matrix::new_from_rock_paths(input.lines()),
            sand_source,
            floor: None,
        }
    }

    fn set_floor(&mut self, y: usize) {
        self.matrix.grow_y(y + 1);
        self.floor = Some(y);
    }

    const fn size(&self) -> (usize, usize) {
        self.matrix.size()
    }

    fn run(&mut self) -> usize {
        for i in 0.. {
            if self.drop_sand() {
                return i;
            }
        }

        unreachable!()
    }

    fn drop_sand(&mut self) -> bool {
        let (mut x, mut y) = self.sand_source;
        if *self.matrix.get(x, y) == Tile::Sand {
            return true;
        }
        loop {
            if self.floor == Some(y + 1) {
                self.matrix.fill_tile(Tile::Sand, x, y);
                return false;
            }
            if self.matrix.row_is_void(y + 1) {
                return true;
            }

            if *self.matrix.get(x, y + 1) == Tile::Air {
                y += 1;
            } else if *self.matrix.get(x - 1, y + 1) == Tile::Air {
                x -= 1;
                y += 1;
            } else if *self.matrix.get(x + 1, y + 1) == Tile::Air {
                x += 1;
                y += 1;
            } else {
                self.matrix.fill_tile(Tile::Sand, x, y);
                return false;
            }
        }
    }
}

impl Matrix {
    const fn new() -> Self {
        Self {
            data: Vec::new(),
            x_size: 0,
            y_size: 0,
        }
    }

    const fn size(&self) -> (usize, usize) {
        (self.x_size, self.y_size)
    }

    fn new_from_rock_paths<'a>(rock_paths: impl Iterator<Item = &'a str>) -> Self {
        let mut matrix = Self::new();
        rock_paths.for_each(|s| matrix.add_rock_path(s));
        matrix
    }

    fn add_rock_path(&mut self, rock_path: &str) {
        let paths = rock_path
            .split(" -> ")
            .filter_map(|path| {
                let (s1, s2) = path.split_once(',')?;
                Some((s1.parse::<usize>().ok()?, s2.parse::<usize>().ok()?))
            })
            .collect::<Vec<_>>();
        for ((mut x1, mut y1), (mut x2, mut y2)) in paths.windows(2).map(|w| (w[0], w[1])) {
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            if y1 > y2 {
                std::mem::swap(&mut y1, &mut y2);
            }
            self.fill(Tile::Rock, x1..=x2, y1..=y2);
        }
    }

    fn grow_x(&mut self, x_size: usize) {
        if x_size > self.x_size {
            self.data
                .iter_mut()
                .for_each(|v| v.resize(x_size, Tile::Air));
            self.x_size = x_size;
        }
    }

    fn grow_y(&mut self, y_size: usize) {
        if y_size > self.y_size {
            self.data.resize_with(y_size, || {
                let mut v = Vec::with_capacity(self.x_size);
                v.resize(self.x_size, Tile::Air);
                v
            });
            self.y_size = y_size;
        }
    }

    fn grow(&mut self, x_size: usize, y_size: usize) {
        self.grow_x(x_size);
        self.grow_y(y_size);
    }

    fn get(&mut self, x: usize, y: usize) -> &Tile {
        self.grow(x + 1, y + 1);
        &self.data[y][x]
    }

    fn row_is_void(&self, y: usize) -> bool {
        self.data.get(y).is_none()
    }

    fn fill_tile(&mut self, tile: Tile, x: usize, y: usize) {
        self.grow(x + 1, y + 1);
        self.data[y][x] = tile;
    }

    fn fill(&mut self, tile: Tile, x_range: RangeInclusive<usize>, y_range: RangeInclusive<usize>) {
        for y in y_range {
            self.grow(*x_range.end() + 1, y + 1);
            for x in x_range.clone() {
                self.data[y][x] = tile;
            }
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for tile in row {
                write!(
                    f,
                    "{}",
                    match tile {
                        Tile::Air => '.',
                        Tile::Sand => 'o',
                        Tile::Rock => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Tile {
    const fn is_empty(self) -> bool {
        matches!(self, Self::Air)
    }

    fn put_sand(&mut self) {
        *self = Self::Sand;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../../inputs/day_14/test_input.txt");

    #[test]
    fn test_a() {
        let mut grid = Grid::new(TEST_INPUT, (500, 0));
        assert_eq!(grid.run(), 24);
    }

    #[test]
    fn test_b() {
        let mut grid = Grid::new(TEST_INPUT, (500, 0));
        std::fs::write("test.txt", grid.matrix.to_string()).expect("failed to write file");
        grid.set_floor(grid.size().1 + 1);
        assert_eq!(grid.run(), 93);
    }

    #[test]
    fn test_input() {
        let mut grid = Grid::new(include_str!("../../../inputs/day_14/input.txt"), (500, 0));
        grid.set_floor(grid.size().1 + 1);
        assert_eq!(grid.run(), 27324);

        std::fs::write("test.txt", grid.matrix.to_string()).expect("failed to write file");
    }
}
