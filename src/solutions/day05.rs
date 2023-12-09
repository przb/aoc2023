use crate::solutions::Solution;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::str::Split;

fn src_to_dst_map(src_num: u64, group: &Vec<u64>) -> Option<u64> {
    let size = group[2];
    let src_start = group[1];
    let dst_start = group[0];
    let src_end = src_start + size;

    if src_num >= src_start && src_num < src_end {
        let dst = (src_num - src_start) + dst_start;
        Some(dst)
    } else {
        None
    }
}

fn map_to_dst(src_num: u64, map: &Vec<Vec<u64>>) -> u64 {
    map.iter()
        .find_map(|group| src_to_dst_map(src_num, group))
        .unwrap_or(src_num)
}

fn get_mappings(input_iter: Split<char>) -> Vec<Vec<Vec<u64>>> {
    input_iter
        .map(|mapping| {
            mapping
                .trim_end_matches(|e: char| !e.is_ascii_digit())
                .trim()
                .lines()
                .map(|l| {
                    l.split_ascii_whitespace()
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect()
}

fn get_smallest_location<I>(seeds: I, mappings: &Vec<Vec<Vec<u64>>>) -> u64
where
    I: ParallelIterator<Item = u64>,
{
    seeds
        .map(|seed| {
            let mut seed = seed;
            for map in mappings {
                seed = map_to_dst(seed, map);
            }
            seed
        })
        .min()
        .unwrap()
}

pub(crate) struct Day05;

impl Solution for Day05 {
    const DAY_NUM: i32 = 5;
    type ReturnType = u64;
    fn part_one(&self) -> u64 {
        let input = Day05.get_input();
        let mut input_iter = input.split(':');

        let seeds = input_iter
            .nth(1)
            .unwrap()
            .lines()
            .next()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap());

        let mappings = get_mappings(input_iter);

        get_smallest_location(seeds.par_bridge(), &mappings)
    }

    fn part_two(&self) -> u64 {
        //! This works, but it is very inefficient
        //! It takes around 300 seconds on my laptop
        let input = Day05.get_input();
        let mut input_iter = input.split(':');

        let seeds = input_iter
            .nth(1)
            .unwrap()
            .lines()
            .next()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .tuples()
            .flat_map(|(start_num, len)| start_num..(start_num + len));

        let mappings = get_mappings(input_iter);

        get_smallest_location(seeds.par_bridge(), &mappings)
    }
}
