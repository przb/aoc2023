use crate::solutions::day12::SpringStatus::{Broken, Operational, Unknown};
use crate::solutions::Solution;
use itertools::Itertools;
use std::str::FromStr;
use rayon::prelude::ParallelString;
use rayon::prelude::ParallelIterator;

pub(crate) struct Day12;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SpringStatus {
    Operational,
    Broken,
    Unknown,
}

struct UnknownGroup {
    index_start: usize,
    size: usize,
    left: Option<SpringStatus>,
    right: Option<SpringStatus>,
}

#[derive(Debug)]
struct SpringRow {
    statuses: Vec<SpringStatus>,
    damaged_groups: Vec<usize>,
}

impl TryFrom<char> for SpringStatus {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Broken),
            '.' => Ok(Operational),
            '?' => Ok(Unknown),
            _ => Err("Error Parsing String Status"),
        }
    }
}

impl FromStr for SpringRow {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (statuses, groups) = s.split_once(' ').ok_or("No valid whitespace separator")?;
        let statuses = statuses.chars().flat_map(|c| c.try_into()).collect();
        let groups = groups.split(',').flat_map(|s| s.parse()).collect();
        Ok(SpringRow {
            statuses,
            damaged_groups: groups,
        })
    }
}

fn completed_row<'a>(row: &str) -> bool {
    // no unknowns
    if let Some((springs, counts)) = row.split_once(' ') {
        let counts: Vec<usize> = counts.split(',').map(|c| c.parse().unwrap()).collect();
        springs.chars().all(|c| c == '#' || c == '.')
            && springs
                .chars()
                .dedup_with_count()
                .filter_map(|(count, c)| (c == '#').then(|| count))
                .zip(counts.iter())
                .all(|(l, r)| l == *r)
    } else {
        false
    }
}

fn count_line_combinations(invalid_row: &str) -> i32 {
    if completed_row(invalid_row) {
        1
    } else if invalid_row.chars().all(|c| c != '?') {
        0
    } else {
        let mut total = 0;
        for (i, c) in invalid_row.chars().enumerate() {
            if c == '?' {
                let mut new_operational = invalid_row.to_string();
                new_operational.replace_range(i..=i, ".");
                let mut new_broken = invalid_row.to_string();
                new_broken.replace_range(i..=i, "#");
                
                total += count_line_combinations(new_operational.as_str())
                    + count_line_combinations(new_broken.as_str());
            }
        }
        total
    }
}

impl Solution for Day12 {
    const DAY_NUM: i32 = 12;
    type ReturnType = i32;

    fn part_one(&self) -> Self::ReturnType {
        // let input = self.get_input();
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            .to_string();

        input.par_lines().map(|l| count_line_combinations(l)).sum()
    }

    fn part_two(&self) -> Self::ReturnType {
        0
    }
}
