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

fn part_one_cmp(a: &str, b: &str) -> Ordering {
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

fn part_two_cmp(a: &str, b: &str) -> Ordering {
    let a_type = part_two_hand(a).expect("error parsing hand type");
    let b_type = part_two_hand(b).expect("error parsing hand type");

    if a_type == b_type {
        // compare strings
        let (a_char, b_char) = a.chars().zip(b.chars()).find(|(a, b)| a != b).unwrap();
        let a = Card::from(a_char);
        let b = Card::from(b_char);

        if b == Card::CardJ {
            Ordering::Greater
        } else if a == Card::CardJ {
            Ordering::Less
        } else {
            b.cmp(&a)
        }
    } else {
        b_type.cmp(&a_type)
    }
}

fn part_two_hand(s: &str) -> Result<HandType, Error> {
    let t = HandType::from_str(s).expect("Error parsing hand");

    let t = if !s.contains('J') {
        t
    } else {
        match t {
            FiveKind | FourKind | FullHouse => FiveKind,
            ThreeKind => FourKind,
            TwoPair => {
                if s.chars().filter(|c| *c == 'J').count() == 1 {
                    FullHouse
                } else {
                    FourKind
                }
            }
            OnePair => ThreeKind,
            HighCard => OnePair,
        }
    };
    Some(t).ok_or(Error)
}

fn find_winning_sum<F>(day: &Day07, sorter: F) -> u32
where
    F: Fn(&str, &str) -> Ordering,
{
    let input = day.get_input();

    input
        .lines()
        .map(|l| {
            let (hand, bet) = l.split_ascii_whitespace().collect_tuple().unwrap();
            (hand, bet.parse::<u32>().unwrap())
        })
        .sorted_by(|(a, _), (b, _)| sorter(a, b))
        .enumerate()
        .map(|(i, (_, bet))| (i + 1) as u32 * bet)
        .sum()
}

impl Solution for Day07 {
    const DAY_NUM: i32 = 7;
    type ReturnType = u32;

    fn part_one(&self) -> Self::ReturnType {
        find_winning_sum(self, part_one_cmp)
    }

    fn part_two(&self) -> Self::ReturnType {
        find_winning_sum(self, part_two_cmp)
    }
}
