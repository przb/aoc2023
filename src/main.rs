use std::error::Error;

mod solutions;
#[cfg(test)]
mod test;

use crate::solutions::{day01, day02, day03};

fn run_all() {
    day01::time_both();
    day02::time_both();
    day03::time_both();
}

fn run_current() {
    let sum = day03::part_two();
    println!("3.2: {sum}");
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(_v) = std::env::args().nth(1) {
        run_all();
    } else {
        run_current()
    }

    Ok(())
}
