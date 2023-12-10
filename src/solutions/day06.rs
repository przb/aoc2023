use crate::solutions::Solution;

pub(crate) struct Day06;

impl Solution for Day06 {
    const DAY_NUM: i32 = 6;
    type ReturnType = u32;

    fn part_one(&self) -> Self::ReturnType {
        let input = Day06.get_input();

        parse_line(&input, 0)
            .zip(parse_line(&input, 1))
            .map(|(time, record_distance)| {
                let mut num_ways_to_win = 0;
                for speed in 1..time {
                    let distance = speed * (time - speed);
                    if distance > record_distance {
                        num_ways_to_win += 1
                    }
                }
                num_ways_to_win
            })
            .product()
    }

    fn part_two(&self) -> Self::ReturnType {
        0
    }
}

fn parse_line(input: &String, line_num: usize) -> impl Iterator<Item = u32> + '_ {
    input
        .lines()
        .nth(line_num)
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|num| num.parse().ok())
}
