use itertools::Itertools;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) fn part_one() -> i32 {
    let filename = PathBuf::from_str("inputs/04.txt").unwrap();
    let input = fs::read_to_string(filename).unwrap();

    input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|game| game.split_once('|').unwrap())
        .map(|(winning_nums, my_nums)| {
            (
                winning_nums
                    .trim()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<i32>().unwrap()),
                my_nums
                    .trim()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<i32>().unwrap()),
            )
        })
        .map(|(win_nums, my_nums)| {
            my_nums.fold(0, |acc, val| {
                if win_nums.clone().contains(&val) {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                } else {
                    acc
                }
            })
        })
        .sum()
}

pub(crate) fn part_two() -> i32 {
    let filename = PathBuf::from_str("inputs/03.txt").unwrap();
    let input = fs::read_to_string(filename).unwrap();

    0
}

pub(crate) fn time_both() {
    let t1 = std::time::SystemTime::now();
    let _ = part_one();
    let t2 = std::time::SystemTime::now();
    println!("Day 4 part 1 took {:?}", t2.duration_since(t1).unwrap());

    let t1 = std::time::SystemTime::now();
    let _ = crate::solutions::day03::part_two();
    let t2 = std::time::SystemTime::now();
    println!("Day 4 part 2 took {:?}", t2.duration_since(t1).unwrap());
}
