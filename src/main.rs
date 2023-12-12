use aoc2023::time_day_and_add_row;
use prettytable::{row, Table};
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

mod solutions;
#[cfg(test)]
mod test;

use crate::solutions::{day07, Solution};

fn run_all() {
    let mut table = Table::new();
    table.add_row(row!["Day Num", "Part 1 Duration", "Part 2 Duration"]);
    time_day_and_add_row!(01);
    time_day_and_add_row!(02);
    time_day_and_add_row!(03);
    time_day_and_add_row!(04);
    // make_table_row!(05);
    time_day_and_add_row!(06);
    time_day_and_add_row!(07);

    table.printstd();

    // save to file else print error
    let time_results_filename = PathBuf::from("times.csv");
    let file = File::create(time_results_filename.clone()).expect("error creating file");
    match table.to_csv(file) {
        Ok(_) => println!("saved results to {:?}", time_results_filename),
        Err(_) => println!("error saving csv"),
    };
}

fn run_current() {
    let sum = day07::Day07.part_one();
    println!("7.1: {sum}");
    let sum = day07::Day07.part_two();
    println!("7.2: {sum}");
    day07::Day07.print_time();
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(_v) = std::env::args().nth(1) {
        run_all();
    } else {
        run_current();
    }

    Ok(())
}
