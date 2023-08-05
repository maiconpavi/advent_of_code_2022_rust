use std::collections::{HashMap, HashSet, VecDeque};

#[must_use]
pub fn calc_a(input: &str) -> String {
    let height_map = parse_height_map(input).expect("failed to parse input");
    height_map.shortest_path(Square::Source).map_or_else(
        || "No path found".to_string(),
        |path| (path.len() - 1).to_string(),
    )
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    let height_map = parse_height_map(input).expect("failed to parse input");
    height_map.shortest_path(Square::Other(1)).map_or_else(
        || "No path found".to_string(),
        |path| (path.len() - 1).to_string(),
    )
}

struct HeightMap {
    squares: Vec<Vec<Square>>,
    source: Cord,
    destiny: Cord,
    edges: HashMap<Cord, Vec<Cord>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cord(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Square {
    Source,
    Destiny,
    Other(u8),
}

impl HeightMap {
    fn new(squares: Vec<Vec<Square>>) -> Result<Self, String> {
        let mut source = None;
        let mut destiny = None;
        let mut edges = HashMap::new();

        for (y, row) in squares.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                match square {
                    Square::Source => {
                        if source.is_some() {
                            return Err("Multiple sources".to_string());
                        }
                        source = Some(Cord(x, y));
                    }
                    Square::Destiny => {
                        if destiny.is_some() {
                            return Err("Multiple destinies".to_string());
                        }
                        destiny = Some(Cord(x, y));
                    }
                    Square::Other(_) => {}
                }
                let cord = Cord(x, y);
                edges.insert(cord, cord.neighbors(row.len(), squares.len()));
            }
        }

        for (c1, v) in &mut edges {
            let s1 = &squares[c1.1][c1.0];
            v.retain(|c2| {
                let s2 = &squares[c2.1][c2.0];
                match (s1, s2) {
                    (Square::Other(h1), Square::Other(h2)) => *h1 <= h2 + 1,
                    (Square::Other(h), Square::Source) => *h == 1,
                    (Square::Destiny, Square::Other(h)) => *h == 26,
                    _ => false,
                }
            });
        }

        Ok(Self {
            squares,
            source: source.ok_or_else(|| "No source".to_string())?,
            destiny: destiny.ok_or_else(|| "No destiny".to_string())?,
            edges,
        })
    }

    fn shortest_path(&self, goal: Square) -> Option<Vec<Cord>> {
        let mut distances = HashMap::new();
        let mut previous = HashMap::new();
        let mut queue = VecDeque::with_capacity(self.edges.len());

        let mut visisted = HashSet::new();
        visisted.insert(self.destiny);
        distances.insert(self.destiny, 0);
        queue.push_back(self.destiny);
        let mut found = None;
        'outer: while let Some(c1) = queue.pop_front() {
            let d1 = distances[&c1];
            if let Some(edges) = self.edges.get(&c1) {
                for c2 in edges {
                    if visisted.contains(c2) {
                        continue;
                    }
                    visisted.insert(*c2);
                    distances.insert(*c2, d1 + 1);
                    previous.insert(*c2, c1);
                    queue.push_back(*c2);
                    if self.squares[c2.1][c2.0] == goal {
                        found = Some(*c2);
                        break 'outer;
                    }
                }
            }
        }

        let mut path = Vec::new();
        let mut c = found?;
        while c != self.destiny {
            path.push(c);
            if let Some(&p) = previous.get(&c) {
                c = p;
            } else {
                return None;
            }
        }
        path.push(self.destiny);
        Some(path)
    }
}

impl Cord {
    fn neighbors(self, max_x: usize, max_y: usize) -> Vec<Self> {
        let Self(x, y) = self;
        let mut neighbors = Vec::with_capacity(4);
        if x > 0 {
            neighbors.push(Self(x - 1, y));
        }
        if y > 0 {
            neighbors.push(Self(x, y - 1));
        }
        if x < max_x - 1 {
            neighbors.push(Self(x + 1, y));
        }
        if y < max_y - 1 {
            neighbors.push(Self(x, y + 1));
        }
        neighbors
    }
}

fn parse_height_map(input: &str) -> Result<HeightMap, String> {
    HeightMap::new(
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|r| {
                r.chars()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>, _>>()
                    .expect("Invalid row")
            })
            .collect(),
    )
}

impl TryFrom<char> for Square {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Source),
            'E' => Ok(Self::Destiny),
            'a'..='z' => Ok(Self::Other(value as u8 - b'a' + 1)),
            _ => Err(format!("Invalid square: {value}")),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_calc_a_1() {
        let input = include_str!("../../../inputs/day_12/test_input.txt");
        assert_eq!(super::calc_a(input), "31");
    }

    #[test]
    fn test_calc_a_2() {
        let input = include_str!("../../../inputs/day_12/input.txt");
        assert_eq!(super::calc_a(input), "449");
    }

    #[test]
    fn test_calc_b_1() {
        let input = include_str!("../../../inputs/day_12/test_input.txt");
        assert_eq!(super::calc_b(input), "29");
    }

    #[test]
    fn test_calc_b_2() {
        let input = include_str!("../../../inputs/day_12/input.txt");
        assert_eq!(super::calc_b(input), "443");
    }
}
