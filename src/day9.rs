use anyhow::Result;
use fxhash::FxHashSet;
use rayon::prelude::*;
use std::convert::TryFrom;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<u64> {
    input.split('\n').map(|line| line.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[u64]) -> Result<u64> {
    // Checking for sum by substracting each number
    // from the supposed sum and checking if the preable
    // contains the result. That reduces the load
    // from O(n*n) to nearly O(n) when using a HashSet.

    let preamble_length = 25; // Use 5 for the example

    // Fill up initial buffer
    let mut preamble_numbers = FxHashSet::default();
    input.iter().take(preamble_length).for_each(|num| {
        preamble_numbers.insert(num);
    });

    for i in preamble_length..input.len() {
        if i > preamble_length {
            preamble_numbers.remove(&input[i - preamble_length - 1]);
            preamble_numbers.insert(&input[i - 1]);
        }

        let supposed_sum = input[i];

        //println!("{} may be sum of {:?}", supposed_sum, preamble_numbers);

        let is_valid = preamble_numbers.iter().any(|other_num| preamble_numbers.contains(&(supposed_sum - *other_num)));
        if !is_valid {
            return Ok(supposed_sum);
        }
    }

    Err(anyhow!("No number found!"))
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[u64]) -> Result<u64> {
    todo!()
}
