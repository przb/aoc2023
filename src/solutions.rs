use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub(crate) mod day01;
pub(crate) mod day02;
pub(crate) mod day03;
pub(crate) mod day04;
pub(crate) mod day05;
pub(crate) mod day06;

pub(crate) trait Solution {
    const DAY_NUM: i32;
    type ReturnType;
    fn part_one(&self) -> Self::ReturnType;
    fn part_two(&self) -> Self::ReturnType;

    fn get_input(&self) -> String {
        let filename = PathBuf::from(format!("inputs/{:0>2}.txt", Self::DAY_NUM));
        fs::read_to_string(filename).unwrap()
    }
    fn time_both(&self) -> (Duration, Duration) {
        let t1 = std::time::SystemTime::now();
        let _ = self.part_one();
        let t2 = std::time::SystemTime::now();

        let d1 = t2.duration_since(t1).expect("Timing error");

        let t1 = std::time::SystemTime::now();
        let _ = self.part_two();
        let t2 = std::time::SystemTime::now();
        let d2 = t2.duration_since(t1).expect("Timing error");

        (d1, d2)
    }
    fn print_time(&self) {
        let (duration_one, duration_two) = self.time_both();

        println!("Day {} part 1 took {:?}", Self::DAY_NUM, duration_one);

        println!("Day {} part 2 took {:?}", Self::DAY_NUM, duration_two);
    }
}
