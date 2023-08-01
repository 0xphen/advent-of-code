use crate::solutions::traits::Solution;

use std::{collections::HashMap, collections::HashSet, fs::File, io::BufRead, io::BufReader};

pub struct Day4 {
    pair_map: HashMap<usize, Vec<Vec<u32>>>,
}

impl<'a> Day4 {
    fn new(pair_map: HashMap<usize, Vec<Vec<u32>>>) -> Self {
        Day4 { pair_map }
    }

    fn part_1(&self) -> u32 {
        let mut count = 0;

        for (_k, v) in &self.pair_map {
            let (largest_vec, smallest_vec) = Day4::largest_vec(&v[0], &v[1]);

            let set_0: HashSet<_> = largest_vec.iter().collect();
            let is_subset = smallest_vec.iter().all(|item| set_0.contains(item));

            if is_subset {
                count += 1;
            }
        }

        return count;
    }

    // TODO: remove duplicate code in part_1 and part_2
    fn part_2(&self) -> u32 {
        let mut count = 0;

        for (_k, v) in &self.pair_map {
            let (largest_vec, smallest_vec) = Day4::largest_vec(&v[0], &v[1]);
            let set_0: HashSet<_> = largest_vec.iter().collect();
            let is_subset = smallest_vec.iter().any(|item| set_0.contains(item));

            if is_subset {
                count += 1;
            }
        }

        return count;
    }

    fn parse_line_into_vec(line: String) -> Vec<Vec<u32>> {
        let split_line = line
            .split(",")
            .map(|group| group.split("-").map(|c| c.parse::<u32>().unwrap()));

        let parsed_sections = split_line
            .map(|item| {
                let temp_vec = item.collect::<Vec<u32>>();

                (temp_vec[0]..=temp_vec[1]).collect()
            })
            .collect::<Vec<Vec<u32>>>();

        return parsed_sections;
    }

    fn largest_vec(a: &'a Vec<u32>, b: &'a Vec<u32>) -> (&'a Vec<u32>, &'a Vec<u32>) {
        if a.len() > b.len() {
            return (a, b);
        }

        return (b, a);
    }
}

impl Solution for Day4 {
    type PartOne = u32;
    type PartTwo = u32;

    fn solution() -> (Self::PartOne, Self::PartTwo) {
        let file = File::open("src/inputs/day_4_input.txt").expect("Failed to open file");
        let lines = BufReader::new(file).lines();

        let mut pair_map: HashMap<usize, Vec<Vec<u32>>> = HashMap::new();

        for (index, line) in lines.enumerate() {
            if let Ok(l) = line {
                let parsed_sections = Day4::parse_line_into_vec(l);
                pair_map.insert(index, parsed_sections);
            }
        }

        let day_4 = Day4::new(pair_map);
        let part_1_soln = day_4.part_1();
        let part_2_soln = day_4.part_2();

        (part_1_soln, part_2_soln)
    }
}
