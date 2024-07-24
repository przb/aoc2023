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
        let input = self.get_input();
        let starting_index = input.find('S').unwrap();
        let mut path = HashMap::new();
        mark_path(&input, starting_index, &mut path);
        let _line_len = find_line_len(&input);

        // So i know what the main path is
        // Now i need to count all the squares that are contained in the path
        // To do this, i will use a ray casting algorithm on each line in the string
        // something like (pseudocode):

        // num inside <- 0
        //
        // for each line l
        //      num intersections <- 0
        //      for each char c
        //          if the char c is contained in the path,
        //              if the line is just tangent, do not increment the number of intersections
        //              if the c ends by going up or down (such as a '|', 'J', or '7', increment the num intersections
        //              if the c ends by going left or right ('-', 'F', or 'L'), dont increment the number of intersections until the end (ie the same conditions as above)
        //              if the c is '.' dont increment the number of intersections
        //              todo figure out how to handle the starting point S
        //          else if the char is not contained in the path,
        //          if the current num intersections is even, then do not increment the number of points inside the shape
        //          if the current num intersections is od, then increment the number of points inside the shape

        let mut num_intersections = 0;
        let mut num_inside = 0;
        let mut could_be_tangent = false;
        let mut char_to_make_tangent = '0'; // just some random char for now
        for (i, c) in input.chars().enumerate() {
            // the char c is contained in the path
            if path.contains_key(&i) {
                match c {
                    // in my specific puzzle, the S acts as an F
                    'F' | 'S' => {
                        could_be_tangent = true;
                        char_to_make_tangent = '7';
                    }
                    'L' => {
                        could_be_tangent = true;
                        char_to_make_tangent = 'J';
                    }
                    '|' => {
                        num_intersections += 1;
                        could_be_tangent = false;
                    }
                    'J' => {
                        if could_be_tangent && char_to_make_tangent == c {
                            num_intersections += 0;
                        } else {
                            num_intersections += 1;
                        }
                        could_be_tangent = false;
                    }
                    '7' => {
                        if could_be_tangent && char_to_make_tangent == c {
                            num_intersections += 0;
                        } else {
                            num_intersections += 1;
                        }
                        could_be_tangent = false;
                    }
                    _ => {}
                }
            } else if c == '\n' {
                // reset
                num_intersections = 0;
                could_be_tangent = false;
            }
            // the char is not contained in the path
            else {
                if num_intersections % 2 == 1 {
                    // point is inside the shape
                    num_inside += 1;
                }
                // else {
                //     // point is not inside
                //     num_inside += 0
                // }
            }
        }

        num_inside
    }
}
