use crate::solutions::traits::Solution;

use std::{collections::HashSet, fs::read_to_string, io::BufRead, io::BufReader};

pub struct Day6(String);

impl Day6 {
    fn new(packets: String) -> Self {
        Day6(packets)
    }

    fn part_1(&self) -> u32 {
        return Day6::find_marker(4, &self.0);
    }

    fn part_2(&self) -> u32 {
        return Day6::find_marker(14, &self.0);
    }

    fn find_marker(msg_len: u32, packet: &str) -> u32 {
        let mut start: u32 = 0;

        for i in (0..packet.len()) {
            let temp_marker = &packet[i..(i + msg_len as usize)];
            let temp_set = temp_marker.chars().collect::<HashSet<_>>();

            if temp_set.len() == temp_marker.len() {
                start = (i as u32) + msg_len;
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
        let part_2_soln = day_6.part_2();

        (part_1_soln, part_2_soln)
    }
}
