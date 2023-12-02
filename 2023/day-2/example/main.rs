use aoc_2023_day_2::{Game, SetOfCubes};

const INPUT: &str = include_str!("../input.txt");
const GIVEN: SetOfCubes = SetOfCubes::rgb(12, 13, 14);

fn main() {
    let games: Vec<_> = Game::iter_games(INPUT.lines())
        .map(|g| g.expect("found invalid game"))
        .collect();

    let sum_of_possible_game_ids: u32 = Game::filter_playable_games(games.iter(), &GIVEN)
        .map(Game::game_number)
        .sum();
    println!("The sum of all possible game IDs is: {sum_of_possible_game_ids}");

    let power_of_smallest: u32 = games
        .iter()
        .map(|g| g.smallest_set_needed())
        .map(|g| g.power())
        .sum();
    println!("The total power of all smallest sets is: {power_of_smallest}");
}
