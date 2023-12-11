use aoc_2023_day_11::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("2023 Day 11: Cosmic Expansion");
    println!("Sum of shortest pairwise distances: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}
