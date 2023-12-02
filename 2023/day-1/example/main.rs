use aoc_2023_day_1::sum_calibration_values;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Day 1: Trebuchet?!");
    let sum = sum_calibration_values(INPUT);
    println!("The sum of all calibration values is {}", sum);
}
