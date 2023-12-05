use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

const DEC_RADIX: u32 = 10;

fn main() -> Result<(), Box<dyn Error>> {
    let input = PathBuf::from_str("inputs/01.txt")?;

    let sum: u32 = fs::read_to_string(input)?
        .lines()
        .map(|l| {
            let mut digits = l.chars().filter(|c| c.is_ascii_digit());
            let first = digits.next().unwrap_or_default();
            let last = digits.last().unwrap_or(first);
            first.to_digit(DEC_RADIX).unwrap() * DEC_RADIX + last.to_digit(DEC_RADIX).unwrap()
        })
        .sum();

    println!("calibration: {sum}");

    Ok(())
}
