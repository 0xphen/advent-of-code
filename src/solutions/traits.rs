pub trait Solution {
    type PartOne;
    type PartTwo;

    fn solution() -> (Self::PartOne, Self::PartTwo);
}
