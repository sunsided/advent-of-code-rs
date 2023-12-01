use lazy_static::lazy_static;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

lazy_static! {
    static ref DIGIT_REPLACEMENT: HashMap<&'static str, char> = {
        let mut map = HashMap::new();
        map.insert("one", '1');
        map.insert("two", '2');
        map.insert("three", '3');
        map.insert("four", '4');
        map.insert("five", '5');
        map.insert("six", '6');
        map.insert("seven", '7');
        map.insert("eight", '8');
        map.insert("nine", '9');
        map.insert("zero", '0');
        map
    };
}

fn main() {
    println!("Day 1: Trebuchet?!");
    let sum = sum_calibration_values(INPUT);
    println!("The sum of all calibration values is {}", sum);
}

fn sum_calibration_values(input: &str) -> u32 {
    sum_calibration_values_lines(input.lines())
}

fn sum_calibration_values_lines<'a, I: Iterator<Item = &'a str>>(input: I) -> u32 {
    input
        .filter(|line| !line.is_empty() && !line.chars().all(char::is_whitespace))
        .fold(0, |sum, line| sum + get_calibration_value_enhanced(&line))
}

fn get_calibration_value_enhanced(line: &str) -> u32 {
    let line = preprocess_calibration_digits(line);
    get_calibration_value(&line)
}

fn get_calibration_value(line: &str) -> u32 {
    let (first, second) = get_calibration_digits(line);
    first * 10 + second
}

fn get_calibration_digits(line: &str) -> (u32, u32) {
    let first = line
        .chars()
        .find_map(|c| c.to_digit(10))
        .expect("line contained no digits");
    let last = line
        .chars()
        .rev()
        .find_map(|c| c.to_digit(10))
        .expect("line contained no digits");
    (first, last)
}

fn preprocess_calibration_digits(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut start = 0;
    'char: while start < line.len() {
        let slice = &line[start..];
        let first_char = slice.chars().next().expect("the slice was empty");
        if first_char.is_numeric() {
            out.push(first_char);
            start += 1;
            continue;
        }

        for (needle, replacement) in DIGIT_REPLACEMENT.iter() {
            if slice.starts_with(needle) {
                out.push(*replacement);
                start += needle.len();
                continue 'char;
            }
        }

        out.push(first_char);
        start += 1;
    }

    out
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
        expected_output,
        case("two1nine", "219"),
        case("eightwothree", "8wo3"),
        case("abcone2threexyz", "abc123xyz"),
        case("xtwone3four", "x2ne34"),
        case("4nineeightseven2", "49872"),
        case("zoneight234", "z1ight234"),
        case("7pqrstsixteen", "7pqrst6teen"),
        case(
            "cgpqqcbfksnvppdqqsgh7twotzqglbvptmfive",
            "cgpqqcbfksnvppdqqsgh72tzqglbvptm5"
        ),
        case("7fourfourfivevbnlgzgxnpt", "7445vbnlgzgxnpt"),
        case("jeightwo5", "j8wo5"),
        case("lmgzcd4sixslonetwo", "lmgzcd46sl12"),
        case("8682", "8682"),
        case("2bhzhzpglp", "2bhzhzpglp"),
        case("onetwothreefourfivesixseveneightnine", "123456789"),
        case("onetwothreefourfivesixseveneightnine", "123456789"),
        case("one1two2three3four4five5six6seven7eight8nine9", "112233445566778899"),
        case("1one2two3three4four5five6six7seven8eight9nine", "112233445566778899"),
        case("123456789", "123456789"),
        case("2three5three", "2353"),
        case("2sevenclone1", "27cl11")
    )]
    fn test_preprocess_calibration_digits(input: &str, expected_output: &str) {
        assert_eq!(preprocess_calibration_digits(input), expected_output);
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
        case("threeeightwo", 38),
        case("seven31", 71),
        case("m4", 44),
        case("1", 11),
        case("2sevenclone1", 21),
        case("2three5three", 23)
    )]
    #[test]
    fn test_get_calibration_value_enhanced(input: &str, expected_sum: u32) {
        assert_eq!(get_calibration_value_enhanced(input), expected_sum);
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
}
