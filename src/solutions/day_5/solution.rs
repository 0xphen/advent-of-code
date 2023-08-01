use std::{fs::File, io::BufRead, io::BufReader};

use crate::solutions::traits::Solution;

//Starting stack
// [N]         [C]     [Z]
// [Q] [G]     [V]     [S]         [V]
// [L] [C]     [M]     [T]     [W] [L]
// [S] [H]     [L]     [C] [D] [H] [S]
// [C] [V] [F] [D]     [D] [B] [Q] [F]
// [Z] [T] [Z] [T] [C] [J] [G] [S] [Q]
// [P] [P] [C] [W] [W] [F] [W] [J] [C]
// [T] [L] [D] [G] [P] [P] [V] [N] [R]
//  1   2   3   4   5   6   7   8   9

struct Instruction {
    count: u32,
    from: u32,
    to: u32,
}

pub struct Day5<'a> {
    stacks: Vec<Vec<&'a str>>,
    instructions: Vec<String>,
}

impl<'a> Day5<'a> {
    fn new(stacks: Vec<Vec<&'a str>>, instructions: Vec<String>) -> Self {
        Day5 {
            stacks,
            instructions,
        }
    }

    fn part_1(&mut self) -> String {
        return self.move_crate_and_get_top_stacks(true);
    }

    fn part_2(&mut self) -> String {
        return self.move_crate_and_get_top_stacks(false);
    }

    fn move_crate_and_get_top_stacks(&mut self, rev: bool) -> String {
        for instruction in self.instructions.iter_mut() {
            let instruction = Day5::decode_instruction(instruction);

            let from_stack_index = (instruction.from - 1) as usize;
            let to_stack_index = (instruction.to - 1) as usize;

            let start_index = self.stacks[from_stack_index]
                .len()
                .saturating_sub(instruction.count as usize);

            let slice_stack = self.stacks[from_stack_index]
                .drain(start_index..)
                .collect::<Vec<&str>>();

            if rev {
                self.stacks[to_stack_index].extend(slice_stack.into_iter().rev());
            } else {
                self.stacks[to_stack_index].extend(slice_stack.into_iter());
            }
        }

        let mut top_stack = String::new();
        for stack in self.stacks.iter() {
            top_stack.push_str(stack[stack.len() - 1]);
        }

        return top_stack;
    }

    fn decode_instruction(line: &str) -> Instruction {
        // Instructions will always be in the format "move # from # to #".
        let split_instruction = line.trim_end().split_whitespace().collect::<Vec<&str>>();
        Instruction {
            count: split_instruction[1].parse::<u32>().unwrap(),
            from: split_instruction[3].parse::<u32>().unwrap(),
            to: split_instruction[5].parse::<u32>().unwrap(),
        }
    }
}

impl<'a> Solution for Day5<'a> {
    type PartOne = String;
    type PartTwo = String;

    fn solution() -> (Self::PartOne, Self::PartTwo) {
        // We hardcode the starting stacks as given in the problem statement.
        let starting_stacks: Vec<Vec<&str>> = vec![
            ["T", "P", "Z", "C", "S", "L", "Q", "N"].to_vec(),
            ["L", "P", "T", "V", "H", "C", "G"].to_vec(),
            ["D", "C", "Z", "F"].to_vec(),
            ["G", "W", "T", "D", "L", "M", "V", "C"].to_vec(),
            ["P", "W", "C"].to_vec(),
            ["P", "F", "J", "D", "C", "T", "S", "Z"].to_vec(),
            ["V", "W", "G", "B", "D"].to_vec(),
            ["N", "J", "S", "Q", "H", "W"].to_vec(),
            ["R", "C", "Q", "F", "S", "L", "V"].to_vec(),
        ];

        let file = File::open("src/inputs/day_5_input.txt").expect("Failed to open file");
        let instructions = BufReader::new(file)
            .lines()
            .collect::<Result<Vec<String>, _>>()
            .unwrap();

        let mut day_5 = Day5::new(starting_stacks, instructions);
        let part_1_soln = day_5.part_1();
        let part_2_soln = day_5.part_2();

        (part_1_soln, part_2_soln)
    }
}
