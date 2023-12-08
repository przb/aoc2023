use crate::solutions::Solution;
use std::cmp::max;
use std::collections::HashMap;

fn game_maps(input: &str) -> Vec<(i32, HashMap<&str, i32>)> {
    let mut game_possibilities = vec![];

    for line in input.lines() {
        let (game_title, game_str) = line.split_once(':').unwrap();
        let mut current_game_map = HashMap::new();
        let game_num: i32 = game_title.split_once(' ').unwrap().1.parse().unwrap();

        for dice_draw in game_str.split(';') {
            for dice_colors in dice_draw.split(',') {
                let (amount, color) = dice_colors.trim().split_once(' ').unwrap();
                let amount = amount.parse::<i32>().unwrap();
                let new_val = match current_game_map.get(color) {
                    Some(v) => max(*v, amount),
                    None => amount,
                };
                current_game_map.insert(color, new_val);
            }
        }
        game_possibilities.push((game_num, current_game_map));
    }

    game_possibilities
}

pub(crate) struct Day02;

impl Solution for Day02 {
    const DAY_NUM: i32 = 2;
    type ReturnType = i32;
    fn part_one(&self) -> i32 {
        let input = Day02.get_input();
        let game_possibilities = game_maps(&input);

        // The Elf would first like to know which games would have been possible if the bag contained:
        // only 12 red cubes, 13 green cubes, and 14 blue cubes?
        game_possibilities
            .iter()
            .filter_map(|(num, game)| {
                if (game.get("red").is_none() || game.get("red").is_some_and(|n| *n <= 12))
                    && (game.get("green").is_none() || game.get("green").is_some_and(|n| *n <= 13))
                    && (game.get("blue").is_none() || game.get("blue").is_some_and(|n| *n <= 14))
                {
                    Some(*num)
                } else {
                    None
                }
            })
            .sum()
    }
    fn part_two(&self) -> i32 {
        let input = Day02.get_input();
        let game_possibilities = game_maps(&input);

        game_possibilities
            .iter()
            .map(|(_, game)| game.iter().fold(1, |acc, (_, v)| acc * v))
            .sum()
    }
}
