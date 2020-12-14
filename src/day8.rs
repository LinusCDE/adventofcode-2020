use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

type Instruction = (Operation, i16);

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    ACC,
    JMP,
    NOP,
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|line| {
            let words: Vec<&str> = line.split(' ').collect();

            let operation = match words[0] {
                "acc" => Operation::ACC,
                "jmp" => Operation::JMP,
                "nop" => Operation::NOP,
                _ => unreachable!("Unexpected instruction!"),
            };
            let argument = words[1].parse().expect("Failed to parse argument!");

            (operation, argument)
        })
        .collect()
}

pub enum ExecutionResult {
    InfiniteLoop {
        acc: i16,
    },
    /// Terminates when running the instruction after the max pc
    Terminated,
}

pub fn run_program(instructions: &Vec<Instruction>) -> ExecutionResult {
    let mut pc: i16 = 0;
    let mut acc: i16 = 0;

    let mut visited_pcs = HashSet::new();

    loop {
        // Check for sucess
        if pc == instructions.len() as i16 {
            return ExecutionResult::Terminated;
        }
        // Check for duplicate
        if !visited_pcs.insert(pc) {
            return ExecutionResult::InfiniteLoop { acc };
        }

        // Fetch
        let (operation, argument) = instructions[usize::try_from(pc).expect("Invalid PC!")];
        // Execute
        match operation {
            Operation::ACC => {
                acc += argument;
                pc += 1;
            }
            Operation::JMP => {
                pc += argument;
            }
            Operation::NOP => {
                pc += 1;
            }
        }
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<Instruction>) -> Result<i16> {
    match run_program(input) {
        ExecutionResult::InfiniteLoop { acc } => Ok(acc),
        ExecutionResult::Terminated => Err(anyhow!("Not expected to terminate!")),
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<Instruction>) -> Result<usize> {
    todo!()
}
