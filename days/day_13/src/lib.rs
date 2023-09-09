#![allow(clippy::unwrap_used, clippy::significant_drop_tightening)]

use std::{cmp::Ordering, str::FromStr};

use nom::{
    branch::alt, bytes::complete::take_while1, character::complete::char, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};
use nom_locate::LocatedSpan;
use nom_supreme::error::ErrorTree;
pub type RawSpan<'a> = LocatedSpan<&'a str>;
pub type RawParseError<'a> = ErrorTree<RawSpan<'a>>;

pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T, RawParseError<'a>>;

#[must_use]
pub fn calc_a(input: &str) -> String {
    let pairs = parse_input(input);
    pairs
        .into_iter()
        .enumerate()
        .map(|(i, (a, b))| {
            if a.is_right_order(&b).expect("invalid packet") {
                i + 1
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    let mut input = input.replace("\n\n", "\n");
    input.push_str(
        r#"[[2]]
[[6]]"#,
    );
    let mut packets = input
        .split('\n')
        .map(|s| parse_element(RawSpan::new(s)).expect("invalid packet").1)
        .collect::<Vec<_>>();
    packets.sort_by(|a, b| match a.is_right_order(b) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });

    let first_package = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let second_package = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    let first_pos = packets
        .iter()
        .position(|v| *v == first_package)
        .expect("first divider packet not found");
    let second_pos = packets
        .iter()
        .position(|v| *v == second_package)
        .expect("second divider packet not found");

    ((first_pos + 1) * (second_pos + 1)).to_string()
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn is_right_order(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => match a.cmp(b) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None,
            },
            (Self::List(a), Self::List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();
                loop {
                    match (a.next(), b.next()) {
                        (Some(a), Some(b)) => {
                            if let Some(v) = a.is_right_order(b) {
                                return Some(v);
                            }
                        }
                        (None, None) => return None,
                        (None, Some(_)) => return Some(true),
                        (Some(_), None) => return Some(false),
                    }
                }
            }
            (Self::Number(a), b @ Self::List(_)) => {
                Self::List(vec![Self::Number(*a)]).is_right_order(b)
            }
            (a @ Self::List(_), Self::Number(b)) => {
                a.is_right_order(&Self::List(vec![Self::Number(*b)]))
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|s| {
            let lines = s.split_once('\n').unwrap();
            let a = parse_element(RawSpan::new(lines.0))
                .expect("invalid packet")
                .1;
            let b = parse_element(RawSpan::new(lines.1))
                .expect("invalid packet")
                .1;
            (a, b)
        })
        .collect()
}

fn parse_element(input: RawSpan<'_>) -> ParseResult<'_, Packet> {
    alt((
        map(parse_number, Packet::Number),
        map(parse_list, Packet::List),
    ))(input)
}
fn parse_number(input: RawSpan<'_>) -> ParseResult<'_, u32> {
    take_while1(|c: char| c.is_ascii_digit())(input).map(|(i, o)| {
        let o = u32::from_str(o.fragment()).unwrap();
        (i, o)
    })
}

fn parse_list(input: RawSpan<'_>) -> ParseResult<'_, Vec<Packet>> {
    delimited(
        char('['),
        separated_list0(char(','), parse_element),
        char(']'),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = include_str!("../../../inputs/day_13/test_input.txt");
        let expected = "13";
        let actual = calc_a(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_b() {
        let input = include_str!("../../../inputs/day_13/test_input.txt");
        let expected = "140";
        let actual = calc_b(input);
        assert_eq!(actual, expected);
    }
}
