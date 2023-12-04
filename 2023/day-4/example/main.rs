use aoc_2023_day_4::Card;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Day 4: Scratchcards");
    println!(
        "Total points: {}",
        Card::sum_all_scores(INPUT).expect("invalid input")
    )
}
