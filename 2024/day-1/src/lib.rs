use aoc_utils::parse_whitespace_delimited;

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

fn sum_scores(sorted_lhs: &[i128], sorted_rhs: &[i128]) -> i128 {
    let lhs_counts = count_occurrences(sorted_lhs);
    let rhs_counts = count_occurrences(sorted_rhs);

    // Iterate through both counts in parallel
    let mut lhs_iter = lhs_counts.into_iter();
    let mut rhs_iter = rhs_counts.into_iter();
    let mut lhs_entry = lhs_iter.next();
    let mut rhs_entry = rhs_iter.next();
    let mut total_sum = 0;

    while let (Some(lhs_item), Some(rhs_item)) = (lhs_entry.as_ref(), rhs_entry.as_ref()) {
        match (lhs_item, rhs_item) {
            (lhs, rhs) if lhs.value == rhs.value => {
                total_sum += lhs.value * (lhs.count as i128) * (rhs.count as i128);
                lhs_entry = lhs_iter.next();
                rhs_entry = rhs_iter.next();
            }
            (lhs, rhs) if lhs.value < rhs.value => {
                lhs_entry = lhs_iter.next();
            }
            (lhs, rhs) if lhs.value > rhs.value => {
                rhs_entry = rhs_iter.next();
            }
            _ => unreachable!(),
        }
    }

    total_sum
}

struct Item {
    value: i128,
    count: usize,
}

fn count_occurrences(sorted_lhs: &[i128]) -> Vec<Item> {
    let mut counts = Vec::new();
    let mut current_value = sorted_lhs.first().cloned();
    for &value in sorted_lhs {
        if Some(value) != current_value || counts.is_empty() {
            counts.push(Item { value, count: 1 });
            current_value = Some(value);
        } else if let Some(last) = counts.last_mut() {
            last.count += 1;
        }
    }
    counts
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
