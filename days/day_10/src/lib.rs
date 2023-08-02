#[must_use]
pub fn calc_a(input: &str) -> String {
    let mut program = Program::new_from_input(input);
    program.execute();
    let values = [20, 60, 100, 140, 180, 220]
        .into_iter()
        .filter_map(|cycle| program.x_signal_strength(cycle))
        .collect::<Vec<_>>();
    println!("{values:?}");
    values.iter().sum::<isize>().to_string()
}

#[must_use]
pub fn calc_b(input: &str) -> String {
    let mut program = Program::new_from_input(input);
    program.execute();
    program.draw(40, 6)
}

enum Instruction {
    NoOp,
    AddToX(isize),
}

struct Program {
    instructions: Vec<Instruction>,
    registers: Registers,
    instruction_pointer: usize,
    instruction_cycle_count: usize,
    cycles: Vec<Registers>,
}

#[derive(Debug, Clone)]
struct Registers {
    x: isize,
}

impl Program {
    fn new_from_input(input: &str) -> Self {
        let instructions = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(Instruction::try_from)
            .collect::<Result<Vec<_>, _>>()
            .expect("failed to parse instructions");
        Self::new(instructions)
    }
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            registers: Registers { x: 1 },
            instruction_pointer: 0,
            instruction_cycle_count: 0,
            cycles: Vec::new(),
        }
    }

    fn execute_cycle(&mut self) -> bool {
        if let Some(instruction) = self.instructions.get(self.instruction_pointer) {
            self.cycles.push(self.registers.clone());
            self.instruction_cycle_count += 1;
            if instruction.execute_cycle(self.instruction_cycle_count, &mut self.registers) {
                self.instruction_pointer += 1;
                self.instruction_cycle_count = 0;
            }
            true
        } else {
            self.cycles.push(self.registers.clone());
            false
        }
    }

    fn execute(&mut self) {
        while self.execute_cycle() {}
    }

    fn x_signal_strength(&self, cycle: usize) -> Option<isize> {
        self.cycles.get(cycle - 1).map(|p| cycle as isize * p.x)
    }

    fn draw(&self, width: usize, height: usize) -> String {
        let mut buffer = String::new();
        let mut counter = 0;
        let size = width * height;
        while counter < size {
            let x = (counter % width) as isize;
            let value = self
                .cycles
                .get(counter)
                .map_or_else(|| panic!("missing value for {counter}"), |p| p.x);
            if (value.saturating_sub(1)..=value.saturating_add(1)).contains(&x) {
                buffer.push('#');
            } else {
                buffer.push('.');
            }
            if counter % width == width - 1 && counter != size - 1 {
                buffer.push('\n');
            }
            counter += 1;
        }
        buffer
    }
}

impl Instruction {
    fn execute_cycle(&self, cycle_count: usize, persistence: &mut Registers) -> bool {
        match (self, cycle_count) {
            (Self::NoOp, _) => true,
            (Self::AddToX(value), 2) => {
                persistence.x += value;
                true
            }
            _ => false,
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut parts = s.split_whitespace();
        let instruction = parts
            .next()
            .ok_or_else(|| "missing instruction".to_string())?;
        match instruction {
            "noop" => Ok(Self::NoOp),
            "addx" => {
                let value = parts
                    .next()
                    .ok_or_else(|| "missing value".to_string())?
                    .parse()
                    .map_err(|e| format!("failed to parse value: {e}"))?;
                Ok(Self::AddToX(value))
            }
            _ => Err(format!("unknown instruction: {instruction}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(
            calc_a(include_str!("../../../inputs/day_10/test_input.txt")),
            "13140"
        );
    }
    #[test]
    fn test_b() {
        assert_eq!(
            calc_b(include_str!("../../../inputs/day_10/test_input.txt")),
            r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
        );
    }
}
