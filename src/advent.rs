pub mod advent02;

pub trait Advent<T> {
    fn transform(line: &str) -> T;
    fn part_one(data: &Vec<T>);
    fn part_two(data: &Vec<T>);
}