use lazy_static::lazy_static;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

lazy_static! {
    static ref DIGIT_REPLACEMENT: HashMap<&'static str, u32> = {
        let mut map = HashMap::new();
        map.insert("1", 1);
        map.insert("2", 2);
        map.insert("3", 3);
        map.insert("4", 4);
        map.insert("5", 5);
        map.insert("6", 6);
        map.insert("7", 7);
        map.insert("8", 8);
        map.insert("9", 9);
        map.insert("0", 0);
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("three", 3);
        map.insert("four", 4);
        map.insert("five", 5);
        map.insert("six", 6);
        map.insert("seven", 7);
        map.insert("eight", 8);
        map.insert("nine", 9);
        map.insert("zero", 0);
        map
    };
}

fn main() {
    println!("Day 1: Trebuchet?!");
    let sum = sum_calibration_values(INPUT);
    println!("The sum of all calibration values is {}", sum);
}

/// Sums the calibration values present in the given input string.
///
/// # Arguments
///
/// * `input` - The input string containing individual calibration values.
///
/// # Returns
///
/// The sum of all calibration values present in the input string.
fn sum_calibration_values(input: &str) -> u32 {
    sum_calibration_values_lines(input.lines())
}

/// Sums up the calibration values from the input lines.
///
/// This function takes an iterator of string references as input and returns the sum
/// of all calibration values found in the non-empty lines. A calibration value is
/// extracted from a line using the [`get_calibration_value`] function.
///
/// # Arguments
///
/// * `input` - An iterator of string references representing the input lines.
///
/// # Returns
///
/// The sum of all calibration values found in the input lines.
///
/// # Examples
///
/// ```
/// let lines = vec![
///     "12",
///     "",
///     "  ",
///     "34",
///     "56",
///     "  78 ",
/// ];
///
/// let sum = sum_calibration_values_lines(lines.iter());
/// assert_eq!(sum, 12 + 34 + 56 + 78);
/// ```
fn sum_calibration_values_lines<'a, I: Iterator<Item = &'a str>>(input: I) -> u32 {
    input
        .filter(|line| !line.is_empty() && !line.chars().all(char::is_whitespace))
        .fold(0, |sum, line| sum + get_calibration_value(&line))
}

/// Extracts the calibration value from a given line.
///
/// # Arguments
///
/// * `line` - A string slice representing the line of text.
///
/// # Returns
///
/// The calibration value as an unsigned 32-bit integer.
fn get_calibration_value(line: &str) -> u32 {
    let (first, second) = get_calibration_digits(line);
    first * 10 + second
}

/// Extracts the calibration digits from a given line.
///
/// # Arguments
///
/// * `line` - The line containing the calibration digits.
///
/// # Returns
///
/// A tuple containing the first and second calibration digits found in the line.
///
/// # Example
///
/// ```
/// let line = "Calibration digits: one23 34";
/// let (first, second) = get_calibration_digits(line);
/// assert_eq!(first, 1);
/// assert_eq!(second, 4);
/// ```
fn get_calibration_digits(line: &str) -> (u32, u32) {
    let first = get_first_calibration_digit(line);
    let last = get_second_calibration_digit(line);
    (first, last)
}

/// Returns the first calibration digit found in the given line.
///
/// This function searches for a specific pattern in the line and returns the corresponding
/// calibration digit. The line parameter is a string slice that represents the line to search in.
/// The function returns an unsigned 32-bit integer that represents the calibration digit found. If
/// no digit is found, the function panics with an error message.
///
/// # Arguments
///
/// * `line` - A string slice representing the line to search in.
///
/// # Panics
///
/// This function panics if the line contains no digits.
///
/// # Examples
///
/// ```rust
/// let line = "one 2 3 four";
/// let result = get_first_calibration_digit(line);
/// assert_eq!(result, 1);
/// ```
fn get_first_calibration_digit(line: &str) -> u32 {
    let mut start = 0;
    while start < line.len() {
        for (&needle, &replacement) in DIGIT_REPLACEMENT.iter() {
            if line[start..].starts_with(needle) {
                return replacement;
            }
        }
        start += 1;
    }

    panic!("line contained no digits");
}

/// Returns the second calibration digit from a given line.
///
/// # Arguments
///
/// * `line` - A string slice containing the line to search for the second calibration digit.
///
/// # Panics
///
/// This function will panic if the given line does not contain any digits.
///
/// # Examples
///
/// ```
/// let line = "one 2 3 four";
/// let digit = get_second_calibration_digit(line);
/// assert_eq!(result, 4);
/// ```
fn get_second_calibration_digit(line: &str) -> u32 {
    let mut end = line.len();
    while end > 0 {
        for (&needle, &replacement) in DIGIT_REPLACEMENT.iter() {
            if line[..end].ends_with(needle) {
                return replacement;
            }
        }
        end -= 1;
    }

    panic!("line contained no digits");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        expected_first,
        expected_second,
        case("1abc2", 1, 2),
        case("pqr3stu8vwx", 3, 8),
        case("a1b2c3d4e5f", 1, 5),
        case("treb7uchet", 7, 7)
    )]
    fn test_get_calibration_digits(input: &str, expected_first: u32, expected_second: u32) {
        assert_eq!(
            get_calibration_digits(input),
            (expected_first, expected_second)
        );
    }

    #[rstest(
        input,
        expected_sum,
        case("1abc2", 12),
        case("pqr3stu8vwx", 38),
        case("a1b2c3d4e5f", 15),
        case("treb7uchet", 77),
        case("37kc", 37),
        case("1", 11)
    )]
    fn test_get_calibration_value(input: &str, expected_sum: u32) {
        assert_eq!(get_calibration_value(input), expected_sum);
    }

    #[rstest(
        input,
        expected_sum,
        case("two1nine", 29),
        case("eightwothree", 83),
        case("abcone2threexyz", 13),
        case("xtwone3four", 24),
        case("4nineeightseven2", 42),
        case("zoneight234", 14),
        case("7pqrstsixteen", 76),
        case("37kc", 37),
        case("nxjseven7", 77),
        case("nxjseven6", 76),
        case("threeightwo", 32),
        case("threeeightwo", 32),
        case("seven31", 71),
        case("m4", 44),
        case("1", 11),
        case("2sevenclone1", 21),
        case("2three5three", 23)
    )]
    #[test]
    fn test_get_calibration_value_enhanced(input: &str, expected_sum: u32) {
        assert_eq!(get_calibration_value(input), expected_sum);
    }

    #[test]
    fn test_sum_calibration_values() {
        let sum = sum_calibration_values(
            "1abc2
                   pqr3stu8vwx
                   a1b2c3d4e5f
                   treb7uchet
                ",
        );
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_sum_calibration_values_enhanced() {
        let sum = sum_calibration_values(
            "two1nine
                   eightwothree
                   abcone2threexyz
                   xtwone3four
                   4nineeightseven2
                   zoneight234
                   7pqrstsixteen
                ",
        );
        assert_eq!(sum, 281);
    }

    #[test]
    fn test_sum_calibration_values_on_input() {
        assert_eq!(sum_calibration_values(INPUT), 53515);
    }
}
