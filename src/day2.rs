use regex::Regex;

#[derive(Debug)]
pub struct Entry {
    password: String,
    letter: char,
    numbers: (usize, usize),
}

impl From<&str> for Entry {
    fn from(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new("([0-9]*)-([0-9]*) ([A-Za-z]): (.*)").unwrap();
        }
        let caps = RE.captures(line).unwrap();

        return Entry {
            numbers: (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
            letter: caps[3].parse().unwrap(),
            password: caps[4].to_owned(),
        };
    }
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Entry> {
    input.split('\n').map(|line| line.into()).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Entry]) -> u16 {
    let mut valid_passwords = 0;

    for entry in input {
        let count = entry.password.matches(entry.letter).count();
        if count >= entry.numbers.0 && count <= entry.numbers.1 {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Entry]) -> u16 {
    let mut valid_passwords = 0;

    for entry in input {
        let chars = entry.password.chars().collect::<Vec<_>>();
        let first_valid = chars[entry.numbers.0 - 1] == entry.letter;
        let second_valid = chars[entry.numbers.1 - 1] == entry.letter;

        if first_valid && !second_valid || !first_valid && second_valid {
            valid_passwords += 1;
        }
    }

    valid_passwords
}
