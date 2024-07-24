use crate::solutions::Solution;
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::cmp::{max, min};
use std::slice::Iter;

pub(crate) struct Day11;

struct Galaxy {
    // starting at zero from the top left
    x: i32,
    y: i32,
}

fn get_expanded_rows(input: &String) -> ArrayVec<usize, 20> {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, l)| l.chars().all(|c| c == '.').then(|| i))
        .collect()
}

fn get_expanded_cols(input: &String) -> ArrayVec<usize, 20> {
    let line_len = input.find('\n').unwrap();
    input
        .chars()
        .take_while(|c| *c != '\n')
        .enumerate()
        .filter_map(|(i, _)| {
            input
                .chars()
                .skip(i)
                .step_by(line_len + 1)
                .all(|c| c == '.')
                .then(|| i)
        })
        .collect()
}

fn get_galaxies(input: &String, line_len: usize) -> ArrayVec<Galaxy, 1000> {
    input
        .chars()
        .filter(|c| *c != '\n') // filter out the new line chars
        // this enumerate counts the position of the galaxy in the string (excluding newlines)
        .enumerate()
        .filter(|(_, c)| *c == '#') // only include the galaxy chars
        // this enumerate counts the number of galaxies
        .enumerate()
        .map(|(_num, (index, _c))| Galaxy {
            x: (index % line_len) as i32,
            y: (index / line_len) as i32,
        })
        .collect()
}

fn get_manhattan_distance(
    i: &Galaxy,
    j: &Galaxy,
    expanded_rows: Iter<usize>,
    expanded_cols: Iter<usize>,
    expansion_factor: i64,
) -> i64 {
    let x_r = min(i.x, j.x)..max(i.x, j.x);
    let y_r = min(i.y, j.y)..max(i.y, j.y);

    let mut dist = i64::abs((i.x - j.x) as i64);
    dist += i64::abs((i.y - j.y) as i64);
    let rows = expanded_rows
        .filter(|row| y_r.contains(&(**row as i32)))
        .count() as i64;
    let cols = expanded_cols
        .filter(|col| x_r.contains(&(**col as i32)))
        .count() as i64;
    dist + (rows + cols) * (expansion_factor - 1)
}

impl Solution for Day11 {
    const DAY_NUM: i32 = 11;
    type ReturnType = i64;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();

        let line_len = input.find('\n').unwrap();
        let galaxies = get_galaxies(&input, line_len);
        let expanded_rows = get_expanded_rows(&input);
        let expanded_cols = get_expanded_cols(&input);

        galaxies
            .iter()
            .tuple_combinations()
            .map(|(i, j)| {
                get_manhattan_distance(i, j, expanded_rows.iter(), expanded_cols.iter(), 2)
            })
            .sum()
    }

    fn part_two(&self) -> Self::ReturnType {
        let input = self.get_input();

        let line_len = input.find('\n').unwrap();
        let galaxies = get_galaxies(&input, line_len);
        let expanded_rows = get_expanded_rows(&input);
        let expanded_cols = get_expanded_cols(&input);

        galaxies
            .iter()
            .tuple_combinations()
            .map(|(i, j)| {
                get_manhattan_distance(i, j, expanded_rows.iter(), expanded_cols.iter(), 1_000_000)
            })
            .sum()
    }
}
