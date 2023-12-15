use crate::solutions::Solution;
use std::collections::{HashMap};
use std::fmt::{Display};


pub(crate) struct Day08;

fn create_mapping<'a, I: Iterator<Item=&'a str>>(input_lines: I) -> (HashMap<&'a str, (&'a str, &'a str)>, Vec<&'a str>) {
    let mut mappings = HashMap::new();
    let mut startings = vec![];
    input_lines.filter(|l| !l.is_empty())
        .for_each(|line| {
            let (parent, children) = line.split_once('=').unwrap();
            let parent = parent.trim();
            if parent.ends_with('A') {
                startings.push(parent);
            }
            let children = children
                .trim()
                .strip_prefix('(')
                .expect("no '(' found")
                .strip_suffix(')')
                .expect("no ')' found")
                .split_once(',')
                .expect("no ',' found");
            let children = (children.0.trim(), children.1.trim());
            mappings.insert(parent, children);
        });

    (mappings, startings)
}

fn update_current_nodes<'a>(nodes: &mut Vec<&'a str>, mappings: &HashMap<&'a str, (&'a str, &'a str)>, direction: char) {
    for current in nodes.iter_mut() {
        *current = match direction {
            'L' => mappings.get(current).expect("current string not in tree").0,
            'R' => mappings.get(current).expect("current string not in tree").1,
            _ => panic!("direction is not 'L' nor 'R'"),
        };
    }
}


impl Solution for Day08 {
    const DAY_NUM: i32 = 8;
    type ReturnType = u32;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();
        let mut lines = input.lines();
        let navigation_seq = lines.next().unwrap();
        lines.next();
        let (mappings, _) = create_mapping(lines);
        let mut current = vec!["AAA"];
        let mut directions = navigation_seq.chars().cycle();
        let mut steps = 0;
        while current.iter().any(|n| *n != "ZZZ") {
            update_current_nodes(&mut current, &mappings, directions.next().unwrap());
            steps += 1;
        }

        steps
    }

    fn part_two(&self) -> Self::ReturnType {
        let input = self.get_input();
        let mut lines = input.lines();
        let mut steps = 0;
        let navigation_seq = lines.next().unwrap();
        lines.next();
        let (mappings, mut nodes) = create_mapping(lines);
        let mut directions = navigation_seq.chars().cycle();
        while !nodes.iter().all(|s| s.ends_with('Z')) {
            let next_dir = directions.next().unwrap();
            let mut f = nodes[0].eq("VGA");
            // if f { println!("loop!"); } else { eprintln!("{}", nodes[0]); }
            update_current_nodes(&mut nodes, &mappings, next_dir);
            steps += 1;
        }

        steps
    }
}
