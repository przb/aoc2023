use crate::solutions::day12::SpringStatus::{Broken, Operational, Unknown};
use crate::solutions::Solution;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

pub(crate) struct Day12;

const BROKEN_CHAR: char = '#';
const OPERATIONAL_CHAR: char = '.';
const UNKNOWN_CHAR: char = '?';

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum SpringStatus {
    Operational,
    Broken,
    Unknown,
}

impl TryFrom<char> for SpringStatus {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            BROKEN_CHAR => Ok(Broken),
            OPERATIONAL_CHAR => Ok(Operational),
            UNKNOWN_CHAR => Ok(Unknown),
            _ => Err("Error Parsing String Status"),
        }
    }
}

fn count(cfg: &[u8], nums: &[usize]) -> u32 {
    if cfg.len() == 0 {
        if nums.len() == 0 {
            1
        } else {
            0
        }
    } else if nums.len() == 0 {
        if cfg.contains(&(BROKEN_CHAR as u8)) {
            0
        } else {
            1
        }
    } else {
        let len = cfg.len();
        let mut result = 0;
        let first_char = cfg[0];
        if ".?".as_bytes().contains(&first_char) {
            result += count(&cfg[1..len], nums);
        }

        if "#?".as_bytes().contains(&first_char) {
            let first_num = nums[0] as usize;
            let functional = &(OPERATIONAL_CHAR as u8);
            let broken = BROKEN_CHAR as u8;
            if first_num <= len
                && !cfg[0..first_num].contains(functional)
                && (first_num == len || cfg[first_num] != broken)
            {
                let start_range = usize::min(first_num + 1, len);
                let new_cfg = &cfg[start_range..len];
                result += count(&new_cfg, &nums[1..nums.len()]);
            } else {
                result += 0;
            }
        }

        result
    }
}

fn dp_count<'a>(
    cfg: &'a [u8],
    nums: &'a [usize],
    cache: &mut HashMap<(&'a [u8], &'a [usize]), u64>,
) -> u64 {
    if cfg.len() == 0 {
        if nums.len() == 0 {
            1
        } else {
            0
        }
    } else if nums.len() == 0 {
        if cfg.contains(&(BROKEN_CHAR as u8)) {
            0
        } else {
            1
        }
    } else {
        let key = (cfg, nums);

        match cache.get(&key) {
            Some(val) => *val,
            None => {
                let len = cfg.len();
                let mut result = 0;
                let first_char = cfg[0];
                if ".?".as_bytes().contains(&first_char) {
                    result += dp_count(&cfg[1..len], nums, cache);
                }

                if "#?".as_bytes().contains(&first_char) {
                    let first_num = nums[0] as usize;
                    let functional = &(OPERATIONAL_CHAR as u8);
                    let broken = BROKEN_CHAR as u8;
                    if first_num <= len
                        && !cfg[0..first_num].contains(functional)
                        && (first_num == len || cfg[first_num] != broken)
                    {
                        let start_range = usize::min(first_num + 1, len);
                        let new_cfg = &cfg[start_range..len];
                        result += dp_count(&new_cfg, &nums[1..nums.len()], cache);
                    } else {
                        result += 0;
                    }
                }
                cache.insert(key, result);
                result
            }
        }
    }
}
impl Solution for Day12 {
    const DAY_NUM: i32 = 12;
    type ReturnType = u64;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();
        input
            .par_lines()
            .map(|line| {
                let (cfg, nums) = line.split_once(" ").unwrap();
                let nums = nums.split(',').map(|c| c.parse().unwrap()).collect_vec();
                count(cfg.as_bytes(), &nums) as u64
            })
            .sum()
    }

    fn part_two(&self) -> Self::ReturnType {
        let input = self.get_input();

        input
            .par_lines()
            .map(|line| {
                let (cfg, nums) = line.split_once(" ").unwrap();
                let cfg = Vec::from([cfg, cfg, cfg, cfg, cfg]).join("?");
                let nums = Vec::from([nums, nums, nums, nums, nums]).join(",");
                let nums = nums.split(",").map(|c| c.parse().unwrap()).collect_vec();
                let mut cache = HashMap::new();
                dp_count(cfg.as_bytes(), &nums, &mut cache)
            })
            .sum()
    }
}
