use aoc_2024_day_1::{first_part, second_part};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("2024 Day 1: Historian Hysteria");
    let sum = first_part(INPUT);
    println!("The sum of distances is {}", sum);

    let sum = second_part(INPUT);
    println!("The sum of similarity scores is {}", sum);
}
