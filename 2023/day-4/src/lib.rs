use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Card {
    card_no: u32,
    winning_numbers: Vec<u32>,
    our_numbers: Vec<u32>,
}

impl Card {
    /// Parses all lines into a vector of [`Card`].
    pub fn parse_all(input: &str) -> Result<Vec<Card>, ParseCardError> {
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(Card::from_str)
            .collect()
    }

    /// Sums all winning scores across all cards..
    pub fn sum_all_scores<'a, C: IntoIterator<Item = &'a Card>>(cards: C) -> u32 {
        cards
            .into_iter()
            .fold(0, |sum, card| sum + card.get_score())
    }

    /// Counts the number of copied cards.
    pub fn count_copied_cards<C: IntoIterator<Item = Card>>(cards: C) -> u32 {
        Self::determine_copies(cards)
            .iter()
            .fold(0, |sum, card| sum + card.0)
    }

    /// Determines the number of copies per card.
    pub fn determine_copies<C: IntoIterator<Item = Card>>(cards: C) -> Vec<(u32, Card)> {
        let mut cards: Vec<_> = cards.into_iter().map(|c| (1, c)).collect();
        for i in 0..cards.len() {
            let num_copies_to_make = cards[i].0;
            let num_rows_to_copy = cards[i].1.get_num_winning() as usize;
            for (count, _) in cards.iter_mut().take(i + num_rows_to_copy + 1).skip(i + 1) {
                *count += num_copies_to_make;
            }
        }

        cards
    }

    /// Returns the number of winning numbers in `our_numbers`.
    fn get_num_winning(&self) -> u32 {
        let winning: HashSet<&u32> = HashSet::from_iter(&self.winning_numbers);
        let ours = HashSet::from_iter(&self.our_numbers);
        winning.intersection(&ours).count() as u32
    }

    /// Calculate the score based on the number of winnings.
    ///
    /// # Arguments
    ///
    /// - `self` - The reference to the object.
    ///
    /// # Returns
    ///
    /// The score calculated based on the number of winnings.
    pub fn get_score(&self) -> u32 {
        let our_winning = self.get_num_winning();
        if our_winning > 0 {
            2u32.pow(our_winning - 1)
        } else {
            0
        }
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon_pos = s
            .find(':')
            .ok_or(ParseCardError("missing card separator"))?;
        if !s.starts_with("Card ") {
            return Err(ParseCardError("invalid prefix"));
        }

        let card_no: u32 = s[5..colon_pos]
            .trim()
            .parse()
            .map_err(|_| ParseCardError("invalid card number"))?;

        let s = &s[colon_pos + 1..];
        let bar_pos = s
            .find('|')
            .ok_or(ParseCardError("missing number separator"))?;

        let winning_numbers = s[..bar_pos].trim();
        let our_numbers = s[bar_pos + 1..].trim();

        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(u32::from_str)
            .collect::<Result<_, _>>()
            .map_err(|_| ParseCardError("failed to parse a winning number"))?;

        let our_numbers = our_numbers
            .split_whitespace()
            .map(u32::from_str)
            .collect::<Result<_, _>>()
            .map_err(|_| ParseCardError("failed to parse an owned number"))?;

        Ok(Self {
            card_no,
            winning_numbers,
            our_numbers,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParseCardError(&'static str);

impl Display for ParseCardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse card: {}", self.0)
    }
}

impl Error for ParseCardError {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_parse_card() {
        let card = Card::from_str("Card 31:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")
            .expect("invalid card");
        assert_eq!(card.card_no, 31);
        assert_eq!(card.winning_numbers, [1, 21, 53, 59, 44]);
        assert_eq!(card.our_numbers, [69, 82, 63, 72, 16, 21, 14, 1]);
    }

    #[rstest(
        input,
        num_winning,
        score,
        case("Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 4, 8),
        case("Card  2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2, 2),
        case("Card  3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2, 2),
        case("Card  4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1, 1),
        case("Card  5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0, 0),
        case("Card 10: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0, 0)
    )]
    fn test_winning_numbers(input: &str, num_winning: u32, score: u32) {
        let card = Card::from_str(input).expect("invalid card");
        assert_eq!(card.get_num_winning(), num_winning);
        assert_eq!(card.get_score(), score);
    }

    #[test]
    fn test_count_copies() {
        const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                             Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                             Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                             Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                             Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                             Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = Card::parse_all(INPUT).expect("invalid input");
        let counted = Card::determine_copies(cards.clone());
        assert_eq!(counted.len(), 6);
        assert_eq!(counted[0].0, 1);
        assert_eq!(counted[1].0, 1 + 1);
        assert_eq!(counted[2].0, 1 + 1 + 2);
        assert_eq!(counted[3].0, 1 + 1 + 2 + 4);
        assert_eq!(counted[4].0, 1 + 1 + 4 + 8);
        assert_eq!(counted[5].0, 1);

        let total_copies = Card::count_copied_cards(cards);
        assert_eq!(total_copies, 30);
    }
}
