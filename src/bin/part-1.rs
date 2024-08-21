use day_02::process_input_part1;
use std::fs;


fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_input_part1(&file));
}