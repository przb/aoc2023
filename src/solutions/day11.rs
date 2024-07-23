use std::cmp::{max, min};
use crate::solutions::Solution;
use itertools::Itertools;

pub(crate) struct Day11;

struct Galaxy {
    // starting at zero from the top left
    x: i32,
    y: i32,
    index: usize,
    num: usize,
}

fn get_expanded_rows(input: &String) -> Vec<usize> {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, l)| l.chars().all(|c| c == '.').then(|| i))
        .collect()
}

fn get_expanded_cols(input: &String) -> Vec<usize> {
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

fn get_galaxies(input: &String, line_len: usize) -> Vec<Galaxy> {
    input
        .chars()
        .filter(|c| *c != '\n') // filter out the new line chars
        // this enumerate counts the position of the galaxy in the string (excluding newlines)
        .enumerate()
        .filter(|(_, c)|*c == '#') // only include the galaxy chars
        // this enumerate counts the number of galaxies
        .enumerate()
        .map(|(num, (index, c))| Galaxy {
            num,
            index,
            x: (index % line_len) as i32,
            y: (index / line_len) as i32,
        })
        .collect()
}

fn get_manhattan_distance(
    i: &Galaxy,
    j: &Galaxy,
    expanded_rows: &[usize],
    expanded_cols: &[usize],
) -> i32 {
    let x_r = min(i.x, j.x)..max(i.x, j.x);
    let y_r = min(i.y, j.y)..max(i.y, j.y);

    let mut dist = i32::abs(i.x - j.x);
    dist += i32::abs(i.y - j.y);
    dist += expanded_rows
        .iter()
        .filter(|row| y_r.contains(&(**row as i32)))
        .count() as i32;
    dist += expanded_cols
        .iter()
        .filter(|col| x_r.contains(&(**col as i32)))
        .count() as i32;
    // println!("dist: {dist}");
    dist
}

impl Solution for Day11 {
    const DAY_NUM: i32 = 11;
    type ReturnType = i32;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();
//        let input = "...#......
//.......#..
//#.........
//..........
//......#...
//.#........
//.........#
//..........
//.......#..
//#...#....."
//            .to_string();


        let line_len = input.find('\n').unwrap();
        let galaxies = get_galaxies(&input, line_len);
        let expanded_rows = get_expanded_rows(&input);
        let expanded_cols = get_expanded_cols(&input);

        galaxies
            .iter()
            .tuple_combinations()
            .map(|(i, j)| get_manhattan_distance(i, j, &expanded_rows, &expanded_cols))
            .sum()
    }

    fn part_two(&self) -> Self::ReturnType {
        0
    }
}
