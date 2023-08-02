mod solutions;

use crate::solutions::{
    day_1::solution::Day1, day_2::solution::Day2, day_3::solution::Day3, day_4::solution::Day4,
    day_5::solution::Day5, day_6::solution::Day6, traits::Solution,
};

fn main() {
    // Day 1
    println!("Day 1 solution : {:?} ", Day1::solution());

    // Day 2
    println!("Day 2 solution : {:?} ", Day2::solution());

    // Day 3
    println!("Day 3 solution : {:?} ", Day3::solution());

    // Day 4
    println!("Day 4 solution : {:?} ", Day4::solution());

    // Day 4
    println!("Day 5 solution : {:?} ", Day5::solution());

    // Day 4
    println!("Day 6 solution : {:?} ", Day6::solution());
}
