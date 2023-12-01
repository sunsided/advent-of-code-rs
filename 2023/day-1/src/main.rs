const INPUT: &str = include_str!("../input.txt");

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
        .fold(0, |sum, line| sum + get_calibration_value(line))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_calibration_digits_works() {
        assert_eq!(get_calibration_digits("1abc2"), (1, 2));
        assert_eq!(get_calibration_digits("pqr3stu8vwx"), (3, 8));
        assert_eq!(get_calibration_digits("a1b2c3d4e5f"), (1, 5));
        assert_eq!(get_calibration_digits("treb7uchet"), (7, 7));
    }

    #[test]
    fn get_calibration_value_works() {
        assert_eq!(get_calibration_value("1abc2"), 12);
        assert_eq!(get_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(get_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(get_calibration_value("treb7uchet"), 77);
    }

    #[test]
    fn sum_calibration_values_works() {
        let sum = sum_calibration_values(
            "1abc2
                   pqr3stu8vwx
                   a1b2c3d4e5f
                   treb7uchet
                ",
        );
        assert_eq!(sum, 142);
    }
}
