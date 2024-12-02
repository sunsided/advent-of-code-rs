use aoc_utils::parse_whitespace_delimited;

pub fn first_part(input: &str) -> usize {
    let mut safe = 0;
    'line: for line in input.trim().lines() {
        let numbers = parse_whitespace_delimited::<usize>(line).expect("failed to parse line");
        let mut iter = numbers
            .iter()
            .zip(numbers.iter().skip(1))
            .map(|(a, b)| *a as i128 - *b as i128);

        // Check start condition
        match iter.next().expect("expected at least two numbers") {
            0 => {
                continue 'line;
            }
            x if (-3..0).contains(&x) => {
                // Ensure all descending.
                for x in iter {
                    if !(-3..0).contains(&x) {
                        continue 'line;
                    }
                }
            }
            x if (1..=3).contains(&x) => {
                // Ensure all ascending.
                for x in iter {
                    if x <= 0 || x > 3 {
                        continue 'line;
                    }
                }
            }
            _ => {
                continue 'line;
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
}
