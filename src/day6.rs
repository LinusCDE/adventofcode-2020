use anyhow::Result;
use std::collections::{HashMap, HashSet};

type GroupAnswers = HashMap<char /*Question*/, HashSet<u8> /*Per group person numbers who answered yes */>;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<GroupAnswers> {
    let mut group_answers_list: Vec<GroupAnswers> = Vec::new();

    for group_lines in input.split("\n\n") {
        let mut answered_yes_by: GroupAnswers = HashMap::new();
        let mut person_num = 0;
        for line in group_lines.split('\n') {
            person_num += 1;
            for ch in line.chars() {
                answered_yes_by.entry(ch).or_default().insert(person_num);
            }
        }

        group_answers_list.push(answered_yes_by);
    }

    group_answers_list
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<GroupAnswers>) -> usize {
    input.iter().map(|ga| ga.len()).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<GroupAnswers>) -> Result<u16> {
    todo!()
}
