use anyhow::{Context, Result};
use fxhash::FxHashMap;
use regex::Regex;

type Passport = FxHashMap<String, String>;

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Result<Vec<Passport>> {
    let mut passports: Vec<Passport> = Vec::new();

    for pairs in input.split("\n\n").map(|pairs| pairs.replace('\n', " ")) {
        let mut passport = Passport::default();
        for pair in pairs.split(' ') {
            ensure!(pair.matches(':').count() == 1, "Excactly one colon");
            let sp: Vec<_> = pair.split(':').collect();
            passport.insert(sp[0].to_owned(), sp[1].to_owned());
        }
        passports.push(passport);
    }

    Ok(passports)
}

pub fn has_required_fields(passport: &Passport) -> bool {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for required_field in required_fields.iter() {
        if !passport.contains_key(*required_field) {
            return false;
        }
    }
    true
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Passport]) -> usize {
    input.iter().filter(|passport| has_required_fields(passport)).count()
}

pub fn validate_entries(passport: &Passport) -> Result<()> {
    ensure!(has_required_fields(passport), "Required fields are present");
    lazy_static! {
        static ref HCL_PATTERN: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
        static ref PID_PATTERN: Regex = Regex::new("^[0-9]{9}$").unwrap();
        static ref VALID_ECLS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    }

    let byr: i32 = passport["byr"].parse().context("Convert BYR to number")?;
    ensure!(byr >= 1920 && byr <= 2002, "BYR is between 1920 and 2002");

    let iyr: i32 = passport["iyr"].parse().context("Convert IYR to number")?;
    ensure!(iyr >= 2010 && iyr <= 2020, "IYR is between 2010 and 2020");

    let eyr: i32 = passport["eyr"].parse().context("Convert EYR to number")?;
    ensure!(eyr >= 2020 && eyr <= 2030, "EYR is between 2020 and 2030");

    let hgt_str = &passport["hgt"];
    ensure!(hgt_str.ends_with("cm") || hgt_str.ends_with("in"), "Height is valid",);
    let hgt: i32 = hgt_str[0..hgt_str.len() - 2].parse().context("Convert HGT to number")?;
    if hgt_str.ends_with("cm") {
        ensure!(hgt >= 150 && hgt <= 193, "HGT is between 150cm and 193cm");
    } else {
        ensure!(hgt >= 59 && hgt <= 76, "HGT is between 59in and 76in");
    }

    ensure!(VALID_ECLS.iter().any(|ecl| *ecl == passport["ecl"]), "ECL is valid");

    ensure!(HCL_PATTERN.is_match(&passport["hcl"]), "HCL is a valid color format");

    ensure!(PID_PATTERN.is_match(&passport["pid"]), "PID is a valid 9 digit number");

    Ok(())
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Passport]) -> usize {
    input.iter().filter(|passport| validate_entries(passport).is_ok()).count()
}
