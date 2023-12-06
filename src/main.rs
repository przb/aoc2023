use std::error::Error;
mod solutions;
use crate::solutions::day01;

fn run_all() {
    day01::time_both()
}
fn run_current() {
    let sum = solutions::day02::part_one();
    println!("2.1: {sum}");
    let sum = solutions::day02::part_two();
    println!("2.2: {sum}");
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(_v) = std::env::args().nth(1) {
        run_all();
    } else {
        run_current()
    }

    Ok(())
}
