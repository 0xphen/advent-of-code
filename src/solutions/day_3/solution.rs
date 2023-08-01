use std::{collections::HashMap, fs};

use crate::solutions::traits::Solution;

pub struct Day3<'a> {
    data: Vec<&'a str>,
}

impl<'a> Day3<'a> {
    fn new(data: Vec<&'a str>) -> Day3<'a> {
        return Day3 { data };
    }

    fn part_1(&self) -> u32 {
        let mut duplicate_map: HashMap<u32, Vec<String>> = HashMap::new();

        for (index, item) in self.data.clone().into_iter().enumerate() {
            let mid = item.len() / 2;
            let (first_half, second_half) = item.split_at(mid);

            first_half.chars().for_each(|c| {
                if second_half.contains(c) {
                    let vec_value = duplicate_map.entry(index as u32).or_insert(vec![]);

                    if !vec_value.contains(&c.to_string()) {
                        vec_value.push(c.to_string());
                    }
                }
            });
        }

        let mut total_pririoty = 0;

        for i in 0..self.data.len() {
            let option_map_value = duplicate_map.get(&(i as u32));

            match option_map_value {
                Some(v) => {
                    total_pririoty += Day3::get_total_vec_priority(v);
                }
                _ => continue,
            }
        }

        return total_pririoty;
    }

    fn part_2(&self) -> u32 {
        let mut elf_groups: Vec<Vec<&str>> = vec![];

        let group_max = 3;

        for i in (0..self.data.len()).step_by(3) {
            let elg_group = (&self.data[i..(i + group_max)]).to_vec();
            elf_groups.push(elg_group);
        }

        let mut all_badges: Vec<String> = vec![];

        for group in elf_groups.into_iter() {
            let item_str = Day3::longest_str(&group);

            for c in item_str.chars() {
                if group[0].contains(c) && group[1].contains(c) && group[2].contains(c) {
                    all_badges.push(c.to_string());
                    break;
                }
            }
        }

        let total_badges_pririoty = Day3::get_total_vec_priority(&all_badges);

        return total_badges_pririoty;
    }

    fn get_total_vec_priority(arr: &Vec<String>) -> u32 {
        let mut total_priority = 0;
        for c in arr.into_iter() {
            total_priority += Day3::priority(c);
        }

        return total_priority;
    }

    fn priority(letter: &str) -> u32 {
        let mut pririoty: u32 = 0;

        for i in (b'a'..=b'z').chain(b'A'..=b'Z') {
            let number = if i.is_ascii_lowercase() {
                i - b'a' + 1
            } else {
                i - b'A' + 27
            };

            if (i as char).to_string().as_str() == letter {
                pririoty = number as u32;
            }
        }

        return pririoty;
    }

    fn longest_str(vec_str: &'a Vec<&str>) -> &'a str {
        let mut largest = "";

        for v in vec_str {
            if v.len() >= largest.len() {
                largest = v;
            }
        }

        largest
    }
}

impl<'a> Solution for Day3<'a> {
    type PartOne = u32;
    type PartTwo = u32;

    fn solution() -> (Self::PartOne, Self::PartTwo) {
        let read_string = fs::read_to_string("src/inputs/day_3_input.txt").unwrap();

        let parsed_input = read_string.split("\n").collect::<Vec<&str>>();

        let day_3 = Day3::new(parsed_input);
        let part_1_soln = day_3.part_1();

        let part_2_soln = day_3.part_2();

        (part_1_soln, part_2_soln)
    }
}
