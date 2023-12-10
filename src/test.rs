use crate::solutions::{day01, day02, day03, day04, day05, day06, Solution};
use itertools::Itertools;
use prettytable::csv;
use std::fs::File;
use std::path::PathBuf;

fn nth_day_answers(day: usize) -> (u64, u64) {
    let file_path = PathBuf::from("answers.csv");
    let file = File::open(file_path).expect("csv answer file not found");
    let mut rdr = csv::Reader::from_reader(file);
    let r = rdr.records().nth(day - 1).unwrap().unwrap();

    let (_day, p1, p2) = r.iter().collect_tuple().unwrap();

    (p1.parse().unwrap(), p2.parse().unwrap())
}

#[test]
fn day_01() {
    let (p1, p2) = nth_day_answers(1);
    assert_eq!(p1, day01::Day01.part_one() as u64);
    assert_eq!(p2, day01::Day01.part_two() as u64);
}

#[test]
fn day_02() {
    let (p1, p2) = nth_day_answers(2);
    assert_eq!(p1, day02::Day02.part_one() as u64);
    assert_eq!(p2, day02::Day02.part_two() as u64);
}

#[test]
fn day_03() {
    let (p1, p2) = nth_day_answers(3);
    assert_eq!(p1, day03::Day03.part_one() as u64);
    assert_eq!(p2, day03::Day03.part_two() as u64);
}

#[test]
fn day_04() {
    let (p1, p2) = nth_day_answers(4);
    assert_eq!(p1, day04::Day04.part_one() as u64);
    assert_eq!(p2, day04::Day04.part_two() as u64);
}

#[test]
fn day_05() {
    let (p1, p2) = nth_day_answers(5);
    assert_eq!(p1, day05::Day05.part_one());
    assert_eq!(p2, day05::Day05.part_two());
}

#[test]
fn day_06() {
    let (p1, p2) = nth_day_answers(6);
    assert_eq!(p1, day06::Day06.part_one());
    assert_eq!(p2, day06::Day06.part_two());
}
