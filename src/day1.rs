use anyhow::Result;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u16> {
    input.split('\n').map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u16]) -> Result<u32> {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            let i = input[i];
            let j = input[j];
            if i + j == 2020 {
                return Ok(i as u32 * j as u32);
            }
        }
    }

    bail!("No number found!");
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u16]) -> Result<u64> {
    for i in 0..input.len() {
        for j in 0..input.len() {
            for k in 0..input.len() {
                if i == j || j == k || i == k {
                    continue;
                }
                let i = input[i];
                let j = input[j];
                let k = input[k];
                if i + j + k == 2020 {
                    return Ok(i as u64 * j as u64 * k as u64);
                }
            }
        }
    }

    bail!("No number found!");
}
