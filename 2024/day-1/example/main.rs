use aoc_2024_day_1::first_part;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("2024 Day 1: Historian Hysteria");
    let sum = first_part(INPUT);
    println!("The sum of distances is {}", sum);
}
