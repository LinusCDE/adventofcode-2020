use anyhow::Result;

fn parse_binary_number(number: &str, lower: char, upper: char) -> Result<u8> {
    let binary_string = number.replace(lower, "0").replace(upper, "1");
    Ok(u8::from_str_radix(&binary_string, 2)?)
}

#[derive(Debug)]
pub struct Seat {
    row: u8,
    column: u8,
}

impl From<&str> for Seat {
    fn from(line: &str) -> Self {
        Self {
            row: parse_binary_number(&line[0..7], 'F', 'B').unwrap(),
            column: parse_binary_number(&line[7..10], 'L', 'R').unwrap(),
        }
    }
}

impl Seat {
    fn seat_id(&self) -> u16 {
        self.row as u16 * 8 + self.column as u16
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<Seat> {
    input.split('\n').map(|line| line.into()).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<Seat>) -> u16 {
    input.iter().map(|seat| seat.seat_id()).max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<Seat>) -> usize {
    todo!()
}
