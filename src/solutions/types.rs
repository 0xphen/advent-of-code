#[derive(Debug)]
pub struct Solution {
    pub part_1_soln: u64,
    pub part_2_soln: u64,
}

pub trait SolutionT {
  type Output;

  fn solution() -> (Self::Output, Self::Output);
}