use fxhash::{FxHashMap, FxHashSet};

type GroupAnswers = FxHashMap<char /*Question*/, FxHashSet<u8> /*Per group person numbers who answered yes */>;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<GroupAnswers> {
    let mut group_answers_list: Vec<GroupAnswers> = Vec::new();

    for group_lines in input.split("\n\n") {
        let mut answered_yes_by: GroupAnswers = FxHashMap::default();
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
pub fn solve_part1(input: &[GroupAnswers]) -> usize {
    input.iter().map(|ga| ga.len()).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[GroupAnswers]) -> usize {
    input
        .iter()
        .map(|ga| {
            // The first person in a group has the number one.
            // Ergo the person count is the highest person_number.
            let person_count = *ga.values().flatten().max().unwrap() as usize;
            // Count all answer sets, where the size is equal to the total persons
            ga.values().filter(|set| set.len() == person_count).count()
        })
        .sum()
}
