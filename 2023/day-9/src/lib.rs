use aoc_utils::parse_whitespace_delimited;
use itertools::Itertools;

/// Solution for part 1.
pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(parse_whitespace_delimited::<i64>)
        .map(|result| result.expect("invalid input"))
        .map(predict_part1)
        .sum()
}

/// Solution for part 2.
pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(parse_whitespace_delimited::<i64>)
        .map(|result| result.expect("invalid input"))
        .map(predict_part2)
        .sum()
}

/// Obtains the new history value prediction for part 1.
fn predict_part1(mut history: Vec<i64>) -> i64 {
    let mut last_values = vec![*history.last().expect("history has zero length")];

    while !all_zero(&history) {
        history = differentiate(&history);
        last_values.push(*history.last().expect("history has zero length"));
    }

    last_values.into_iter().sum()
}

/// Obtains the new history value prediction for part 2.
fn predict_part2(mut history: Vec<i64>) -> i64 {
    let mut last_values = vec![*history.first().expect("history has zero length")];

    while !all_zero(&history) {
        history = differentiate(&history);
        last_values.push(*history.first().expect("history has zero length"));
    }

    last_values
        .into_iter()
        .rev()
        .fold(0, |sum, current| current - sum)
}

/// Determines whether all input values are zero.
fn all_zero<H: AsRef<[i64]>>(values: H) -> bool {
    values.as_ref().iter().all(|&value| value == 0)
}

/// Obtains the difference of values and returns a vector of differences.
fn differentiate<H: AsRef<[i64]>>(values: H) -> Vec<i64> {
    values
        .as_ref()
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_prediction_part1() {
        assert_eq!(predict_part1(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(predict_part1(vec![1, 3, 6, 10, 15, 21]), 28);
    }

    #[test]
    fn test_part1() {
        const TEST: &str = "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45";

        assert_eq!(part1(TEST), 114);
    }

    #[test]
    fn test_prediction_part2() {
        assert_eq!(predict_part2(vec![10, 13, 16, 21, 30, 45]), 5);
    }

    #[test]
    fn test_part2() {
        const TEST: &str = "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45";

        assert_eq!(part2(TEST), 2);
    }
}
