mod day1;

fn main() {
    let f_day1 = "input/day1.txt";
    println!("Day1.1: {}", day1::get_max(f_day1));
    println!("Day1.1: {}", day1::get_top_three_sum(f_day1));
}
