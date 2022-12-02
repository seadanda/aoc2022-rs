mod day1;
mod day2;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_file(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines()
}

fn main() {
    let f_day1 = "input/day1.txt";
    println!("Day1.1: {}", day1::get_max(f_day1));
    println!("Day1.1: {}", day1::get_top_three_sum(f_day1));
}
