use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Solution for part 1 and 2.
pub fn total_winnings(input: &str, allow_jokers: bool) -> u64 {
    let mut games: Vec<_> = input
        .lines()
        .map(|line| Game::from_str(line).expect("invalid input"))
        .collect();
    games.sort_by(|lhs, rhs| lhs.hand().cmp(rhs.hand(), allow_jokers));

    games
        .into_iter()
        .enumerate()
        .map(|(i, game)| (i as u64 + 1) * game.bid().0)
        .sum()
}

/// A game consisting of a [`Hand`] and a [`Bid`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game(Hand, Bid);

/// A bid.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bid(u64);

/// A hand of cards.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Hand([Card; 5]);

/// A card.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Card {
    /// Card value `2`.
    Two,
    /// Card value `3`.
    Three,
    /// Card value `4`.
    Four,
    /// Card value `5`.
    Five,
    /// Card value `6`.
    Six,
    /// Card value `7`.
    Seven,
    /// Card value `8`.
    Eight,
    /// Card value `9`.
    Nine,
    /// Card value `T`.
    T,
    /// Card value `J`.
    J,
    /// Card value `Q`.
    Q,
    /// Card value `K`.
    K,
    /// Card value `A`.
    A,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HandType {
    /// All cards' labels are distinct, e.g. `23456`.
    HighCard,
    /// two cards share one label, and the other three cards have a different label from the pair and each other, e.g. `A23A4`.
    OnePair,
    /// Two cards share one label, two other cards share a second label, and the remaining card has a third label, e.g. `23432`.
    TwoPair,
    /// Three cards have the same label, and the remaining two cards are each different from any other card in the hand, e.g. `TTT98`.
    ThreeOfAKind,
    /// Three cards have the same label and the remaining cards share a different label, e.g. `23332`.
    FullHouse,
    /// Four cards have the same label, e.g. `AA8AA`.
    FourOfAKind,
    /// All five cards have the same label, e.g. `AAAAA`.
    FiveOfAKind,
}

impl Game {
    pub fn hand(&self) -> &Hand {
        &self.0
    }

    pub fn bid(&self) -> Bid {
        self.1
    }
}

impl Hand {
    /// Compares this hand to another hand.
    pub fn cmp(&self, other: &Self, allow_jokers: bool) -> Ordering {
        // First rule: The higher hand type wins.
        let hand = self
            .hand_type(allow_jokers)
            .cmp(&other.hand_type(allow_jokers));
        if hand != Ordering::Equal {
            return hand;
        }

        // Second rule: For identical hands, the first larger card determines the outcome.
        self.0
            .iter()
            .zip(other.0)
            .map(|(lhs, rhs)| lhs.cmp(&rhs))
            .find(|&ordering| ordering != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }

    /// Determines the hand type with or without allowing jokers.
    pub fn hand_type(&self, allow_jokers: bool) -> HandType {
        if allow_jokers {
            self.hand_type_with_jokers()
        } else {
            self.hand_type_without_jokers()
        }
    }

    /// Determines the hand type when jokers are disallowed.
    fn hand_type_without_jokers(&self) -> HandType {
        let counted = self.count_cards();

        Self::hand_from_card_count(counted)
    }

    /// Determines the hand type when jokers are allowed.
    fn hand_type_with_jokers(&self) -> HandType {
        let counted = self.count_cards();

        Self::hand_from_card_count(counted)
    }

    fn count_cards(&self) -> Vec<(Card, usize)> {
        let mut counts = [0_usize; Card::NUM_CARDS];
        debug_assert_eq!(Card::A.index(), 12);
        for card in &self.0 {
            counts[card.index()] += 1;
        }

        // There are at most five different cards per hand.
        let mut counted = Vec::with_capacity(5);

        for (card, count) in counts
            .into_iter()
            .rev()
            .enumerate()
            .filter(|(_, count)| *count > 0)
            .map(|(index, count)| (Card::from_index(index), count))
        {
            counted.push((card, count));
        }
        counted
    }

    /// Determines the hand from the card count.
    fn hand_from_card_count(counted: Vec<(Card, usize)>) -> HandType {
        let highest_count = counted.iter().map(|(_, count)| *count).max().unwrap_or(0);
        match counted.len() {
            // All cards are the same.
            1 => HandType::FiveOfAKind,
            // Two distinct group of cards, e.g. `AA8AA` (four of a kind) or `23332` (full house)
            2 => {
                if highest_count == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            // Three distinct groups, e.g. `TTT98` (three of a kind) or `23432` (two pair)
            3 => {
                if highest_count == 3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            // One pair and three distinct cards, e.g. `A23A4`.
            4 => HandType::OnePair,
            // All cards are different, e.g. `23456`.
            5 => HandType::HighCard,
            // No other combination is allowed.
            _ => unreachable!(),
        }
    }
}

impl Card {
    const NUM_CARDS: usize = 13;

    const CARDS: [Card; Self::NUM_CARDS] = [
        Card::Two,
        Card::Three,
        Card::Four,
        Card::Five,
        Card::Six,
        Card::Seven,
        Card::Eight,
        Card::Nine,
        Card::T,
        Card::J,
        Card::Q,
        Card::K,
        Card::A,
    ];

    /// Returns an index corresponding to each card value.
    fn index(&self) -> usize {
        match self {
            Card::Two => 0,
            Card::Three => 1,
            Card::Four => 2,
            Card::Five => 3,
            Card::Six => 4,
            Card::Seven => 5,
            Card::Eight => 6,
            Card::Nine => 7,
            Card::T => 8,
            Card::J => 9,
            Card::Q => 10,
            Card::K => 11,
            Card::A => 12,
        }
    }

    /// Returns a card corresponding to its index.
    fn from_index(index: usize) -> Card {
        Self::CARDS[index]
    }
}

impl From<u64> for Bid {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Bid> for u64 {
    fn from(value: Bid) -> Self {
        value.0
    }
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut lines = s.split_whitespace();
        let hand = Hand::from_str(
            lines
                .next()
                .ok_or(ParseGameError("Invalid game input when reading hand"))?,
        )
        .map_err(|_| ParseGameError("Invalid hand"))?;
        let bid = u64::from_str(
            lines
                .next()
                .ok_or(ParseGameError("Invalid game input when reading bid"))?,
        )
        .map_err(|_| ParseGameError("Invalid bid"))?
        .into();
        Ok(Self(hand, bid))
    }
}

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() != 5 {
            return Err(ParseHandError::InvalidLength(s.len()));
        }

        let mut cards = [Card::Two; 5];
        for (i, ch) in s.chars().enumerate() {
            cards[i] = ch.try_into().map_err(ParseHandError::InvalidCard)?;
        }

        Ok(Self(cards))
    }
}

impl TryFrom<char> for Card {
    type Error = ParseCardError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::T),
            'J' => Ok(Self::J),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            _ => Err(ParseCardError("Invalid character")),
        }
    }
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseCardError("Invalid input length"));
        }

        s.chars().next().expect("condition failed").try_into()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParseGameError(&'static str);

impl Display for ParseGameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse game: {}", self.0)
    }
}

impl Error for ParseGameError {}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseHandError {
    InvalidLength(usize),
    InvalidCard(ParseCardError),
}

impl Display for ParseHandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseHandError::InvalidLength(len) => {
                write!(f, "Invalid length of input: Expected 5, got {}", len)
            }
            ParseHandError::InvalidCard(e) => write!(f, "Invalid card in hand: {}", e),
        }
    }
}

impl Error for ParseHandError {}

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

    #[test]
    fn test_card_ordering() {
        assert!(Card::A > Card::Two);
        assert_eq!(Card::A, Card::A);
        assert_ne!(Card::A, Card::K);
    }

    #[test]
    fn test_hand_strength_ordering() {
        assert!(HandType::FiveOfAKind > HandType::HighCard);
        assert_eq!(HandType::FiveOfAKind, HandType::FiveOfAKind);
        assert_ne!(HandType::FiveOfAKind, HandType::FourOfAKind);
    }

    #[test]
    fn test_parse_card() {
        assert_eq!(Card::try_from('J'), Ok(Card::J));
        assert_eq!(Card::from_str("7"), Ok(Card::Seven));
        assert_eq!(
            Card::try_from('Y'),
            Err(ParseCardError("Invalid character"))
        );
        assert_eq!(
            Card::from_str("X"),
            Err(ParseCardError("Invalid character"))
        );
        assert_eq!(
            Card::from_str("A2"),
            Err(ParseCardError("Invalid input length"))
        );
    }

    #[test]
    fn test_parse_hand() {
        // Hand parses.
        assert_eq!(
            Hand::from_str("32T3K"),
            Ok(Hand([
                Card::Three,
                Card::Two,
                Card::T,
                Card::Three,
                Card::K
            ]))
        );

        // Spaces are ignored.
        assert_eq!(
            Hand::from_str(" 32T3K "),
            Ok(Hand([
                Card::Three,
                Card::Two,
                Card::T,
                Card::Three,
                Card::K
            ]))
        );

        // Too long input.
        assert_eq!(
            Hand::from_str("32T345"),
            Err(ParseHandError::InvalidLength(6))
        );

        // Invalid card in input.
        assert_eq!(
            Hand::from_str("32T3X"),
            Err(ParseHandError::InvalidCard(ParseCardError(
                "Invalid character"
            )))
        );
    }

    #[test]
    fn test_parse_game() {
        let game = Game::from_str("KK677 28 ").expect("parsing failed");
        assert_eq!(
            game.hand(),
            &Hand([Card::K, Card::K, Card::Six, Card::Seven, Card::Seven])
        );
        assert_eq!(game.bid(), Bid(28));
    }

    #[test]
    fn test_hand_type_five_of_a_kind() {
        assert_eq!(
            Hand::from_str("AAAAA")
                .expect("failed to parse hand")
                .hand_type(false),
            HandType::FiveOfAKind
        );
    }

    #[test]
    fn test_hand_type_four_of_a_kind() {
        assert_eq!(
            Hand::from_str("AA8AA")
                .expect("failed to parse hand")
                .hand_type(false),
            HandType::FourOfAKind
        );
    }

    #[test]
    fn test_hand_type_full_house() {
        assert_eq!(
            Hand::from_str("23332")
                .expect("failed to parse hand")
                .hand_type(false),
            HandType::FullHouse
        );
    }

    #[test]
    fn test_hand_type_three_of_a_kind() {
        assert_eq!(
            Hand::from_str("TTT98")
                .expect("failed to parse hand")
                .hand_type(false),
            HandType::ThreeOfAKind
        );
    }

    #[test]
    fn test_hand_type_two_pair() {
        assert_eq!(
            Hand::from_str("23432")
                .expect("failed to parse hand")
                .hand_type(false),
            HandType::TwoPair
        );
    }

    #[test]
    fn test_hand_type_one_pair() {
        assert_eq!(
            Hand::from_str("A23A4")
                .expect("failed to parse hand")
                .hand_type(false),
            HandType::OnePair
        );
    }

    #[test]
    fn test_hand_type_high_card() {
        assert_eq!(
            Hand::from_str("23456")
                .expect("failed to parse hand")
                .hand_type(false),
            HandType::HighCard
        );
    }

    #[test]
    fn test_compare_hands_without_jokers() {
        const ALLOW_JOKERS: bool = false;

        // `33332` starts with a higher card than `2AAAA`.
        assert_eq!(
            Hand::from_str("33332").expect("failed to parse hand").cmp(
                &Hand::from_str("2AAAA").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Greater
        );

        // Same as before but reversing the comparison.
        assert_eq!(
            Hand::from_str("2AAAA").expect("failed to parse hand").cmp(
                &Hand::from_str("33332").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Less
        );

        // `777JJ` starts with a lower card than `77888`.
        assert_eq!(
            Hand::from_str("777JJ").expect("failed to parse hand").cmp(
                &Hand::from_str("77888").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Less
        );

        // Both inputs are equal.
        assert_eq!(
            Hand::from_str("32T3K").expect("failed to parse hand").cmp(
                &Hand::from_str("32T3K").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Equal
        );

        // Five of a kind is better than four of a kind.
        assert_eq!(
            Hand::from_str("AAAAA").expect("failed to parse hand").cmp(
                &Hand::from_str("AA8AA").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Greater
        );

        // Full house is better than three of a kind.
        assert_eq!(
            Hand::from_str("J333J").expect("failed to parse hand").cmp(
                &Hand::from_str("TTT98").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Greater
        );
    }

    #[test]
    fn test_compare_hands_with_jokers() {
        const ALLOW_JOKERS: bool = true;

        // `33332` starts with a higher card than `2AAAA`.
        assert_eq!(
            Hand::from_str("33332").expect("failed to parse hand").cmp(
                &Hand::from_str("2AAAA").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Greater
        );

        // Same as before but reversing the comparison.
        assert_eq!(
            Hand::from_str("2AAAA").expect("failed to parse hand").cmp(
                &Hand::from_str("33332").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Less
        );

        // `777JJ` starts with a lower card than `77888`.
        assert_eq!(
            Hand::from_str("777JJ").expect("failed to parse hand").cmp(
                &Hand::from_str("77888").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Less
        );

        // Both inputs are equal.
        assert_eq!(
            Hand::from_str("32T3K").expect("failed to parse hand").cmp(
                &Hand::from_str("32T3K").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Equal
        );

        // Five of a kind is better than four of a kind.
        assert_eq!(
            Hand::from_str("AAAAA").expect("failed to parse hand").cmp(
                &Hand::from_str("AA8AA").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Greater
        );

        // Full house is better than three of a kind.
        assert_eq!(
            Hand::from_str("J333J").expect("failed to parse hand").cmp(
                &Hand::from_str("TTT98").expect("failed to parse hand"),
                ALLOW_JOKERS
            ),
            Ordering::Greater
        );
    }
}
