use aoc_2023_day_8::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("2023 Day 8: Haunted Wasteland");
    println!(
        "The total number of steps required from AAA to ZZZ is: {}",
        count_steps_to_destination(INPUT)
    );
}
