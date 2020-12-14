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
            let instruction = match words[0] {
                "acc" => Operation::ACC,
                "jmp" => Operation::JMP,
                "nop" => Operation::NOP,
                _ => unreachable!("Unexpected instruction!"),
            };
            let argument = words[1].parse().expect("Failed to parse argument!");
            (instruction, argument)
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<Instruction>) -> i16 {
    let mut pc: i16 = 0;
    let mut acc: i16 = 0;

    let mut visited_pcs = HashSet::new();

    loop {
        // Check for duplicate
        if visited_pcs.contains(&pc) {
            return acc;
        }
        visited_pcs.insert(pc);

        // Fetch
        let (op, arg) = input[usize::try_from(pc).expect("Invalid PC!")];
        // Execute
        match op {
            Operation::ACC => {
                acc += arg;
                pc += 1;
            }
            Operation::JMP => {
                pc += arg;
            }
            Operation::NOP => {
                pc += 1;
            }
        }
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<Instruction>) -> Result<usize> {
    todo!()
}
