use aoc_2023_day_4::Card;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Day 4: Scratchcards");

    let sum_scores = INPUT
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| Card::from_str(line).expect("invalid input"))
        .fold(0, |sum, card| sum + card.get_score());

    println!("Total points: {sum_scores}")
}
