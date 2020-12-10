use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

type Passport = HashMap<String, String>;

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Result<Vec<Passport>> {
    let mut passports: Vec<Passport> = Vec::new();

    for pairs in input.split("\n\n").map(|pairs| pairs.replace('\n', " ")) {
        let mut passport = Passport::new();
        for pair in pairs.split(' ') {
            ensure!(pair.matches(':').count() == 1, "Excactly one colon");
            let sp: Vec<_> = pair.split(':').collect();
            passport.insert(sp[0].to_owned(), sp[1].to_owned());
        }
        passports.push(passport);
    }

    Ok(passports)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<Passport>) -> usize {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid_passports = 0;
    for passport in input {
        let is_valid = (|| {
            for required_field in required_fields.iter() {
                if !passport.contains_key(*required_field) {
                    return false;
                }
            }
            true
        })();

        if is_valid {
            valid_passports += 1;
        }
    }

    valid_passports
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<Passport>) -> Result<usize> {
    todo!()
}
