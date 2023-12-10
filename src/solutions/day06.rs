use crate::solutions::Solution;
use itertools::Itertools;

pub(crate) struct Day06;

impl Solution for Day06 {
    const DAY_NUM: i32 = 6;
    type ReturnType = u64;

    fn part_one(&self) -> Self::ReturnType {
        let input = Day06.get_input();

        parse_line(&input, 0)
            .zip(parse_line(&input, 1))
            .map(|(time, record_distance)| calculate_ways_big(time, record_distance))
            .product()
    }

    fn part_two(&self) -> Self::ReturnType {
        let input = Day06.get_input();

        let (time, record_distance) = input
            .lines()
            .map(|l| {
                l.split_once(':')
                    .expect("error splitting lines")
                    .1
                    .replace(' ', "")
                    .parse::<u64>()
                    .expect("error parsing file")
            })
            .collect_tuple()
            .expect("error parsing tuple");

        calculate_ways_big(time, record_distance)
    }
}

fn parse_line(input: &String, line_num: usize) -> impl Iterator<Item = u64> + '_ {
    input
        .lines()
        .nth(line_num)
        .unwrap()
        .split_ascii_whitespace()
        .filter_map(|num| num.parse().ok())
}

#[allow(dead_code)]
fn calculate_ways(time: u64, record_distance: u64) -> u64 {
    let mut num_ways = 0;
    for speed in 1..time {
        let distance = speed * (time - speed);
        if distance > record_distance {
            num_ways += 1
        }
    }
    num_ways
}

fn calculate_ways_big(time: u64, record_distance: u64) -> u64 {
    let mut start_win = 0;
    let mut end_win = 0;
    for speed in 1..time {
        let distance = speed * (time - speed);
        if distance > record_distance {
            start_win = speed;
            break;
        }
    }

    for speed in (1..time).rev() {
        let distance = speed * (time - speed);
        if distance > record_distance {
            end_win = speed;
            break;
        }
    }
    end_win - start_win + 1
}
