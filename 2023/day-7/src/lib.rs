use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

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

impl Game {
    pub fn hand(&self) -> &Hand {
        &self.0
    }

    pub fn bid(&self) -> Bid {
        self.1
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
}
