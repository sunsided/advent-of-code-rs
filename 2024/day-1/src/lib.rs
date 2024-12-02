use aoc_utils::parse_whitespace_delimited;
use std::collections::HashMap;

pub fn first_part(input: &str) -> i128 {
    let (lhs, rhs) = split_values(input);
    sum_distances(&lhs, &rhs)
}

pub fn second_part(input: &str) -> i128 {
    let (lhs, rhs) = split_values(input);
    sum_scores(&lhs, &rhs)
}

fn split_values(input: &str) -> (Vec<i128>, Vec<i128>) {
    let mut lhs = Vec::new();
    let mut rhs = Vec::new();

    for line in input.trim().lines() {
        let nums = parse_whitespace_delimited::<i128>(line)
            .expect("expect all lines to contain exactly two numbers");
        lhs.push(nums[0]);
        rhs.push(nums[1]);
    }

    lhs.sort_unstable();
    rhs.sort_unstable();

    (lhs, rhs)
}

fn sum_distances(lhs: &[i128], rhs: &[i128]) -> i128 {
    lhs.iter()
        .zip(rhs)
        .map(|(&a, &b)| (a.max(b), a.min(b)))
        .map(|(a, b)| a - b)
        .sum()
}

fn sum_scores(lhs: &[i128], rhs: &[i128]) -> i128 {
    let lhs = count_occurrences(lhs);
    let rhs = count_occurrences(rhs);

    lhs.iter()
        .filter_map(|(num, &lhs_count)| {
            rhs.get(num)
                .map(|&rhs_count| *num * lhs_count as i128 * rhs_count as i128)
        })
        .sum()
}

fn count_occurrences(values: &[i128]) -> HashMap<i128, usize> {
    let mut occurrences = HashMap::new();
    for &value in values {
        *occurrences.entry(value).or_insert(0) += 1;
    }
    occurrences
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        3    4
        4    3
        2    5
        1    3
        3    9
        3    3
    ";

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(INPUT), 31);
    }
}
