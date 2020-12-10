use anyhow::Result;
use regex::Regex;

pub struct Map {
    data_width: usize,
    /// True is a tree (#), false not (.)
    data: Vec<Vec<bool>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let lines: Vec<_> = input.split('\n').collect();
        // Expect all lines to have this length
        let data_width = lines[0].len();
        let mut data = Vec::new();

        for line in lines {
            data.push(
                line.chars()
                    .into_iter()
                    .map(|ch| match ch {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Invalid char encountered: {}", ch),
                    })
                    .collect(),
            );
        }

        Self { data, data_width }
    }
}

impl Map {
    /// Origin is top left
    pub fn is_tree(&self, x: usize, y: usize) -> bool {
        self.data[y][x % self.data_width]
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn count_trees(&self, slope_vector: (usize, usize)) -> Result<usize> {
        ensure!(slope_vector.1 > 0, "This slope will actually end");
        let mut tree_count = 0;
        let mut x = 0;
        let mut y = 0;
        while y < self.height() {
            if self.is_tree(x, y) {
                tree_count += 1;
            }

            x += slope_vector.0;
            y += slope_vector.1;
        }

        Ok(tree_count)
    }
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Map {
    input.into()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Map) -> Result<usize> {
    input.count_trees((3, 1))
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Map) -> Result<usize> {
    let mut result = 1;
    for slope_vector in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        result *= input.count_trees(*slope_vector)?;
    }

    Ok(result)
}
