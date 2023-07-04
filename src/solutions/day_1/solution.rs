use std::fs;

use crate::solutions::types::Solution;

// #[derive(Debug)]
// pub struct Solutions {
//     pub part_1_soln: u64,
//     pub part_2_soln: u64,
// }

// https://adventofcode.com/2022/day/1
// https://adventofcode.com/2022/day/1#part2
pub fn solution() -> Solution {
    let content = fs::read_to_string("src/inputs/day_1_input.txt").unwrap();

    let groups = content
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.parse::<u64>().unwrap()));

    let mut calories: Vec<u64> = groups.map(|group| group.sum()).collect();

    calories.sort_by(|a, b| a.cmp(b));
    let part_1_soln = calories[calories.len() - 1];
    let part_2_soln = calories.into_iter().rev().take(3).sum();

    return Solution {
        part_1_soln,
        part_2_soln,
    };
}
