#[must_use]
pub fn calc_a(input: &str) -> String {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (s1, s2) = line.split_once(' ').and_then(|(s1, s2)| {
                Some((Shape::try_from(s1).ok()?, Shape::try_from(s2).ok()?))
            })?;
            let round_result = s2.wins(s1);
            Some(u64::from(s2.shape_score() + round_result.round_score()))
        })
        .sum::<u64>()
        .to_string()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (s1, mut s2) = line.split_once(' ').and_then(|(s1, s2)| {
                Some((Shape::try_from(s1).ok()?, Shape::try_from(s2).ok()?))
            })?;
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

            Some(u64::from(s2.shape_score() + round_result.round_score()))
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
    const fn shape_score(self) -> u8 {
        self as u8
    }

    fn wins(self, other: Self) -> RoundResult {
        match (self, other) {
            (a, b) if a == b => RoundResult::Draw,
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => RoundResult::Win,
            _ => RoundResult::Loss,
        }
    }
}

impl TryFrom<&str> for Shape {
    type Error = String;
    fn try_from(shape: &str) -> Result<Self, Self::Error> {
        match shape {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(format!("Invalid shape: {shape}")),
        }
    }
}

impl RoundResult {
    pub const fn round_score(self) -> u8 {
        self as u8
    }
}
