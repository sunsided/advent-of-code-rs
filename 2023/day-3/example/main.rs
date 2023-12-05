use aoc_2023_day_3::Schematic;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("2023 Day 3: Gear Ratios");

    let schematic = Schematic::from_str(INPUT).expect("Failed to parse schematic");
    println!("Sum of all part numbers: {}", schematic.sum_valid_parts());
    println!("Sum of all gear ratios: {}", schematic.sum_gear_ratios());
}
