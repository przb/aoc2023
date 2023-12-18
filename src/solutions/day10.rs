use crate::solutions::string_grid::find_line_len;
use crate::solutions::Solution;
use std::cmp;
use std::collections::{HashMap, VecDeque};

pub(crate) struct Day10;

fn mark_path(input: &String, starting_index: usize, path_indexes: &mut HashMap<usize, i32>) {
    let line_len = find_line_len(input);
    let mut q = VecDeque::new();
    // imma be a bit cheaty since i know that the correct path from s is the right index and the row below
    let mut dist = 0;
    path_indexes.insert(starting_index, dist);
    let (v1, v2) = (starting_index + 1, starting_index + line_len);
    q.push_front(v1);
    q.push_front(v2);
    while let Some(i) = q.pop_front() {
        let (v1, v2) = match *input.as_bytes().get(i).unwrap() as char {
            '|' => (i - line_len, i + line_len),
            '-' => (i - 1, i + 1),
            'J' => (i - line_len, i - 1),
            'L' => (i - line_len, i + 1),
            '7' => (i - 1, i + line_len),
            'F' => (i + 1, i + line_len),
            _ => panic!("uh-oh, whoopsie"),
        };
        if path_indexes.contains_key(&v1) && path_indexes.contains_key(&v2) {
            dist = cmp::max(
                *path_indexes.get(&v1).unwrap(),
                *path_indexes.get(&v2).unwrap(),
            );
        } else if path_indexes.contains_key(&v1) {
            q.push_back(v2);
            dist = *path_indexes.get(&v1).unwrap();
        } else if path_indexes.contains_key(&v2) {
            q.push_back(v1);
            dist = *path_indexes.get(&v2).unwrap();
        } else {
            return;
        }
        path_indexes.insert(i, dist + 1);
    }
}

/// given the starting row and col, perform a bfs until an index is found
fn find_loop_max_dist(input: &String, starting_index: usize) -> i32 {
    let mut path = HashMap::new();
    mark_path(input, starting_index, &mut path);

    *path.values().max().unwrap()
}

impl Solution for Day10 {
    const DAY_NUM: i32 = 10;
    type ReturnType = i32;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();
        let s_index = input.find('S').unwrap();

        find_loop_max_dist(&input, s_index)
    }

    fn part_two(&self) -> Self::ReturnType {
        0
    }
}
