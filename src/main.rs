use std::error::Error;

mod day01;

fn run_all() {
    day01::time_both()
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(_v) = std::env::args().nth(1) {
        run_all();
    } else {
        let sum = day01::part_two();
        println!("calibration: {sum}");
    }

    Ok(())
}
