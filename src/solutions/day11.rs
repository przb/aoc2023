use std::cmp::{max, min};
use crate::solutions::Solution;
use itertools::Itertools;

pub(crate) struct Day11;

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

fn get_galaxy_locs(input: &String) -> Vec<usize> {
    input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .filter_map(|(i, c)| (c == '#').then(|| i))
        .collect()
}

fn get_manhattan_distance(
    i: usize,
    j: usize,
    expanded_rows: &[usize],
    expanded_cols: &[usize],
    line_len: usize,
) -> i32 {
    let xi = (i % line_len) as i32;
    let yi = (i / line_len) as i32;
    let xj = (j % line_len) as i32;
    let yj = (j / line_len) as i32;

    let x_r = min(xi, xj)..max(xi, xj);
    let y_r = min(yi, yj)..max(yi, yj);

    let mut dist = i32::abs(xi - xj);
    dist += i32::abs(yi - yj);
    dist += expanded_rows
        .iter()
        .filter(|r| y_r.contains(&(**r as i32)))
        .count() as i32;
    dist += expanded_cols
        .iter()
        .filter(|c| x_r.contains(&(**c as i32)))
        .count() as i32;
    // println!("dist: {dist}");
    dist
}

impl Solution for Day11 {
    const DAY_NUM: i32 = 11;
    type ReturnType = i32;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();
//         let input = "...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#....."
//             .to_string();

        let line_len = input.find('\n').unwrap();
        let galaxies = get_galaxy_locs(&input);
        let expanded_rows = get_expanded_rows(&input);
        let expanded_cols = get_expanded_cols(&input);

        galaxies
            .iter()
            .tuple_combinations()
            .map(|(i, j)| get_manhattan_distance(*i, *j, &expanded_rows, &expanded_cols, line_len))
            .sum()
    }

    fn part_two(&self) -> Self::ReturnType {
        0
    }
}
