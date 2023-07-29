use std::collections::{HashMap, HashSet};

#[must_use]
pub fn calc_a(input: &str) -> String {
    get_visited_cords_count::<1>(input)
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    get_visited_cords_count::<9>(input)
}

fn get_visited_cords_count<const TAIL_LEN: usize>(input: &str) -> String {
    let grid = get_grid::<TAIL_LEN>(input);
    let visited_cords = grid.get_last_knot_visited_cords();
    visited_cords.len().to_string()
}

#[must_use]
pub fn get_grid<const TAIL_LEN: usize>(input: &str) -> Grid<TAIL_LEN> {
    let mut grid = Grid::<TAIL_LEN>::new();
    let moves = input
        .lines()
        .filter(|s| !s.is_empty())
        .filter_map(|s| KnotMove::try_from(s).ok())
        .collect::<Vec<_>>();
    for km in &moves {
        grid.move_rope(km);
    }
    grid
}

pub struct Grid<const TAIL_LEN: usize> {
    rope: Rope<TAIL_LEN>,
    states_map: Vec<(KnotMove, Vec<GridState<TAIL_LEN>>)>,
}

struct GridState<const TAIL_LEN: usize> {
    rope: Rope<TAIL_LEN>,
}

#[derive(Debug, Clone)]
struct Rope<const TAIL_LEN: usize> {
    head: Knot,
    tail: [Knot; TAIL_LEN],
}

#[derive(Debug, Clone, Copy)]
struct Knot(Cord);

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Cord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct KnotMove {
    direction: Direction,
    distance: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<const TAIL_LEN: usize> Grid<TAIL_LEN> {
    const fn new() -> Self {
        Self {
            rope: Rope::new(),
            states_map: Vec::new(),
        }
    }

    fn get_last_knot_visited_cords(&self) -> HashSet<Cord> {
        let mut visited_cords = HashSet::from_iter([Cord { x: 0, y: 0 }]);
        for (_, states) in &self.states_map {
            for state in states {
                let Some(last_knot) = state.rope.tail.last() else {
                continue;
            };
                visited_cords.insert(last_knot.0);
            }
        }
        visited_cords
    }

    fn move_rope(&mut self, knot_move: &KnotMove) {
        self.states_map
            .push((*knot_move, self.rope.move_rope(knot_move)));
    }

    /// Returns a string visualization of the grid.
    #[must_use]
    pub fn get_visualization(&self) -> String {
        let cords = self.grid_points();
        let (min_x, min_y) = (cords.0.x, cords.0.y);
        let (max_x, max_y) = (cords.1.x, cords.1.y);
        let mut buf = String::from("== Initial State ==\n\n");
        buf.push_str(
            &GridState {
                rope: Rope::<TAIL_LEN>::new(),
            }
            .get_visualization(cords),
        );
        for (mv, states) in &self.states_map {
            buf.push_str(&mv.get_visualization());
            for state in states {
                buf.push_str(&state.get_visualization(cords));
            }
        }
        buf.push_str("\n\n== Visited by Last Knot ==\n\n");
        let visited_cords = self.get_last_knot_visited_cords();
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let cord = Cord { x, y };
                if x == 0 && y == 0 {
                    buf.push('s');
                } else if visited_cords.contains(&cord) {
                    buf.push('#');
                } else {
                    buf.push('.');
                }
            }
            buf.push('\n');
        }

        buf
    }

    /// Returns the min and max x and y cords of the grid.
    fn grid_points(&self) -> (Cord, Cord) {
        let x_cords = self
            .states_map
            .iter()
            .flat_map(|(_, states)| {
                states.iter().flat_map(|state| {
                    state
                        .rope
                        .tail
                        .iter()
                        .map(|knot| knot.0.x)
                        .chain(std::iter::once(state.rope.head.0.x))
                })
            })
            .collect::<Box<[_]>>();
        let y_cords = self
            .states_map
            .iter()
            .flat_map(|(_, states)| {
                states.iter().flat_map(|state| {
                    state
                        .rope
                        .tail
                        .iter()
                        .map(|knot| knot.0.y)
                        .chain(std::iter::once(state.rope.head.0.y))
                })
            })
            .collect::<Box<[_]>>();

        let min_x = x_cords.iter().min().expect("No x cords");
        let max_x = x_cords.iter().max().expect("No x cords");
        let min_y = y_cords.iter().min().expect("No y cords");
        let max_y = y_cords.iter().max().expect("No y cords");

        (
            Cord {
                x: *min_x,
                y: *min_y,
            },
            Cord {
                x: *max_x,
                y: *max_y,
            },
        )
    }
}

impl<const TAIL_LEN: usize> Rope<TAIL_LEN> {
    const fn new() -> Self {
        Self {
            head: Knot::new(),
            tail: [Knot::new(); TAIL_LEN],
        }
    }
    fn move_rope(&mut self, knot_move: &KnotMove) -> Vec<GridState<TAIL_LEN>> {
        let mut states = Vec::new();
        for _ in 0..knot_move.distance {
            self.head.move_one(knot_move.direction);

            self.tail.iter_mut().fold(self.head, |mut last_knot, knot| {
                knot.follow_up(last_knot);
                last_knot = *knot;
                last_knot
            });

            states.push(self.clone().into());
        }

        states
    }
}

impl Knot {
    const fn new() -> Self {
        Self(Cord { x: 0, y: 0 })
    }
    /// Returns true if the knot moved.
    /// # Panics
    /// If the knot cannot move.
    fn follow_up(&mut self, other: Self) {
        if self.0.step_distance(other.0) <= 1 {
            return;
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
    }

    fn move_one(&mut self, direction: Direction) {
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

impl KnotMove {
    /// Returns the visualization of the knot move.
    /// # Example
    /// ```
    /// == U 1 ==
    ///
    /// ```
    fn get_visualization(&self) -> String {
        let mv: &str = match self.direction {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
        };
        format!("== {} {} ==\n\n", mv, self.distance)
    }
}

#[inline]
/// Draws the vizualization of a row.
/// # Example
/// ```
/// 1H....  ( 1 convers s )
/// ```
fn draw_row(
    visualization: &mut String,
    min_x: i32,
    max_x: i32,
    y: i32,
    cords: &HashMap<(i32, i32), Vec<char>>,
) {
    let mut covereds = Vec::new();
    for x in min_x..=max_x {
        if let Some(knots) = cords.get(&(x, y)) {
            visualization.push(*knots.first().expect("No knot found"));
            if knots.len() > 1 {
                covereds.push(knots.as_slice());
            }
        } else {
            visualization.push('.');
        }
    }

    draw_convered_points(visualization, covereds);
    visualization.push('\n');
}

#[inline]
/// Draws the covered points in the visualization.
/// Example
/// ```
///   ( A convers B, C, D ; E convers F, G )
/// ```
fn draw_convered_points(visualization: &mut String, covereds: Vec<&[char]>) {
    if !covereds.is_empty() {
        visualization.push_str("  (");
        let mut messages = Vec::with_capacity(covereds.len());
        for covered in covereds {
            let message = format!(
                " {} convers {} ",
                covered[0],
                &covered
                    .iter()
                    .skip(1)
                    .flat_map(|c| [*c, ',', ' '])
                    .collect::<String>()[..covered.len() * 3 - 2]
            );
            messages.push(message);
        }
        visualization.push_str(&messages.join(";"));
        visualization.push(')');
    }
}

impl<const TAIL_LEN: usize> GridState<TAIL_LEN> {
    #[inline]
    /// Returns the visualization of the grid state.
    /// # Example
    /// ```
    /// ......
    /// ......
    /// ......
    /// ......
    /// 1H....  ( 1 convers s )
    /// ```
    fn get_visualization(&self, (min_cord, max_cord): (Cord, Cord)) -> String {
        let (min_x, min_y) = (min_cord.x, min_cord.y);
        let (max_x, max_y) = (max_cord.x, max_cord.y);

        let mut cords = HashMap::<(i32, i32), Vec<char>>::new();
        cords.entry(self.rope.head.0.into()).or_default().push('H');
        for (i, knot) in self.rope.tail.iter().enumerate() {
            cords
                .entry(knot.0.into())
                .or_default()
                .push(char::from_digit(i as u32 + 1, 16).expect("Invalid digit"));
        }
        cords.entry((0, 0)).or_default().push('s');

        let mut visualization = String::new();
        for y in (min_y..=max_y).rev() {
            draw_row(&mut visualization, min_x, max_x, y, &cords);
        }
        visualization.push('\n');

        visualization
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

impl<const TAIL_LEN: usize> From<Rope<TAIL_LEN>> for GridState<TAIL_LEN> {
    fn from(rope: Rope<TAIL_LEN>) -> Self {
        Self { rope }
    }
}
impl From<Cord> for (i32, i32) {
    fn from(cord: Cord) -> Self {
        (cord.x, cord.y)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;
    const TEST_INPUT: &str = include_str!("../../../inputs/day_09/test_input.txt");

    #[test]
    #[ignore]
    fn test_visualization_1() {
        let grid = get_grid::<1>(TEST_INPUT);
        let visualization = grid.get_visualization();
        std::fs::write("test_visualization_1.txt", visualization).unwrap();
    }

    #[test]
    #[ignore]
    fn test_visualization_2() {
        let grid = get_grid::<9>(TEST_INPUT);
        let visualization = grid.get_visualization();
        std::fs::write("test_visualization_2.txt", visualization).unwrap();
    }

    #[test]
    fn test_calc_a() {
        assert_eq!(calc_a(TEST_INPUT), "13");
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
