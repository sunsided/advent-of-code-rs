use std::ops::Sub;
use std::str::FromStr;

/// Parses whitespace-delimited values from an input string.
///
/// This function takes an input string and splits it into words (delimited by whitespaces),
/// then it attempts to parse each word into the designated type `T`.
/// The function collects the successfully parsed numbers into a vector and returns it.
///
/// # Examples
///
/// ```
/// use aoc_utils::parse_whitespace_delimited;
///
/// let result = parse_whitespace_delimited::<u32>("1 2 3");
/// assert_eq!(result, Ok(vec![1, 2, 3]));
///
/// let result = parse_whitespace_delimited::<u32>("10 20 30");
/// assert_eq!(result, Ok(vec![10, 20, 30]));
///
/// let result = parse_whitespace_delimited::<u32>("5 4 3 2 1");
/// assert_eq!(result, Ok(vec![5, 4, 3, 2, 1]));
///
/// let result = parse_whitespace_delimited::<u32>("1 a 3");
/// assert!(result.is_err());
/// ```
///
/// # Errors
///
/// This function returns a `Result` with the vector of parsed numbers or the error returned
/// by the type `T`' [`FromStr`] implementation. If any word fails to parse, an error is returned.
///
/// ```rust
/// use aoc_utils::parse_whitespace_delimited;
///
/// let result = parse_whitespace_delimited::<u32>("1 a 3");
/// assert!(result.is_err());
///
/// let err = result.unwrap_err();
/// assert_eq!(err.to_string(), "invalid digit found in string");
/// ```
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Returns
///
/// Returns a `Result` containing the vector of parsed values or an error.
pub fn parse_whitespace_delimited<T>(input: &str) -> Result<Vec<T>, <T as FromStr>::Err>
where
    T: FromStr,
{
    input.split_whitespace().map(T::from_str).collect()
}

/// Determines the absolute difference between two numbers.
///
/// ## Example
/// ```
/// use aoc_utils::delta;
/// assert_eq!(delta(0_u8, 10_u8), 10_u8);
/// ```
pub fn delta<T>(a: T, b: T) -> T
where
    T: PartialOrd<T> + Sub<T, Output = T>,
{
    if a <= b {
        b - a
    } else {
        a - b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_sequence() {
        assert_eq!(
            parse_whitespace_delimited::<u32>(" 79 14   55 13 1")
                .expect("failed to parse sequence"),
            [79, 14, 55, 13, 1]
        );
    }
}
