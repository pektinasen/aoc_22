

use std::env;
use std::fs;

pub trait Advent<T> {
    fn transform(line: &str) -> T;
    fn part_one(data: T);
    fn part_two(data: T);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    let file_path = "res/01.txt";
    let content = fs::read_to_string(file_path).expect("should have been able to read file");
    let mut lines: Vec<&str> = content.split("\n").collect();

    part_one(&lines);
    part_two(&lines);
}
