use std::error::Error;
mod solutions;
use crate::solutions::day01;

fn run_all() {
    day01::time_both()
}
fn run_current() {
    let sum = day01::part_two();
    println!("calibration: {sum}");
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(_v) = std::env::args().nth(1) {
        run_all();
    } else {
        run_current()
    }

    Ok(())
}
