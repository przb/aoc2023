use std::cmp::max;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::PathBuf;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, Hash)]
struct DiceDraw {
    color: String,
    number: i32,
}

struct DiceGame {
    min_dice: HashMap<DiceDraw, i32>,
}

pub(crate) fn part_one() -> i32 {
    let filename = PathBuf::from_str("inputs/02.txt").unwrap();

    let input = fs::read_to_string(filename).unwrap();
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

    // The Elf would first like to know which games would have been possible if the bag contained:
    // only 12 red cubes, 13 green cubes, and 14 blue cubes?
    game_possibilities.iter().filter_map(|(num, game)| {
        if (game.get("red").is_none() || game.get("red").is_some_and(|n| *n <= 12)) &&
            (game.get("green").is_none() || game.get("green").is_some_and(|n| *n <= 13)) &&
            (game.get("blue").is_none() || game.get("blue").is_some_and(|n| *n <= 14)) {
            Some(*num)
        } else {
            None
        }
    })
        .sum()
}
