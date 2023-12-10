use prettytable::{row, Table};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

mod solutions;
#[cfg(test)]
mod test;

use crate::solutions::{day01, day02, day03, day04, day05, day06, Solution};

fn run_all() {
    let mut table = Table::new();

    // Add a row per time
    table.add_row(row!["Day Num", "Part 1 Duration", "Part 2 Duration"]);
    // A more complicated way to add a row:

    let (d1, d2) = day01::Day01.time_both();
    table.add_row(row!["1", format!("{d1:?}"), format!("{d2:?}")]);
    println!("done with day 1");

    let (d1, d2) = day02::Day02.time_both();
    table.add_row(row!["2", format!("{d1:?}"), format!("{d2:?}")]);
    println!("done with day 2");

    let (d1, d2) = day03::Day03.time_both();
    table.add_row(row!["3", format!("{d1:?}"), format!("{d2:?}")]);
    println!("done with day 3");

    let (d1, d2) = day04::Day04.time_both();
    table.add_row(row!["4", format!("{d1:?}"), format!("{d2:?}")]);
    println!("done with day 4");

    let (d1, d2) = day05::Day05.time_both();
    table.add_row(row!["5", format!("{d1:?}"), format!("{d2:?}")]);
    println!("done with day 5");

    table.printstd();
    let time_results_filename = PathBuf::from("times.csv");
    let file = File::create(time_results_filename.clone()).expect("error creating file");

    match table.to_csv(file) {
        Ok(_) => println!("saved results to {:?}", time_results_filename),
        Err(_) => println!("error saving csv"),
    };
}

fn run_current() {
    let sum = day06::Day06.part_one();
    println!("6.1: {sum}");
    let sum = day06::Day06.part_two();
    println!("6.2: {sum}");
    day06::Day06.print_time();
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(_v) = std::env::args().nth(1) {
        run_all();
    } else {
        run_current();
    }

    Ok(())
}
