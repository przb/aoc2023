pub(crate) struct Day09;

use itertools::Itertools;
use crate::solutions::Solution;

fn get_iters(input: &str) -> impl DoubleEndedIterator<Item=impl DoubleEndedIterator<Item=i32> + '_> + '_ {
    let lines = input.lines();
    lines.map(|l| {
        l.split_ascii_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
    })
}

fn next_history(row: &[i32]) -> i32 {
    let last = *row.last().unwrap();
    let collapsed_row = row.iter().tuple_windows().map(|(l, r)| r - l).collect_vec();
    if collapsed_row.iter().any(|i| *i != 0) {
        last + next_history(&collapsed_row)
    } else {
        last
    }
}
fn prev_history(row: &[i32]) -> i32 {
    let first = *row.first().unwrap();
    let collapsed_row = row.iter().tuple_windows().map(|(l, r)| r - l).collect_vec();
    if collapsed_row.iter().any(|i| *i != 0) {
        first - prev_history(&collapsed_row)
    } else {
        first
    }
}

impl Solution for Day09 {
    const DAY_NUM: i32 = 9;
    type ReturnType = i32;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();
        get_iters(&input)
            .map(|row| next_history(&row.collect_vec()))
            .sum()
    }

    fn part_two(&self) -> Self::ReturnType {
        let input = self.get_input();
        get_iters(&input)
            .map(|row| prev_history(&row.collect_vec()))
            .sum()
    }
}
