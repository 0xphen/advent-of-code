use std::fs;

pub fn solution() -> u64 {
    let content = fs::read_to_string("input.txt").unwrap();

    let groups = content
        .split("\n\n")
        .map(|group| group.lines().map(|line| line.parse::<u64>().unwrap()));

    let mut calories: Vec<u64> = groups.map(|group| group.sum()).collect();

    calories.sort_by(|a, b| a.cmp(b));
    
    // println!("The most calories is {:?}", calories[calories.len() - 1]);
    return calories[calories.len() - 1];
}
