use crate::solutions::Solution;
use itertools::Itertools;
use num::integer::lcm;
use std::collections::HashMap;

pub(crate) struct Day08;

fn get_parent(line: &str) -> &str {
    let (parent, _) = line.split_once('=').unwrap();
    parent.trim()
}

fn get_children(line: &str) -> (&str, &str) {
    let (_, children) = line.split_once('=').unwrap();

    let children = children
        .trim()
        .strip_prefix('(')
        .expect("no '(' found")
        .strip_suffix(')')
        .expect("no ')' found")
        .split_once(',')
        .expect("no ',' found");

    (children.0.trim(), children.1.trim())
}

fn create_mapping<'a, I: Iterator<Item = &'a str>>(
    input_lines: I,
) -> (HashMap<&'a str, (&'a str, &'a str)>, Vec<&'a str>) {
    let mut mappings = HashMap::new();
    let mut startings = vec![];
    input_lines.filter(|l| !l.is_empty()).for_each(|line| {
        let parent = get_parent(line);
        if parent.ends_with('A') {
            startings.push(parent);
        }
        let children = get_children(line);
        mappings.insert(parent, children);
    });

    (mappings, startings)
}

fn update_current_nodes<'a>(
    nodes: &mut Vec<&'a str>,
    mappings: &HashMap<&'a str, (&'a str, &'a str)>,
    direction: char,
) {
    for current in nodes.iter_mut() {
        *current = match direction {
            'L' => mappings.get(current).expect("current string not in tree").0,
            'R' => mappings.get(current).expect("current string not in tree").1,
            _ => panic!("direction is not 'L' nor 'R'"),
        };
    }
}

fn find_steps<'a, F: FnMut(&&str) -> bool>(
    navigation_seq: &str,
    current: &mut Vec<&'a str>,
    mappings: &HashMap<&'a str, (&'a str, &'a str)>,
    mut end_cond: F,
) -> u32 {
    let mut directions = navigation_seq.chars().cycle();
    let mut steps = 0;
    while current.iter().any(|n| !end_cond(n)) {
        update_current_nodes(current, &mappings, directions.next().unwrap());
        steps += 1;
    }

    steps
}

impl Solution for Day08 {
    const DAY_NUM: i32 = 8;
    type ReturnType = u64;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();
        let mut lines = input.lines();
        let navigation_seq = lines.next().unwrap();
        lines.next();
        let (mappings, _) = create_mapping(lines);
        let mut current = vec!["AAA"];
        find_steps(navigation_seq, &mut current, &mappings, |n| *n == "ZZZ").into()
    }

    fn part_two(&self) -> Self::ReturnType {
        let input = self.get_input();
        let mut lines = input.lines();
        let navigation_seq = lines.next().unwrap();
        lines.next();
        let (mappings, nodes) = create_mapping(lines);
        let counts = nodes
            .iter()
            .map(|n| {
                find_steps(navigation_seq, &mut vec![*n], &mappings, |n| {
                    n.ends_with('Z')
                })
            })
            .collect_vec();
        counts.iter().fold(1, |acc, r| lcm(acc, *r as u64))
    }
}
