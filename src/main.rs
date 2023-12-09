use std::error::Error;

mod solutions;
#[cfg(test)]
mod test;

use crate::solutions::{day01, day02, day03, day04, day05, Solution};

fn run_all() {
    day01::Day01.time_both();
    day02::Day02.time_both();
    day03::Day03.time_both();
    day04::Day04.time_both();
    day05::Day05.time_both();
}

fn run_current() {
    let sum = day05::Day05.part_one();
    println!("5.1: {sum}");
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(_v) = std::env::args().nth(1) {
        run_all();
    } else {
        run_current()
    }

    Ok(())
}
