use std::collections::HashSet;

const LOWER_A_DEC: u64 = 97;
const UPPER_A_DEC: u64 = 65;

#[must_use]
pub fn calc_a(input: &str) -> String {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (c1, c2) = line.split_at(line.len() / 2);
            let (s1, s2) = (
                c1.chars().collect::<HashSet<char>>(),
                c2.chars().collect::<HashSet<char>>(),
            );
            let item = s1.intersection(&s2).next()?;
            char_to_priority(*item)
        })
        .sum::<u64>()
        .to_string()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Box<[_]>>()
        .chunks(3)
        .filter_map(|lines| {
            let sets = lines
                .iter()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .collect::<Box<[_]>>();
            let item = sets
                .first()?
                .iter()
                .find(|c| sets.iter().all(|s| s.contains(c)))?;
            char_to_priority(*item)
        })
        .sum::<u64>()
        .to_string()
}

#[must_use]
fn char_to_priority(c: char) -> Option<u64> {
    Some(match c {
        'a'..='z' => u64::from(c) - LOWER_A_DEC + 1,
        'A'..='Z' => u64::from(c) - UPPER_A_DEC + 27,
        _ => return None,
    })
}
