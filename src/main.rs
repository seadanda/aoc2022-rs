mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
    println!("Day3.1: {}", day3::get_priority_sum(f_day3));
    println!("Day3.2: {}", day3::get_group_priority_sum(f_day3));

    let f_day4 = "input/day4.txt";
    println!("Day4.1: {}", day4::get_intersecting_sum(f_day4, false));
    println!("Day4.2: {}", day4::get_intersecting_sum(f_day4, true));

    let f_day5 = "input/day5.txt";
    println!("Day5.1: {}", day5::get_part1_top_crates(f_day5));
    println!("Day5.2: {}", day5::get_part2_top_crates(f_day5));

    let f_day6 = "input/day6.txt";
    let day6_input = read_file(f_day6).next().unwrap().unwrap();
    println!("Day6.1: {}", day6::get_marker_pos(&day6_input, 4));
    println!("Day6.2: {}", day6::get_marker_pos(&day6_input, 14));
}
