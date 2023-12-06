use aoc_2023_day_6::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("2023 Day 6: Wait for It");
    println!(
        "Product of number of winning conditions across all games: {}",
        product_of_winning_conditions_with_spaces(INPUT)
    );
    println!(
        "Product of number of winning conditions for the game: {}",
        product_of_winning_conditions_without_spaces(INPUT)
    );
}
