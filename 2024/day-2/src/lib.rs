use aoc_utils::parse_whitespace_delimited;

pub fn first_part(input: &str) -> usize {
    count_safe(input, false)
}

pub fn second_part(input: &str) -> usize {
    count_safe(input, true)
}

fn count_safe(input: &str, allow_single_outtake: bool) -> usize {
    let mut safe = 0;
    let mut already_found_problem: bool = false;

    // TODO: 1 2 7 8 9 - part 2: removing 2 makes it 1 -> 7, which is unsafe; removing 7 makes it 2 -> 8, which is unsafe

    'line: for line in input.trim().lines() {
        let numbers = parse_whitespace_delimited::<usize>(line).expect("failed to parse line");
        let mut iter = numbers
            .iter()
            .zip(numbers.iter().skip(1))
            .map(|(a, b)| *a as i128 - *b as i128);

        // Check start condition
        match iter.next().expect("expected at least two numbers") {
            0 => {
                if !allow_single_outtake || already_found_problem {
                    continue 'line;
                } else {
                    already_found_problem = true;
                }
            }
            x if (-3..0).contains(&x) => {
                // Ensure all descending.
                for x in iter {
                    if !(-3..0).contains(&x) {
                        if !allow_single_outtake || already_found_problem {
                            continue 'line;
                        } else {
                            already_found_problem = true;
                        }
                    }
                }
            }
            x if (1..=3).contains(&x) => {
                // Ensure all ascending.
                for x in iter {
                    if x <= 0 || x > 3 {
                        if !allow_single_outtake || already_found_problem {
                            continue 'line;
                        } else {
                            already_found_problem = true;
                        }
                    }
                }
            }
            _ => {
                if !allow_single_outtake || already_found_problem {
                    continue 'line;
                } else {
                    already_found_problem = true;
                }
            }
        }

        safe += 1;
    }

    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    ";

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(INPUT), 2);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(INPUT), 4);
    }
}
