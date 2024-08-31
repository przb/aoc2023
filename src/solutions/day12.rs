use crate::solutions::day12::SpringStatus::{Broken, Operational, Unknown};
use crate::solutions::Solution;
use itertools::Itertools;
use std::borrow::Borrow;
use std::collections::HashMap;

pub(crate) struct Day12;

const BROKEN_CHAR: char = '#';
const OPERATIONAL_CHAR: char = '.';
const UNKNOWN_CHAR: char = '?';

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SpringStatus {
    Operational,
    Broken,
    Unknown,
}

impl TryFrom<char> for SpringStatus {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            BROKEN_CHAR => Ok(Broken),
            OPERATIONAL_CHAR => Ok(Operational),
            UNKNOWN_CHAR => Ok(Unknown),
            _ => Err("Error Parsing String Status"),
        }
    }
}

fn line_matches_counts(status_line: &str, broken_counts: &Vec<i32>) -> bool {
    let broken_line = status_line
        .chars()
        .dedup_with_count()
        .filter(|(_count, character)| *character == BROKEN_CHAR)
        .collect_vec();
    broken_counts.len() == broken_line.len()
        && broken_line
            .into_iter()
            .zip(broken_counts)
            .all(|((count, _character), broken_count)| *broken_count as usize == count)
}

fn calculate_combos<'a>(
    status_line: &'a str,
    broken_counts: &Vec<i32>,
    map: &mut HashMap<Box<str>, i32>,
) -> i32 {
    let first = status_line;
    let val = map.get(first);
    if val.is_some() {
        println!("FOUND IN MAP");
        return *val.unwrap();
    }

    // if there are any unknowns
    if first.chars().any(|c| c == UNKNOWN_CHAR) {
        let (unknown_index, _) = first.chars().find_position(|c| *c == UNKNOWN_CHAR).unwrap();

        let mut line = first.to_string();
        line.replace_range(
            unknown_index..(unknown_index + 1),
            &OPERATIONAL_CHAR.to_string(),
        );
        let mut count = calculate_combos(line.borrow(), broken_counts, map);

        line.replace_range(unknown_index..(unknown_index + 1), &BROKEN_CHAR.to_string());
        count += calculate_combos(&line, broken_counts, map);

        map.insert(line.into(), count);
        count
    } else if line_matches_counts(first, broken_counts) {
        // otherwise there are no unknowns, so lets just check if its valid
        map.insert(first.into(), 1);
        1
    } else {
        // otherwise its not valid
        0
    }
}

impl Solution for Day12 {
    const DAY_NUM: i32 = 12;
    type ReturnType = i32;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();
        //        let input = "???.### 1,1,3
        //.??..??...?##. 1,1,3
        //?#?#?#?#?#?#?#? 1,3,1,6
        //????.#...#... 4,1,1
        //????.######..#####. 1,6,5
        //?###???????? 3,2,1"
        //            .to_string();
        //
        input
            .lines()
            .enumerate()
            .map(|(_c, l)| {
                let (springs, counts) = l.split_once(' ').unwrap();
                let counts: Vec<i32> = counts.split(',').map(|c| c.parse().unwrap()).collect();
                calculate_combos(springs, &counts, &mut HashMap::new())
            })
            .sum()
    }

    fn part_two(&self) -> Self::ReturnType {
        0
    }
}
