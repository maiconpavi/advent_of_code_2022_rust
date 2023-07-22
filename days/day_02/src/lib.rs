#[must_use]
pub fn calc_a(input: &str) -> String {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (s1, s2) = line
                .split_once(" ")
                .map(|(s1, s2)| (Shape::from(s1), Shape::from(s2)))
                .unwrap_or_else(|| panic!("Invalid line: {}", line));
            let round_result = s2.wins(&s1);
            u64::from(s2.shape_score() + round_result.round_score())
        })
        .sum::<u64>()
        .to_string()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (s1, mut s2) = line
                .split_once(" ")
                .map(|(s1, s2)| (Shape::from(s1), Shape::from(s2)))
                .unwrap_or_else(|| panic!("Invalid line: {}", line));
            let round_result = match &s2 {
                Shape::Rock => RoundResult::Loss,
                Shape::Paper => RoundResult::Draw,
                Shape::Scissors => RoundResult::Win,
            };
            s2 = match (&s1, &round_result) {
                (s, RoundResult::Draw) => *s,
                (s, RoundResult::Win) => match s {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                },
                (s, RoundResult::Loss) => match s {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper,
                },
            };

            u64::from(s2.shape_score() + round_result.round_score())
        })
        .sum::<u64>()
        .to_string()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RoundResult {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl Shape {
    fn shape_score(&self) -> u8 {
        *self as u8
    }

    fn wins(&self, other: &Self) -> RoundResult {
        match (self, other) {
            (a, b) if *a == *b => RoundResult::Draw,
            (Shape::Rock, Shape::Scissors) => RoundResult::Win,
            (Shape::Paper, Shape::Rock) => RoundResult::Win,
            (Shape::Scissors, Shape::Paper) => RoundResult::Win,
            _ => RoundResult::Loss,
        }
    }
}

impl From<&str> for Shape {
    fn from(shape: &str) -> Self {
        match shape {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Invalid shape: {}", shape),
        }
    }
}

impl RoundResult {
    pub fn round_score(&self) -> u8 {
        *self as u8
    }
}
