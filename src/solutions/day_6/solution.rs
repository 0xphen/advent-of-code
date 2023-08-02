use crate::solutions::traits::Solution;

use std::{collections::HashSet, fs::read_to_string, io::BufRead, io::BufReader};

pub struct Day6(String);

impl Day6 {
    fn new(packets: String) -> Self {
        Day6(packets)
    }

    fn part_1(&self) -> u32 {
        let mut start: u32 = 0;

        for i in (0..self.0.len()) {
            let temp_marker = &self.0[i..(i + 4)];
            let temp_set = temp_marker.chars().collect::<HashSet<_>>();

            if temp_set.len() == temp_marker.len() {
                start = (i as u32) + 4;
                break;
            }
        }

        return start;
    }
}

impl Solution for Day6 {
    type PartOne = u32;
    type PartTwo = u32;

    fn solution() -> (Self::PartOne, Self::PartTwo) {
        let packets = read_to_string("src/inputs/day_6_input.txt").unwrap();

        let day_6 = Day6::new(packets);
        let part_1_soln = day_6.part_1();

        (part_1_soln, 2)
    }
}
