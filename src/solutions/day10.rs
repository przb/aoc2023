use std::cmp;
use std::collections::{HashMap, VecDeque};
use itertools::min;
use crate::solutions::Solution;
use crate::solutions::string_grid::find_line_len;

pub(crate) struct Day10;

/// given the starting row and col, perform a bfs until an index is found
fn find_loop(input: &String, starting_index: usize) -> i32 {
    let mut found_vals = HashMap::new();
    let line_len = find_line_len(input);

    let mut q = VecDeque::new();
    // imma be a bit cheaty since i know that the correct path from s is the right index and the row below
    let mut dist = 0;
    found_vals.insert(starting_index, dist);
    let (v1, v2) = (starting_index + 1, starting_index + line_len);
    q.push_front(v1);
    q.push_front(v2);
    while let Some(i) = q.pop_front() {
        let (v1, v2) = match *input.as_bytes().get(i).unwrap() as char {
            '|' => { (i - line_len, i + line_len) }
            '-' => { (i - 1, i + 1) }
            'J' => { (i - line_len, i - 1) }
            'L' => { (i - line_len, i + 1) }
            '7' => { (i - 1, i + line_len) }
            'F' => { (i + 1, i + line_len) }
            _ => panic!("uh-oh, whoopsie"),
        };
        if found_vals.contains_key(&v1) && found_vals.contains_key(&v2) {
            return cmp::max(*found_vals.get(&v1).unwrap(), *found_vals.get(&v2).unwrap()) + 1;
        } else if found_vals.contains_key(&v1) {
            q.push_back(v2);
            dist = *found_vals.get(&v1).unwrap();
        } else if found_vals.contains_key(&v2) {
            q.push_back(v1);
            dist = *found_vals.get(&v2).unwrap();
        } else {
            panic!("did not contain either");
        }
        found_vals.insert(i, dist + 1);
    }


    -1
}

impl Solution for Day10 {
    const DAY_NUM: i32 = 10;
    type ReturnType = i32;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();
        let s_index = input.find('S').unwrap();

        find_loop(&input, s_index)
    }

    fn part_two(&self) -> Self::ReturnType {
        0
    }
}
