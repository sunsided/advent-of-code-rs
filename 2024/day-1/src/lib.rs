use aoc_utils::parse_whitespace_delimited;

pub fn first_part(input: &str) -> i128 {
    let (lhs, rhs) = split_values(input);
    sum_distances(&lhs, &rhs)
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
