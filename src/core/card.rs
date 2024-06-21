use std::{array::IntoIter, cmp::Ordering, fmt::{Display, Formatter, Result}};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    const VALUES: [Self; 4] = [Self::Club, Self::Diamond, Self::Heart, Self::Spade];

    #[inline]
    pub fn iter() -> IntoIter<Suit, 4> {
        Self::VALUES.into_iter()
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let suit_display = match *self {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };
        write!(f, "{}", suit_display)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: usize,
    pub score: usize,
    pub suit: Suit,
}

impl PartialOrd for Card {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank).then(self.suit.cmp(&other.suit))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let rank_display = match self.rank {
            // NOTE: Probably there's a better way to handle this with static str.
            1 => "A".to_owned(),
            13 => "K".to_owned(),
            12 => "Q".to_owned(),
            11 => "J".to_owned(),
            _ => self.rank.to_string(),
        };
        write!(f, "{}{}", self.suit, rank_display)
    }
}
