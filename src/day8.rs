use std::{collections::HashSet, str::FromStr};

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

#[derive(Debug, Error)]
enum InstructionParseError {
    #[error("Unknown instruction {_0}")]
    UnknownInstruction(String),
    #[error("Int parse error {_0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = input.split(" ").collect();
        match words[0] {
            "acc" => Ok(Instruction::Acc(words[1].parse()?)),
            "jmp" => Ok(Instruction::Jmp(words[1].parse()?)),
            "nop" => Ok(Instruction::Nop(words[1].parse()?)),
            x => Err(InstructionParseError::UnknownInstruction(x.to_string())),
        }
    }
}

#[aoc_generator(day8)]
fn parse_instructions(input: &str) -> Result<Vec<Instruction>, InstructionParseError> {
    input.lines().map(Instruction::from_str).collect()
}

#[derive(Default)]
struct State {
    acc: i64,
    code_pointer: isize,
    visited: HashSet<isize>,
}

impl State {
    pub fn execute(&mut self, instr: Instruction) -> bool {
        if !self.visited.insert(self.code_pointer) {
            return false;
        }
        match instr {
            Instruction::Acc(x) => self.acc += x,
            Instruction::Jmp(x) => {
                self.code_pointer = self.code_pointer + x as isize;
                return true;
            }
            Instruction::Nop(_) => {}
        }
        self.code_pointer += 1;
        true
    }
}

#[aoc(day8, part1)]
fn part_one(input: &[Instruction]) -> i64 {
    let mut state = State::default();
    loop {
        let instr = input[state.code_pointer as usize];
        if !state.execute(instr) {
            break;
        }
    }
    state.acc
}

#[aoc(day8, part2)]
fn part_two(input: &[Instruction]) -> i64 {
    let mut safe_instr: HashSet<isize> = HashSet::default();
    loop {
        let mut state = State::default();
        let mut can_swap = true;
        let mut faulty_index = 0;
        loop {
            if state.code_pointer as usize == input.len() {
                return state.acc;
            }
            let mut instr = input[state.code_pointer as usize];
            if !safe_instr.contains(&state.code_pointer) && can_swap {
                instr = match instr {
                    Instruction::Jmp(x) => {
                        faulty_index = state.code_pointer;
                        can_swap = false;
                        Instruction::Nop(x)
                    }
                    Instruction::Nop(x) => {
                        faulty_index = state.code_pointer;
                        can_swap = false;
                        Instruction::Jmp(x)
                    }
                    x => x,
                };
            }
            if !state.execute(instr) {
                break;
            }
        }
        safe_instr.insert(faulty_index);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const DUMMY_INPUT: &'static str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part_one() {
        let input = parse_instructions(DUMMY_INPUT).expect("Failed to parse instructions");
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    fn test_part_two() {
        let input = parse_instructions(DUMMY_INPUT).expect("Failed to parse instructions");
        assert_eq!(part_two(&input), 8);
    }
}
