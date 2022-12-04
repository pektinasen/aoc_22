

use std::env;
use std::fs;

mod advent;

use crate::advent::Advent;

fn main() {

    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    let file_path = "res/02.txt";
    let content = fs::read_to_string(file_path).expect("should have been able to read file");
    let lines = content.split("\n").map(
        advent::advent02::Advent02::transform
    ).collect();

    //Can I do some dynamic loading depending on what I pass
    // as command line args
    advent::advent02::Advent02::part_one(&lines);
    advent::advent02::Advent02::part_two(&lines);
}
