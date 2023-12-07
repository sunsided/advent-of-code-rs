use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// A marker used for jokers in part 2.
const JOKER_MARKER: char = '*';

/// Solution for part 1 and 2.
pub fn total_winnings(input: &str, jokers: Jokers) -> u64 {
    let mut games: Vec<_> = input
        .lines()
        .map(|line| Game::from_str(line, jokers).expect("invalid input"))
        .collect();
    games.sort_by(|lhs, rhs| lhs.hand().cmp(rhs.hand()));

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

/// Whether or not to allow jokers.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Jokers {
    /// Jokers are disallowed (for part 1).
    Disallowed,
    /// Jokers are allowed (for part 2).
    Allowed,
}

/// A card.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Card {
    /// Card value `1` (for part 2).
    Joker,
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
    /// Card value `J` (for part 1).
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

    pub fn from_str(input: &str, jokers: Jokers) -> Result<Self, ParseGameError> {
        let s = input.trim();
        let mut lines = s.split_whitespace();
        let hand = Hand::from_str(
            lines
                .next()
                .ok_or(ParseGameError("Invalid game input when reading hand"))?,
            jokers,
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

impl Hand {
    /// Determines the hand type with or without allowing jokers.
    pub fn hand_type(&self) -> HandType {
        Self::hand_from_card_count(self.count_cards())
    }

    fn from_str(s: &str, jokers: Jokers) -> Result<Self, ParseHandError> {
        let s = s.trim();
        if s.len() != 5 {
            return Err(ParseHandError::InvalidLength(s.len()));
        }

        let allow_jokers = jokers == Jokers::Allowed;
        let map_jokers = |c| {
            if !allow_jokers {
                c
            } else if c == 'J' {
                JOKER_MARKER
            } else {
                c
            }
        };

        let mut cards = [Card::Two; 5];
        for (i, ch) in s.chars().map(map_jokers).enumerate() {
            cards[i] = ch.try_into().map_err(ParseHandError::InvalidCard)?;
        }

        Ok(Self(cards))
    }

    fn count_cards(&self) -> Vec<(Card, usize)> {
        let mut counts = [0_usize; Card::NUM_CARDS];
        for card in &self.0 {
            counts[card.index()] += 1;
        }

        // There are at most five different cards per hand.
        let mut counted = Vec::with_capacity(5);

        for (card, count) in counts
            .into_iter()
            .enumerate()
            .rev()
            .filter(|(_, count)| *count > 0)
            .map(|(index, count)| (Card::from_index(index), count))
        {
            counted.push((card, count));
        }

        // Sort by count in descending order.
        counted.sort_by_key(|(_, count)| 5 - *count);
        counted
    }

    /// Determines the hand from the card count.
    ///
    /// # Arguments
    /// * `counted` - The counted cards, sorted by count descending (i.e. highest count first).
    fn hand_from_card_count(mut counted: Vec<(Card, usize)>) -> HandType {
        let highest_count = counted[0].1;

        // Fiddle around with jokers. If all five cards are jokers, no action is required as
        // it's a five of a kind either way.
        if highest_count != 5 {
            if let Some((joker_index, (_, num_jokers))) = counted
                .iter()
                .enumerate()
                .find(|(_, (card, _))| *card == Card::Joker)
            {
                // If the first card is the joker, the best card follows immediately after.
                let best_index = if joker_index > 0 { 0 } else { 1 };

                // Add the joker count to the best card. This is the optimal strategy, see
                // comments below for possible scenarios.
                let (card, count) = counted[best_index];
                counted[best_index] = (card, count + num_jokers);

                // Remove the joker from the game.
                counted.remove(joker_index);
            }
        }

        match counted.as_slice() {
            // All cards are the same.
            [(_, 5)] => HandType::FiveOfAKind,
            // Two distinct group of cards, one of them with four entries, e.g. `AA8AA` (four of a kind)
            // A single joker makes this a Five of a kind.
            [(_, 4), (_, 1)] => HandType::FourOfAKind,
            // Two distinct group of cards, one of them with three entries, e.g. `23332` (full house)
            // A single joker makes this a four of a kind (4,1).
            // Two jokers make it a five of a kind (5).
            [(_, 3), (_, 2)] => HandType::FullHouse,
            // Three distinct groups, one of them with three cards, e.g. `TTT98` (three of a kind)
            // A single joker makes this either a four of a kind (4,1 - optimal) or a Full house (3,2).
            [(_, 3), (_, 1), (_, 1)] => HandType::ThreeOfAKind,
            // Three distinct groups, two of them with two cards, e.g. `23432` (two pair)
            // A single joker makes this either a Full house (3,2).
            // Two jokers make this a Four of a kind (4,1).
            [(_, 2), (_, 2), (_, 1)] => HandType::TwoPair,
            // One pair and three distinct cards, e.g. `A23A4`.
            // A single joker makes this a Three of a kind (3,1,1 - optimal) or a Two pair (2,2,1).
            [(_, 2), (_, 1), (_, 1), (_, 1)] => HandType::OnePair,
            // All cards are different, e.g. `23456`.
            // A single joker makes this a One pair (4,1,1,1).
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => HandType::HighCard,
            // No other combination is allowed.
            _ => unreachable!(),
        }
    }
}

impl Card {
    const NUM_CARDS: usize = 14;

    const CARDS: [Card; Self::NUM_CARDS] = [
        Card::Joker,
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
            Card::Joker => 0,
            Card::Two => 1,
            Card::Three => 2,
            Card::Four => 3,
            Card::Five => 4,
            Card::Six => 5,
            Card::Seven => 6,
            Card::Eight => 7,
            Card::Nine => 8,
            Card::T => 9,
            Card::J => 10,
            Card::Q => 11,
            Card::K => 12,
            Card::A => 13,
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

impl TryFrom<char> for Card {
    type Error = ParseCardError;

    fn try_from(value: char) -> Result<Self, ParseCardError> {
        match value {
            JOKER_MARKER => Ok(Self::Joker), // for part 2
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::T),
            'J' => Ok(Self::J), // for part 1
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            _ => Err(ParseCardError("Invalid character")),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // First rule: The higher hand type wins.
        let hand = self.hand_type().cmp(&other.hand_type());
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
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
            Hand::from_str("32T3K", Jokers::Disallowed),
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
            Hand::from_str(" 32T3K ", Jokers::Disallowed),
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
            Hand::from_str("32T345", Jokers::Disallowed),
            Err(ParseHandError::InvalidLength(6))
        );

        // Invalid card in input.
        assert_eq!(
            Hand::from_str("32T3X", Jokers::Disallowed),
            Err(ParseHandError::InvalidCard(ParseCardError(
                "Invalid character"
            )))
        );
    }

    #[test]
    fn test_parse_hand_with_jokers() {
        // J inputs are treated as J cards. No jokers for this game.
        assert_eq!(
            Hand::from_str("JJJJJ", Jokers::Disallowed),
            Ok(Hand([Card::J, Card::J, Card::J, Card::J, Card::J]))
        );

        // J inputs are parsed as jokers. No J cards for this game.
        assert_eq!(
            Hand::from_str("JJJJJ", Jokers::Allowed),
            Ok(Hand([
                Card::Joker,
                Card::Joker,
                Card::Joker,
                Card::Joker,
                Card::Joker
            ]))
        );
    }

    #[test]
    fn test_parse_game() {
        let game = Game::from_str("KK677 28 ", Jokers::Disallowed).expect("parsing failed");
        assert_eq!(
            game.hand(),
            &Hand([Card::K, Card::K, Card::Six, Card::Seven, Card::Seven])
        );
        assert_eq!(game.bid(), Bid(28));
    }

    #[test]
    fn test_hand_type_five_of_a_kind() {
        assert_eq!(
            Hand::from_str("AAAAA", Jokers::Disallowed)
                .expect("failed to parse hand")
                .hand_type(),
            HandType::FiveOfAKind
        );
    }

    #[test]
    fn test_hand_type_four_of_a_kind() {
        assert_eq!(
            Hand::from_str("AA8AA", Jokers::Disallowed)
                .expect("failed to parse hand")
                .hand_type(),
            HandType::FourOfAKind
        );
    }

    #[test]
    fn test_hand_type_full_house() {
        assert_eq!(
            Hand::from_str("23332", Jokers::Disallowed)
                .expect("failed to parse hand")
                .hand_type(),
            HandType::FullHouse
        );
    }

    #[test]
    fn test_hand_type_three_of_a_kind() {
        assert_eq!(
            Hand::from_str("TTT98", Jokers::Disallowed)
                .expect("failed to parse hand")
                .hand_type(),
            HandType::ThreeOfAKind
        );
    }

    #[test]
    fn test_hand_type_two_pair() {
        assert_eq!(
            Hand::from_str("23432", Jokers::Disallowed)
                .expect("failed to parse hand")
                .hand_type(),
            HandType::TwoPair
        );
    }

    #[test]
    fn test_hand_type_one_pair() {
        assert_eq!(
            Hand::from_str("A23A4", Jokers::Disallowed)
                .expect("failed to parse hand")
                .hand_type(),
            HandType::OnePair
        );
    }

    #[test]
    fn test_hand_type_high_card() {
        assert_eq!(
            Hand::from_str("23456", Jokers::Disallowed)
                .expect("failed to parse hand")
                .hand_type(),
            HandType::HighCard
        );
    }

    #[test]
    fn test_compare_hands_without_jokers() {
        // `33332` starts with a higher card than `2AAAA`.
        assert_eq!(
            Hand::from_str("33332", Jokers::Disallowed)
                .expect("failed to parse hand")
                .cmp(&Hand::from_str("2AAAA", Jokers::Disallowed).expect("failed to parse hand")),
            Ordering::Greater
        );

        // Same as before but reversing the comparison.
        assert_eq!(
            Hand::from_str("2AAAA", Jokers::Disallowed)
                .expect("failed to parse hand")
                .cmp(&Hand::from_str("33332", Jokers::Disallowed).expect("failed to parse hand")),
            Ordering::Less
        );

        // `777JJ` starts with a lower card than `77888`.
        assert_eq!(
            Hand::from_str("777JJ", Jokers::Disallowed)
                .expect("failed to parse hand")
                .cmp(&Hand::from_str("77888", Jokers::Disallowed).expect("failed to parse hand")),
            Ordering::Less
        );

        // Both inputs are equal.
        assert_eq!(
            Hand::from_str("32T3K", Jokers::Disallowed)
                .expect("failed to parse hand")
                .cmp(&Hand::from_str("32T3K", Jokers::Disallowed).expect("failed to parse hand")),
            Ordering::Equal
        );

        // Five of a kind is better than four of a kind.
        assert_eq!(
            Hand::from_str("AAAAA", Jokers::Disallowed)
                .expect("failed to parse hand")
                .cmp(&Hand::from_str("AA8AA", Jokers::Disallowed).expect("failed to parse hand")),
            Ordering::Greater
        );

        // Full house is better than three of a kind.
        assert_eq!(
            Hand::from_str("J333J", Jokers::Disallowed)
                .expect("failed to parse hand")
                .cmp(&Hand::from_str("TTT98", Jokers::Disallowed).expect("failed to parse hand")),
            Ordering::Greater
        );
    }

    #[test]
    fn test_hand_type_with_jokers() {
        assert_eq!(
            Hand::from_str("T55J5", Jokers::Allowed)
                .expect("failed to parse hand")
                .hand_type(),
            HandType::FourOfAKind
        );

        assert_eq!(
            Hand::from_str("KTJJT", Jokers::Allowed)
                .expect("failed to parse hand")
                .hand_type(),
            HandType::FourOfAKind
        );

        assert_eq!(
            Hand::from_str("QQQJA", Jokers::Allowed)
                .expect("failed to parse hand")
                .hand_type(),
            HandType::FourOfAKind
        );
    }
}
