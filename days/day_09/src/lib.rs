use std::collections::HashSet;

#[must_use]
pub fn calc_a(input: &str) -> String {
    get_visited_cords_count::<1>(input)
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    get_visited_cords_count::<9>(input)
}

fn get_visited_cords_count<const TAIL_LEN: usize>(input: &str) -> String {
    let mut grid = Grid::<TAIL_LEN>::new();
    let moves = input
        .lines()
        .filter(|s| !s.is_empty())
        .filter_map(|s| KnotMove::try_from(s).ok())
        .collect::<Vec<_>>();
    for km in &moves {
        grid.move_rope(km);
    }
    grid.visited_cords.len().to_string()
}

struct Grid<const TAIL_LEN: usize> {
    rope: Rope<TAIL_LEN>,
    visited_cords: HashSet<Cord>,
}

struct Rope<const TAIL_LEN: usize> {
    head: Knot,
    knots: [Knot; TAIL_LEN],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Knot(Cord);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct KnotMove {
    direction: Direction,
    distance: usize,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<const TAIL_LEN: usize> Grid<TAIL_LEN> {
    fn new() -> Self {
        Self {
            rope: Rope::new(),
            visited_cords: HashSet::from_iter([Cord { x: 0, y: 0 }]),
        }
    }

    fn move_rope(&mut self, knot_move: &KnotMove) {
        self.visited_cords.extend(self.rope.move_rope(knot_move));
    }
}

impl<const TAIL_LEN: usize> Rope<TAIL_LEN> {
    const fn new() -> Self {
        Self {
            head: Knot::new(),
            knots: [Knot::new(); TAIL_LEN],
        }
    }
    fn move_rope(&mut self, knot_move: &KnotMove) -> Vec<Cord> {
        let mut visited_cords = Vec::new();
        for _ in 0..knot_move.distance {
            self.head.move_knot(knot_move.direction);
            let mut last_knot = &self.head;
            for (i, knot) in self.knots.iter_mut().enumerate() {
                if knot.follow_up(last_knot) && i == TAIL_LEN - 1 {
                    visited_cords.push(knot.0);
                }
                last_knot = knot;
            }
        }
        visited_cords
    }
}

impl Knot {
    const fn new() -> Self {
        Self(Cord { x: 0, y: 0 })
    }
    /// Returns true if the knot moved.
    /// # Panics
    /// If the knot cannot move.
    fn follow_up(&mut self, other: &Self) -> bool {
        if self.0.step_distance(other.0) <= 1 {
            return false;
        }
        self.0 = self
            .0
            .surrounding()
            .into_iter()
            .fold(self.0, |mut saved_cord, cord| {
                let d1 = cord.distance(other.0);
                let d2 = saved_cord.distance(other.0);
                if d1 + f64::EPSILON < d2 {
                    saved_cord = cord;
                }
                saved_cord
            });

        true
    }

    fn move_knot(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.0.y += 1,
            Direction::Down => self.0.y -= 1,
            Direction::Left => self.0.x -= 1,
            Direction::Right => self.0.x += 1,
        }
    }
}

impl Cord {
    fn distance(self, other: Self) -> f64 {
        let x_diff = f64::from((self.x - other.x).abs());
        let y_diff = f64::from((self.y - other.y).abs());
        x_diff.hypot(y_diff)
    }

    fn step_distance(self, other: Self) -> i32 {
        let mut point = self;
        let mut steps = 0;
        while point != other {
            let mut distance = other.distance(point);
            for possible_cord in point.surrounding() {
                let d = other.distance(possible_cord);
                if d + f64::EPSILON < distance {
                    distance = d;
                    point = possible_cord;
                }
            }
            steps += 1;
        }

        steps
    }
    const fn surrounding(self) -> [Self; 8] {
        [
            Self {
                x: self.x - 1,
                y: self.y - 1,
            },
            Self {
                x: self.x,
                y: self.y - 1,
            },
            Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            Self {
                x: self.x + 1,
                y: self.y,
            },
            Self {
                x: self.x + 1,
                y: self.y + 1,
            },
            Self {
                x: self.x,
                y: self.y + 1,
            },
            Self {
                x: self.x - 1,
                y: self.y + 1,
            },
            Self {
                x: self.x - 1,
                y: self.y,
            },
        ]
    }
}

impl TryFrom<&str> for KnotMove {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            direction: Direction::try_from(&value[0..1])?,
            distance: value[2..].parse::<usize>().map_err(|e| e.to_string())?,
        })
    }
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("invalid direction: {value}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calc_a() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

        assert_eq!(calc_a(input), "13");
    }

    #[test]
    fn test_calc_b() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

        assert_eq!(calc_b(input), "36");
    }
}
