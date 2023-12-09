use crate::solutions::Solution;
use itertools::Itertools;

fn src_to_dst_map(src_num: u64, group: &Vec<u64>) -> Option<u64> {
    let size = group[2];
    let src_start = group[1];
    let dst_start = group[0];
    let src_end = src_start + size;

    if src_num > src_start && src_num < src_end {
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
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec();

        let mappings = input_iter
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
            .collect_vec();

        seeds
            .iter()
            .map(|seed| {
                let mut seed = *seed;
                for map in mappings.clone() {
                    seed = map_to_dst(seed, &map);
                }
                seed
            })
            .min()
            .unwrap()

        // debugging prints
        // println!("seeds: {seeds:?}");
        // for (i, mapping_group) in mappings.iter().enumerate() {
        //     println!("===================== group {i} =====================");
        //     for mapping in mapping_group {
        //         println!("{mapping:?}");
        //     }
        // }

        // 123
    }

    fn part_two(&self) -> u64 {
        123
    }
}
