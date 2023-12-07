use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use itertools::Itertools;

fn borders_symbol(input: &String, check_indexes: &[usize]) -> bool {
    let line_len = input.find('\n').unwrap() + 1;

    let prev_row_indexes = check_indexes.iter().filter_map(|v| v.checked_sub(line_len));
    let next_row_indexes = check_indexes.iter().filter_map(|v| {
        let check_index = v + line_len;
        if check_index > input.len() {
            None
        } else {
            Some(check_index)
        }
    });

    check_indexes
        .iter()
        .copied()
        .chain(prev_row_indexes)
        .chain(next_row_indexes)
        .any(|index| {
            let check_char = input.as_str().as_bytes()[index] as char;
            !check_char.is_ascii_digit() && check_char != '.' && check_char != '\n'
        })
}

pub(crate) fn part_one() -> i32 {
    let filename = PathBuf::from_str("inputs/03.txt").unwrap();
    let input = fs::read_to_string(filename).unwrap();
    let mut sum = 0;
    let mut i = 0;

    while i < input.len() {
        let mut char = input.as_str().as_bytes()[i] as char;
        let mut number = String::default();
        let mut check_indexes = vec![];

        if !char.is_ascii_digit() {
            i += 1;
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

        if borders_symbol(&input, &check_indexes) {
            sum += number.parse::<i32>().unwrap();
        }
        i += 1;
    }
    sum
}

fn take_forward_while_num(input: &String, mut index: usize) -> Option<i32> {
    let mut num = String::new();
    let mut c = input.as_bytes()[index] as char;
    while c.is_alphanumeric() {
        num.push(c);
        index += 1;
        c = input.as_bytes()[index] as char;
    }

    num.parse().ok()
}

fn take_backward_while_num(input: &String, mut index: usize) -> Option<i32> {
    let mut num = VecDeque::new();
    let mut c = input.as_bytes()[index] as char;
    while c.is_alphanumeric() {
        num.push_front(c);
        index -= 1;
        c = input.as_bytes()[index] as char;
    }

    num.iter().join("").parse().ok()
}

fn take_both_dir_while_num(input: &String, mut index: usize) -> Option<i32> {
    let mut num = String::new();
    let mut c = input.as_bytes()[index] as char;
    while c.is_alphanumeric() {
        index -= 1;
        c = input.as_bytes()[index] as char;
    }
    index += 1;
    c = input.as_bytes()[index] as char;
    while c.is_alphanumeric() {
        num.push(c);
        index += 1;
        c = input.as_bytes()[index] as char;
    }
    num.parse().ok()
}

fn adjacent_nums(input: &String, index: usize) {
    let mut nums = vec![];

    let line_len = input.find('\n').unwrap() + 1;

    let adj_squares = [
        index - 1 - line_len,
        index - line_len,
        index + 1 - line_len,
        index - 1,
        index + 1,
        index - 1 + line_len,
        index + line_len,
        index + 1 + line_len
    ];

    // let c = input.as_str().as_bytes()[adj_squares[0]] as char;

    let adj_chars = adj_squares
        .map(|i| input.as_str().as_bytes()[adj_squares[i]] as char);

    if adj_chars[1] == '.' {
        nums.push(take_backward_while_num(input, index - 1 - line_len));
        nums.push(take_forward_while_num(input, index + 1 - line_len));
    } else {

    }
    if adj_chars[6] == '.' {
        nums.push(take_backward_while_num(input, index - 1 + line_len));
        nums.push(take_backward_while_num(input, index + 1 + line_len));
    } else {

    }

    // all top are numbers AND (l is num OR r is num OR all bottom are num)
    // OR
    // all bottom are numbers AND (l is num OR r is num)
    // OR
    // left is num AND right is num AND top all . and bottom all .
    // OR
    // top left is num AND top middle is . AND top right is num AND all rest .
    // OR
    // bottom left is num AND bottom middle is . AND bottom right is num AND all rest .
    //
}


pub(crate) fn part_two() {
    let filename = PathBuf::from_str("inputs/03.txt").unwrap();
    let input = fs::read_to_string(filename).unwrap();
    let mut sum = 0;
    let mut i = 0;

    while i < input.len() {
        i += 1;
    }
}
