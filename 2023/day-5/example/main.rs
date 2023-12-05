use aoc_2023_day_5::Almanac;
use rayon::prelude::*;
use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("2023 Day 5: If You Give A Seed A Fertilizer");

    let almanac = Almanac::from_str(INPUT).expect("invalid input");

    // Part 1
    let smallest_location = almanac
        .map_seeds()
        .min_by(|(_, lhs), (_, rhs)| lhs.cmp(&rhs))
        .expect("invalid calculation");
    println!(
        "The smallest location number of the mapped seeds is for seed {} at location {}",
        smallest_location.0, smallest_location.1
    );

    // Part 2
    let smallest_location = almanac
        .map_seed_ranges()
        .min_by_key(|(_, loc)| loc.value())
        .expect("invalid calculation");
    println!(
        "The smallest location number of the mapped seed ranges is for seed {} at location {}",
        smallest_location.0, smallest_location.1
    )
}
