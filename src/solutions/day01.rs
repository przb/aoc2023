use crate::solutions::Solution;

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

pub(crate) struct Day01;

impl Solution for Day01 {
    const DAY_NUM: i32 = 1;
    type ReturnType = u32;

    fn part_one(&self) -> u32 {
        let input = Day01.get_input();

        input
            .lines()
            .map(|l| {
                let mut digits = l.chars().filter(|c| c.is_ascii_digit());
                let first = digits.next().unwrap_or_default();
                let last = digits.last().unwrap_or(first);
                first.to_digit(DEC_RADIX).unwrap() * DEC_RADIX + last.to_digit(DEC_RADIX).unwrap()
            })
            .sum()
    }

    fn part_two(&self) -> u32 {
        let input = Day01.get_input();

        input
            .lines()
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
}
