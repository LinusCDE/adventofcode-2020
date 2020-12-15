use anyhow::Result;
use fxhash::FxHashSet;

const PREAMBLE_LENGTH: usize = 25; // Use 5 for the example

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<u64> {
    input.split('\n').map(|line| line.parse().unwrap()).collect()
}

pub fn find_invalid_sum_index(input: &[u64]) -> Option<usize> {
    // Checking for sum by substracting each number
    // from the supposed sum and checking if the preable
    // contains the result. That reduces the load
    // from O(n*n) to nearly O(n) when using a HashSet.

    // Fill up initial buffer
    let mut preamble_numbers = FxHashSet::default();
    input.iter().take(PREAMBLE_LENGTH).for_each(|num| {
        preamble_numbers.insert(num);
    });

    for i in PREAMBLE_LENGTH..input.len() {
        if i > PREAMBLE_LENGTH {
            preamble_numbers.remove(&input[i - PREAMBLE_LENGTH - 1]);
            preamble_numbers.insert(&input[i - 1]);
        }

        let supposed_sum = input[i];
        let is_valid = preamble_numbers.iter().any(|other_num| preamble_numbers.contains(&(supposed_sum - *other_num)));
        if !is_valid {
            return Some(i);
        }
    }

    None
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[u64]) -> Result<u64> {
    let index = find_invalid_sum_index(input).ok_or(anyhow!("No solution found!"))?;
    Ok(input[index])
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[u64]) -> Result<u64> {
    let index = find_invalid_sum_index(input).ok_or(anyhow!("No solution for part 1 found!"))?;
    let expected_sum = input[index];

    // Just adding up all numbers starting from each index
    // between 0 and the one of the found sum until it is
    // equal or bigger as the sum.
    // There is likely a more performant way. But this is
    // really fast on the puzzle input still.

    for i in 0..index {
        let mut sum = input[i] + input[i + 1];
        let mut offset = 1;
        while sum < expected_sum {
            offset += 1;
            sum += input[i + offset];
        }

        if sum == expected_sum {
            let mut elements = Vec::new();
            for o in 0..offset + 1 {
                elements.push(input[i + o])
            }

            elements.sort();
            return Ok(elements[0] + elements[elements.len() - 1]);
        }
    }

    Err(anyhow!("No solution found!"))
}
