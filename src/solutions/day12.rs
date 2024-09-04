use crate::solutions::day12::SpringStatus::{Broken, Operational, Unknown};
use crate::solutions::Solution;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;
use std::borrow::Borrow;

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

fn calculate_combos<'a>(status_line: &'a str, broken_counts: &Vec<i32>) -> i32 {
    let first = status_line;

    // if there are any unknowns
    if first.chars().any(|c| c == UNKNOWN_CHAR) {
        let (unknown_index, _) = first.chars().find_position(|c| *c == UNKNOWN_CHAR).unwrap();

        let mut line = first.to_string();
        line.replace_range(
            unknown_index..(unknown_index + 1),
            &OPERATIONAL_CHAR.to_string(),
        );
        let mut count = calculate_combos(line.borrow(), broken_counts);

        line.replace_range(unknown_index..(unknown_index + 1), &BROKEN_CHAR.to_string());
        count += calculate_combos(&line, broken_counts);

        count
    } else if line_matches_counts(first, broken_counts) {
        // otherwise there are no unknowns, so lets just check if its valid
        1
    } else {
        // otherwise its not valid
        0
    }
}

fn divide_and_conquerer(line: &str, index: usize, broken_counts: &Vec<i32>) -> i32 {
    // base case :
    // "." returns 0
    // "#" returns 1
    //
    // recurisve cases:
    // if the new char is "."
    //      return the count of the rest of the string
    // if the new char is "#"
    //      if the second char is "."
    //          // there is a new group
    //          return 1 + the rest of the string
    //      if the second char is "#"
    //          return the rest of the string

    if index == line.len() - 1 {
        let current_char = line.as_bytes()[index];
        if current_char == OPERATIONAL_CHAR as u8 {
            return 0;
        } else {
            return 1;
        }
    }

    let current_char = line.as_bytes()[index];
    if current_char == OPERATIONAL_CHAR as u8 {
        return divide_and_conquerer(line, index + 1, broken_counts);
    } else if current_char == BROKEN_CHAR as u8 {
        let second_char = line.as_bytes()[index + 1];
        if second_char == OPERATIONAL_CHAR as u8 {
            return 1 + divide_and_conquerer(line, index + 1, broken_counts);
        } else if second_char == BROKEN_CHAR as u8 {
            return divide_and_conquerer(line, index + 1, broken_counts);
        } else {
            println!("error");
            return 0;
        }
    } else {
        return 0;
    }
}

impl Solution for Day12 {
    const DAY_NUM: i32 = 12;
    type ReturnType = i32;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();
        for line in input.lines() {
            

        }
        0
    }

    fn part_two(&self) -> Self::ReturnType {
        let input = self.get_input();
        
        0
    }
}
