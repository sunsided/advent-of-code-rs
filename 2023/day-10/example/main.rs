use aoc_2023_day_10::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("2023 Day 10: Pipe Maze");
    println!(
        "The furthest number of steps from the start in either direction: {}",
        part1(INPUT)
    );
    // println!("...: {}", part2(INPUT));
}
