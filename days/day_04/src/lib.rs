#[must_use]
pub fn calc_a(input: &str) -> String {
    get_ranges(input)
        .filter(|(r1, r2)| r1.fully_contains(r2) || r2.fully_contains(r1))
        .count()
        .to_string()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    get_ranges(input)
        .filter(|(r1, r2)| r1.contains(r2) || r2.contains(r1))
        .count()
        .to_string()
}

fn get_ranges(input: &str) -> impl Iterator<Item = (Range, Range)> + '_ {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (r1, r2) = line.split_once(',')?;
            Some((Range::from(r1), Range::from(r2)))
        })
}

struct Range(u64, u64);

impl Range {
    #[must_use]
    const fn fully_contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    #[must_use]
    const fn contains(&self, other: &Self) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let (first, second) = s.split_once('-').expect("invalid range");
        Self(
            first.parse().expect("invalid range"),
            second.parse().expect("invalid range"),
        )
    }
}
