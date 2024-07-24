use crate::solutions::day12::SpringStatus::{Broken, Operational, Unknown};
use crate::solutions::Solution;
use itertools::Itertools;
use std::str::FromStr;

pub(crate) struct Day12;

#[derive(Debug)]
enum SpringStatus {
    Operational,
    Broken,
    Unknown,
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

impl Solution for Day12 {
    const DAY_NUM: i32 = 12;
    type ReturnType = i32;

    fn part_one(&self) -> Self::ReturnType {
        // let input = self.get_input();
        let input = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1"
            .to_string();

        let springs = input
            .lines()
            .flat_map(|l| SpringRow::from_str(l))
            .collect_vec();
        println!("{:?}", springs);

        0
    }

    fn part_two(&self) -> Self::ReturnType {
        0
    }
}
