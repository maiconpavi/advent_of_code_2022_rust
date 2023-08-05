use std::{cmp::Reverse, collections::VecDeque};

static THRESHOLD: u64 = u32::MAX as u64 - 1;

#[must_use]
pub fn calc_a(input: &str) -> String {
    execute_program(input, 20, Some(3)).to_string()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    execute_program(input, 10000, None).to_string()
}

#[allow(clippy::iter_with_drain)]
fn execute_program(input: &str, rounds: usize, divide_by: Option<u64>) -> usize {
    let mut monkeys = parse_monkeys(input);
    let items_len = monkeys.iter().map(|m| m.items.len()).sum::<usize>();
    let mut monkeys_count = monkeys.iter().map(|_| 0).collect::<Vec<_>>();
    let mut outputs = Vec::<(usize, Item)>::with_capacity(items_len);
    for _ in 0..rounds {
        for monkey in &mut monkeys {
            let p = outputs
                .drain(..)
                .partition::<Vec<_>, _>(|(dest, _)| *dest == monkey.id);
            outputs = p.1;
            monkey.items.extend(p.0.into_iter().map(|(_, item)| item));

            let id = monkey.id;
            let output = monkey.execute_round(divide_by);
            monkeys_count[id] += output.len();

            outputs.extend(output);
        }
        outputs.drain(..).for_each(|(dest, value)| {
            monkeys[dest].items.push_back(value);
        });
    }

    monkeys_count.sort_by_key(|c| Reverse(*c));
    monkeys_count[..2]
        .iter()
        .copied()
        .reduce(|a, b| a * b)
        .expect("failed to get monkeys count")
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(Monkey::try_from)
        .collect::<Result<_, _>>()
        .expect("failed to parse monkeys")
}

struct Monkey {
    id: usize,
    items: VecDeque<Item>,
    operation: Operation,
    test: Test,
}

#[derive(Debug)]
struct Item {
    value: u64,
    operations: Vec<Operation>,
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Pow2,
    Multiply(u64),
    Add(u64),
}

struct Test {
    divisible_by: u64,
    dest_if_true: usize,
    dest_if_false: usize,
}

impl Monkey {
    fn execute_round(
        &mut self,
        divide_by: Option<u64>,
    ) -> impl ExactSizeIterator<Item = (usize, Item)> + '_ {
        let test = &self.test;
        let operation = self.operation;
        self.items.drain(..).map(move |mut item| {
            item.operations.push(operation);
            (test.get_dest(&item, divide_by), item)
        })
    }
}

impl Item {
    const fn new(value: u64) -> Self {
        Self {
            value,
            operations: Vec::new(),
        }
    }

    fn get_mod_equivalent(&self, n: u64, divide_by: Option<u64>) -> u64 {
        divide_by.map_or_else(
            || {
                self.operations
                    .iter()
                    .fold(self.value, |acc, op| op.apply_with_mod(acc, n))
            },
            |divide_by| {
                self.operations
                    .iter()
                    .fold(self.value, |acc, op| op.apply(acc, divide_by))
            },
        )
    }

    fn is_divisible_by(&self, n: u64, divide_by: Option<u64>) -> bool {
        self.get_mod_equivalent(n, divide_by) % n == 0
    }
}

impl Operation {
    fn apply_with_mod(&self, mut value: u64, n: u64) -> u64 {
        match self {
            Self::Pow2 => {
                if value > THRESHOLD {
                    value %= n;
                }
                value * value
            }
            Self::Multiply(m) => {
                if value > THRESHOLD {
                    value %= n;
                }
                value * m
            }
            Self::Add(a) => value + a,
        }
    }

    fn apply(&self, value: u64, divide_by: u64) -> u64 {
        (match self {
            Self::Pow2 => value * value,
            Self::Multiply(m) => value * m,
            Self::Add(a) => value + a,
        }) / divide_by
    }
}

impl Test {
    fn get_dest(&self, item: &Item, divide_by: Option<u64>) -> usize {
        if item.is_divisible_by(self.divisible_by, divide_by) {
            self.dest_if_true
        } else {
            self.dest_if_false
        }
    }
}

impl TryFrom<&str> for Monkey {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines().filter(|l| !l.is_empty());
        let id = lines
            .next()
            .ok_or_else(|| "missing id line".to_string())?
            .split_whitespace()
            .next_back()
            .ok_or_else(|| "missing id value".to_string())?
            .trim_end_matches(':')
            .parse()
            .map_err(|e| format!("failed to parse id: {e}"))?;
        let items = lines
            .next()
            .ok_or_else(|| "missing items line".to_string())?
            .split_once(": ")
            .ok_or_else(|| "missing items separator".to_string())?
            .1
            .split(", ")
            .filter_map(|v| Some(Item::new(v.parse().ok()?)))
            .collect();
        let operation = lines
            .next()
            .ok_or_else(|| "missing operation line".to_string())?
            .try_into()?;
        let test = value
            .split_once("Test:")
            .ok_or_else(|| "missing test line".to_string())?
            .1
            .try_into()?;
        Ok(Self {
            id,
            items,
            operation,
            test,
        })
    }
}

impl TryFrom<&str> for Test {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines().filter(|l| !l.is_empty());
        let divisible_by = lines
            .next()
            .ok_or_else(|| "missing divisible by line".to_string())?
            .split_whitespace()
            .next_back()
            .ok_or_else(|| "missing divisible by value".to_string())?
            .parse()
            .map_err(|e| format!("failed to parse divisible by: {e}"))?;
        let dest_if_true = lines
            .next()
            .ok_or_else(|| "missing dest if true line".to_string())?
            .split_whitespace()
            .next_back()
            .ok_or_else(|| "missing dest if true value".to_string())?
            .parse()
            .map_err(|e| format!("failed to parse dest if true: {e}"))?;
        let dest_if_false = lines
            .next()
            .ok_or_else(|| "missing dest if false line".to_string())?
            .split_whitespace()
            .next_back()
            .ok_or_else(|| "missing dest if false value".to_string())?
            .parse()
            .map_err(|e| format!("failed to parse dest if false: {e}"))?;

        Ok(Self {
            divisible_by,
            dest_if_true,
            dest_if_false,
        })
    }
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(' ');
        let operation_value = parts
            .next_back()
            .ok_or_else(|| "missing operation value".to_string())?;
        let signal = parts
            .next_back()
            .ok_or_else(|| "missing operation signal".to_string())?;
        match (signal, operation_value) {
            ("*", "old") => Ok(Self::Pow2),
            ("*", value) => {
                Ok(Self::Multiply(value.parse().map_err(|e| {
                    format!("failed to parse operation value: {e}")
                })?))
            }
            ("+", value) => {
                Ok(Self::Add(value.parse().map_err(|e| {
                    format!("failed to parse operation value: {e}")
                })?))
            }
            _ => Err(format!("invalid operation: {value}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_a() {
        assert_eq!(
            calc_a(include_str!("../../../inputs/day_11/test_input.txt")),
            "10605"
        );
    }

    #[test]
    fn test_calc_b() {
        assert_eq!(
            calc_b(include_str!("../../../inputs/day_11/test_input.txt")),
            "2713310158"
        );
    }

    #[test]
    fn test_item_1() {
        let n = 5;
        let mut item = Item {
            value: 7,
            operations: vec![Operation::Pow2],
        };

        assert_eq!(item.get_mod_equivalent(n, None) % n, 4);
        item.operations.push(Operation::Multiply(3));
        assert_eq!(item.get_mod_equivalent(n, None) % n, 2);
        item.operations.push(Operation::Add(1));
        assert_eq!(item.get_mod_equivalent(n, None) % n, 3);
    }

    #[test]
    fn test_item_2() {
        let n = 23;
        let item = Item {
            value: 65,
            operations: vec![Operation::Add(6), Operation::Multiply(19)],
        };
        assert_eq!(item.get_mod_equivalent(n, Some(3)) % n, 7);
        assert!(!item.is_divisible_by(n, Some(3)));
    }

    #[test]
    fn test_item_3() {
        let n = 19;
        let item = Item {
            value: 79,
            operations: vec![
                Operation::Multiply(19),
                Operation::Add(3),
                Operation::Add(6),
            ],
        };
        assert_eq!(item.get_mod_equivalent(n, Some(3)) % n, 0);
        assert!(item.is_divisible_by(n, Some(3)));
    }
}
