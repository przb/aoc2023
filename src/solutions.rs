pub(crate) mod day01;
pub(crate) mod day02;
pub(crate) mod day03;
pub(crate) mod day04;
pub(crate) mod day05;

pub(crate) trait Solution {
    const DAY_NUM: i32;
    type ReturnType;
    fn part_one(&self) -> Self::ReturnType;
    fn part_two(&self) -> Self::ReturnType;
    fn time_both(&self) {
        let t1 = std::time::SystemTime::now();
        let _ = self.part_one();
        let t2 = std::time::SystemTime::now();
        println!(
            "Day {} part 1 took {:?}",
            Self::DAY_NUM,
            t2.duration_since(t1).unwrap()
        );

        let t1 = std::time::SystemTime::now();
        let _ = self.part_two();
        let t2 = std::time::SystemTime::now();
        println!(
            "Day {} part 2 took {:?}",
            Self::DAY_NUM,
            t2.duration_since(t1).unwrap()
        );
    }
}
