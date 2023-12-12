use crate::solutions::Solution;
use std::collections::BTreeMap;

// struct TreeNode {
//     data: String,
//     l: Box<Option<TreeNode>>,
//     r: Box<Option<TreeNode>>,
// }
pub(crate) struct Day08;

impl Solution for Day08 {
    const DAY_NUM: i32 = 8;
    type ReturnType = u32;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();

        let mut lines = input.lines();
        let navigation_seq = lines.next().unwrap();
        lines.next();

        let mut mappings = BTreeMap::new();
        lines.filter(|l| !l.is_empty()).for_each(|line| {
            let (parent, children) = line.split_once('=').unwrap();
            let children = children
                .trim()
                .strip_prefix('(')
                .expect("no '(' found")
                .strip_suffix(')')
                .expect("no ')' found")
                .split_once(',')
                .expect("no ',' found");
            let children = (children.0.trim(), children.1.trim());
            mappings.insert(parent.trim(), children);
        });

        let mut current = "AAA";
        let mut directions = navigation_seq.chars().cycle();
        let mut steps = 0;
        while current != "ZZZ" {
            current = match directions.next().unwrap() {
                'L' => mappings.get(current).expect("current string not in tree").0,
                'R' => mappings.get(current).expect("current string not in tree").1,
                _ => panic!("direction is not 'L' nor 'R'"),
            };
            steps += 1;
        }

        steps
    }

    fn part_two(&self) -> Self::ReturnType {
        todo!()
    }
}
