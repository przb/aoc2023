use crate::solutions::Solution;

pub(crate) struct Day10;

fn find_loop(input: &String, row: usize, col: usize, line_len: usize) {
    
}

impl Solution for Day10 {
    const DAY_NUM: i32 = 10;
    type ReturnType = i32;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();
        let len = input.find('\n').unwrap();
        let s_index = input.find('S').unwrap();
        let row = s_index % len;
        let col = s_index / len;

        find_loop(&input, row, col, len);

        0
    }

    fn part_two(&self) -> Self::ReturnType {
        0
    }
}
