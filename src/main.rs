mod day1;
mod day2;
mod day3;

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_file(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines()
}

fn main() {
    let f_day1 = "input/day1.txt";
    println!("Day1.1: {}", day1::get_max(f_day1));
    println!("Day1.2: {}", day1::get_top_three_sum(f_day1));

    let f_day1 = "input/day2.txt";
    println!("Day2.1: {}", day2::get_score(f_day1));
    println!("Day2.2: {}", day2::get_corrected_score(f_day1));

    let f_day3 = "input/day3.txt";
    println!("Day2.1: {}", day3::get_priority_sum(f_day3));
}
