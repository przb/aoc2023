use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

const NUM_TO_MAKE_GEAR: usize = 2;

fn find_line_len(input: &String) -> usize {
    input.find('\n').unwrap() + 1
}

fn borders_symbol(input: &String, check_indexes: &[usize]) -> bool {
    let line_len = find_line_len(input);

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

fn row_nums(input: &String, idx: usize, nums: &mut Vec<i32>) {
    let c = input.as_bytes()[idx] as char;
    if c == '.' {
        if let Some(num) = take_backward_while_num(input, idx - 1) {
            nums.push(num)
        }
        if let Some(num) = take_forward_while_num(input, idx + 1) {
            nums.push(num)
        }
    } else if let Some(num) = take_both_dir_while_num(input, idx) {
        nums.push(num)
    }
}

fn adjacent_nums(input: &String, index: usize) -> Vec<i32> {
    let mut nums = vec![];
    let line_len = find_line_len(input);

    let prev_row_idx = index.checked_sub(line_len);
    let next_row_idx = if (index + line_len) < input.len() {
        Some(index + line_len)
    } else {
        None
    };

    if let Some(idx) = prev_row_idx {
        row_nums(input, idx, &mut nums)
    }
    if let Some(idx) = next_row_idx {
        row_nums(input, idx, &mut nums)
    }
    if let Some(num) = take_backward_while_num(input, index - 1) {
        nums.push(num)
    }
    if let Some(num) = take_forward_while_num(input, index + 1) {
        nums.push(num)
    }

    nums
}

pub(crate) fn part_two() -> i32 {
    let filename = PathBuf::from_str("inputs/03.txt").unwrap();
    let input = fs::read_to_string(filename).unwrap();

    input
        .chars()
        .enumerate()
        // get indexes of *'s
        .filter_map(|(i, c)| if c == '*' { Some(i) } else { None })
        // get adjacent numbers for *'s
        .filter_map(|idx| {
            if adjacent_nums(&input, idx).len() == NUM_TO_MAKE_GEAR {
                Some(adjacent_nums(&input, idx))
            } else {
                None
            }
        })
        .map(|vec| vec.iter().product::<i32>())
        .sum()
}

pub(crate) fn time_both() {
    let t1 = std::time::SystemTime::now();
    let _ = part_one();
    let t2 = std::time::SystemTime::now();
    println!("Day 3 part 1 took {:?}", t2.duration_since(t1).unwrap());

    let t1 = std::time::SystemTime::now();
    let _ = part_two();
    let t2 = std::time::SystemTime::now();
    println!("Day 3 part 2 took {:?}", t2.duration_since(t1).unwrap());
}
