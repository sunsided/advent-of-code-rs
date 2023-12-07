use aoc_2023_day_7::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("2023 Day 7: Camel Cards");
    println!(
        "The total winnings without jokes are: {}",
        total_winnings(INPUT, Jokers::Disallowed)
    );
    println!(
        "The total winnings with jokes are: {}",
        total_winnings(INPUT, Jokers::Allowed)
    );
}
