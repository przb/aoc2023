use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

const DEC_RADIX: u32 = 10;
const BAD_NUM: u32 = 99;

fn parse_num_str(s: &str) -> Option<u32> {
    match s {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn parse_num_string(s: &String) -> Option<u32> {
    match s.len() {
        3 => parse_num_str(s.as_str()),
        4 => parse_num_str(s.as_str()).or_else(|| parse_num_str(&s[1..])),
        5 => parse_num_str(s.as_str())
            .or_else(|| parse_num_str(&s[1..]))
            .or_else(|| parse_num_str(&s[2..])),
        _ => None,
    }
}

pub(crate) fn part_one() -> u32 {
    let input = PathBuf::from_str("inputs/01.txt").unwrap();

    fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| {
            let mut digits = l.chars().filter(|c| c.is_ascii_digit());
            let first = digits.next().unwrap_or_default();
            let last = digits.last().unwrap_or(first);
            first.to_digit(DEC_RADIX).unwrap() * DEC_RADIX + last.to_digit(DEC_RADIX).unwrap()
        })
        .sum()
}

pub(crate) fn part_two() -> u32 {
    let input = PathBuf::from_str("inputs/01.txt").unwrap();

    let file = fs::read_to_string(input).unwrap();
    file.lines()
        .map(|line| {
            let mut state = String::default();
            line.chars()
                .map(move |c| {
                    if c.is_ascii_digit() {
                        c.to_digit(DEC_RADIX).unwrap()
                    } else {
                        state.push(c);
                        if state.len() > 5 {
                            state.remove(0);
                        };
                        parse_num_string(&state).unwrap_or(BAD_NUM)
                    }
                })
                .filter(|v| *v != BAD_NUM)
            // .collect_vec()
        })
        .map(|mut v| {
            let first = v.next().unwrap_or_default();
            let last = v.last().unwrap_or(first);
            // let last = v.into_iter().last().unwrap_or(first);
            (first, last)
        })
        .map(|(l, r)| (l * DEC_RADIX) + r)
        .sum()
}

pub(crate) fn time_both() {
    let t1 = std::time::SystemTime::now();
    let _ = part_one();
    let t2 = std::time::SystemTime::now();
    println!("Day 1 part 1 took {:?}", t2.duration_since(t1).unwrap());

    let t1 = std::time::SystemTime::now();
    let _ = part_two();
    let t2 = std::time::SystemTime::now();
    println!("Day 1 part 2 took {:?}", t2.duration_since(t1).unwrap());
}
