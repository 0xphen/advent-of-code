mod solutions;

use crate::solutions::*;

fn main() {
    // Day 1
    let day_1_solution = day_1::solution::solution();
    println!("Day 1 solution : {:?} ", day_1_solution);

    // Day 2
    let day_2_solution = day_2::solution::Day2::solution();
    println!("Day 2 solution : {:?} ", day_2_solution);

    // Day 3
    let day_3_solution = day_3::solution::Day3::solution();
    println!("Day 3 solution : {:?} ", day_3_solution);

    // Day 4
    let day_4_solution = day_4::solution::Day4::solution();
    println!("Day 4 solution : {:?} ", day_4_solution);
}
