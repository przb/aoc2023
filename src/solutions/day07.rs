use crate::solutions::day07::HandType::{
    FiveKind, FourKind, FullHouse, HighCard, OnePair, ThreeKind, TwoPair,
};
use crate::solutions::Solution;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::Error;
use std::str::FromStr;

pub(crate) struct Day07;

#[derive(PartialOrd, PartialEq, Ord, Eq)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialOrd, PartialEq, Ord, Eq)]
enum Card {
    CardA,
    CardK,
    CardQ,
    CardJ,
    CardT,
    Card9,
    Card8,
    Card7,
    Card6,
    Card5,
    Card4,
    Card3,
    Card2,
}

impl FromStr for HandType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            Err(Error)
        } else {
            let counts = s.chars().counts();
            let t = match counts.len() {
                5 => Some(HighCard),
                4 => Some(OnePair),
                3 => {
                    // Could be two pair, or three kind
                    if counts.iter().any(|(_, num)| *num == 3) {
                        Some(ThreeKind)
                    } else {
                        Some(TwoPair)
                    }
                }
                2 => {
                    // could be:four kind or fullhouse
                    if counts.iter().any(|(_, num)| *num == 4) {
                        Some(FourKind)
                    } else {
                        Some(FullHouse)
                    }
                }

                1 => Some(FiveKind),
                _ => None,
            };

            t.ok_or(Error)
        }
    }
}

impl From<char> for Card {
    fn from(s: char) -> Self {
        match s {
            'A' => Card::CardA,
            'K' => Card::CardK,
            'Q' => Card::CardQ,
            'J' => Card::CardJ,
            'T' => Card::CardT,
            '9' => Card::Card9,
            '8' => Card::Card8,
            '7' => Card::Card7,
            '6' => Card::Card6,
            '5' => Card::Card5,
            '4' => Card::Card4,
            '3' => Card::Card3,
            '2' => Card::Card2,
            _ => panic!("error parsing card"),
        }
    }
}

fn hand_sorter(a: &str, b: &str) -> Ordering {
    let a_type = HandType::from_str(a).expect("error parsing hand type");
    let b_type = HandType::from_str(b).expect("error parsing hand type");

    if a_type == b_type {
        // compare strings
        let (a_char, b_char) = a.chars().zip(b.chars()).find(|(a, b)| a != b).unwrap();
        let a = Card::from(a_char);
        let b = Card::from(b_char);

        b.cmp(&a)
    } else {
        b_type.cmp(&a_type)
    }
}

impl Solution for Day07 {
    const DAY_NUM: i32 = 7;
    type ReturnType = u32;

    fn part_one(&self) -> Self::ReturnType {
        let input = self.get_input();

        input
            .lines()
            .map(|l| {
                let (hand, bet) = l.split_ascii_whitespace().collect_tuple().unwrap();
                (hand, bet.parse::<Self::ReturnType>().unwrap())
            })
            .sorted_by(|(a, _), (b, _)| hand_sorter(a, b))
            .enumerate()
            .map(|(i, (_, bet))| (i + 1) as u32 * bet)
            .sum()
    }

    fn part_two(&self) -> Self::ReturnType {
        3
    }
}
