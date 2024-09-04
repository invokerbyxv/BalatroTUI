use std::fmt::{Display, Formatter, Result as FmtResult};

use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, Hash, PartialEq, EnumIter)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Display for Suit {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let suit_display = match *self {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };
        write!(f, "{}", suit_display)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, Hash, PartialEq, EnumIter)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    #[inline]
    pub fn get_score(&self) -> usize {
        match *self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 10,
        }
    }
}

impl Display for Rank {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let rank_display = match *self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{}", rank_display)
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_ranks() {
        let mut unsorted_cards = vec![
            Rank::Seven,
            Rank::King,
            Rank::Two,
            Rank::Ace,
        ];

        let sorted_cards = vec![
            Rank::Ace,
            Rank::King,
            Rank::Seven,
            Rank::Two,
        ];

        unsorted_cards.sort();
        unsorted_cards.reverse();

        assert_eq!(unsorted_cards, sorted_cards);
    }
}