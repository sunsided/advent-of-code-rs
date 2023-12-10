use aoc_2023_day_9::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("2023 Day 9: Mirage Maintenance");
    println!(
        "The sum of all (next) history predictions is: {}",
        part1(INPUT)
    );
    println!(
        "The sum of all (previous) history predictions is: {}",
        part2(INPUT)
    );
}
