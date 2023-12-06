use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use itertools::Itertools;

pub(crate) fn part_one() -> i32 {
    let filename = PathBuf::from_str("inputs/03.txt").unwrap();
    let input = fs::read_to_string(filename).unwrap();
    let line_len = input.find('\n').unwrap() + 1;

    let mut parts_nums = vec![];
    let mut i = 0;
    while i < input.len() {
        let mut char = input.as_str().as_bytes()[i] as char;
        let mut number = String::default();
        let mut check_indexes = vec![];

        if !char.is_ascii_digit() {
            i+=1;
            continue;
        }

        while char.is_ascii_digit() {
            check_indexes.push(i - 1);
            number.push(char);
            i += 1;
            char = input.as_str().as_bytes()[i] as char;
        }
        check_indexes.push(i - 1);
        check_indexes.push(i);

        let prev_row_indexes = check_indexes.iter()
            .filter_map(|v| v.checked_sub(line_len));
        let next_row_indexes = check_indexes.iter()
            .filter_map(|v| {
                let check_index = v + line_len;
                if check_index > input.len() { None } else { Some(check_index) }
            });
        let borders_symbol = check_indexes.iter().copied()
            .chain(prev_row_indexes)
            .chain(next_row_indexes)
            .any(|index| {
                let check_char = input.as_str().as_bytes()[index] as char;
                !check_char.is_ascii_digit() && check_char != '.' && check_char != '\n'
            });

        if borders_symbol {
            parts_nums.push(number.parse::<i32>().unwrap());
        }
        i+=1;
    }
    println!("first len {}", parts_nums.len());
    // println!("deduped len {}", parts_nums.iter().dedup().count());
    parts_nums.iter().sum()
}

pub(crate) fn part_two() {}
