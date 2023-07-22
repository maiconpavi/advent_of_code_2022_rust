use std::collections::{BTreeMap, HashMap};

#[must_use]
pub fn calc_a(input: &str) -> String {
    let (stacks_input, procedures_input) = input.split_once("\n\n").expect("invalid input");
    let mut stacks = parse_stacks(stacks_input);
    let procedures = parse_procedures(procedures_input);
    stacks.batch_move_crates(&procedures, true);

    stacks.surface_configuration()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    let (stacks_input, procedures_input) = input.split_once("\n\n").expect("invalid input");
    let mut stacks = parse_stacks(stacks_input);
    let procedures = parse_procedures(procedures_input);
    stacks.batch_move_crates(&procedures, false);

    stacks.surface_configuration()
}

struct Stacks {
    stacks: BTreeMap<usize, Vec<Crate>>,
}

struct Crate {
    mark: Box<str>,
}

struct Procedure {
    quantity: usize,
    source: usize,
    destination: usize,
}

impl Stacks {
    fn new() -> Self {
        Self {
            stacks: BTreeMap::new(),
        }
    }

    fn add_stack(&mut self, stack: usize) {
        self.stacks.insert(stack, Vec::new());
    }

    fn add_crate(&mut self, stack: usize, crate_: Crate) {
        let stack = self
            .stacks
            .get_mut(&stack)
            .unwrap_or_else(|| panic!("stack {stack} does not exist",));
        stack.insert(0, crate_);
    }

    fn move_crates(&mut self, procedure: &Procedure, reverse: bool) {
        let source = self
            .stacks
            .get_mut(&procedure.source)
            .unwrap_or_else(|| panic!("stack {} does not exist", procedure.source));
        let len = source.len();
        let crates_iter = source.drain(len - procedure.quantity..len);
        let crates = if reverse {
            crates_iter.rev().collect::<Vec<_>>()
        } else {
            crates_iter.collect::<Vec<_>>()
        };
        let destination = self
            .stacks
            .get_mut(&procedure.destination)
            .unwrap_or_else(|| panic!("stack {} does not exist", procedure.destination));
        destination.extend(crates);
    }

    fn batch_move_crates(&mut self, procedures: &[Procedure], reverse: bool) {
        for procedure in procedures {
            self.move_crates(procedure, reverse);
        }
    }

    fn surface_crates_ids(&self) -> Vec<&str> {
        self.stacks
            .values()
            .filter_map(|stack| stack.last())
            .map(|crate_| crate_.mark.as_ref())
            .collect()
    }

    fn surface_configuration(&self) -> String {
        self.surface_crates_ids().join("")
    }
}

impl TryFrom<String> for Crate {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.trim().is_empty() {
            return Err("invalid crate id");
        }
        Ok(Self {
            mark: s.trim_matches(['[', ']', ' ', '\n'].as_slice()).into(),
        })
    }
}

//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

fn parse_stacks(stacks_input: &str) -> Stacks {
    let mut stacks = Stacks::new();
    let last_line_idx = stacks_input
        .rfind('\n')
        .expect("stacks input must have at least one line");
    let (raw_stacks, footer) = (
        &stacks_input[..last_line_idx],
        &stacks_input[last_line_idx + 1..],
    );

    for stack_id in footer.split_whitespace().filter_map(|id| id.parse().ok()) {
        stacks.add_stack(stack_id);
    }

    for line in raw_stacks.split_inclusive('\n') {
        for (i, raw_crate) in line.chars().collect::<Box<[char]>>().chunks(4).enumerate() {
            let Ok(crate_) = Crate::try_from(raw_crate.iter().collect::<String>()) else { continue };
            stacks.add_crate(i + 1, crate_);
        }
    }

    stacks
}

fn parse_procedures(procedures_input: &str) -> Box<[Procedure]> {
    procedures_input
        .split('\n')
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            Some(Procedure::from(line))
        })
        .collect()
}

impl From<&str> for Procedure {
    fn from(s: &str) -> Self {
        let mut values = s
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .filter_map(|s| s.parse().ok());
        Self {
            quantity: values.next().expect("invalid procedure quantity"),
            source: values.next().expect("invalid procedure source"),
            destination: values.next().expect("invalid procedure destination"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stacks() {
        let stacks_input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "#;
        let stacks = parse_stacks(stacks_input);
        assert_eq!(stacks.surface_configuration(), "NDP");
    }
}
