use aoc_2023_day_4::Card;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Day 4: Scratchcards");

    let cards = Card::parse_all(INPUT).expect("invalid input");
    println!("Total points: {}", Card::sum_all_scores(&cards));
    println!(
        "Total count of copied cards: {}",
        Card::count_copied_cards(cards)
    );
}
