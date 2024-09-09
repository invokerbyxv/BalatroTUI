use std::{cmp::Reverse, fmt::{Display, Formatter, Result as FmtResult}};

use strum::{Display as EnumDisplay, EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr};

#[derive(Clone, Copy, Debug, EnumDisplay, EnumCount, EnumIter, EnumString, Eq, Hash, IntoStaticStr, Ord, PartialEq, PartialOrd)]
pub enum Suit {
    #[strum(serialize = "♣")]
    Club,
    #[strum(serialize = "♦")]
    Diamond,
    #[strum(serialize = "♥")]
    Heart,
    #[strum(serialize = "♠")]
    Spade,
}

#[derive(Clone, Copy, Debug, EnumDisplay, EnumCount, EnumIter, EnumProperty, EnumString, Eq, Hash, IntoStaticStr, Ord, PartialEq, PartialOrd)]
pub enum Rank {
    #[strum(serialize = "2", props(score = "2"))]
    Two = 2,
    #[strum(serialize = "3", props(score = "3"))]
    Three,
    #[strum(serialize = "4", props(score = "4"))]
    Four,
    #[strum(serialize = "5", props(score = "5"))]
    Five,
    #[strum(serialize = "6", props(score = "6"))]
    Six,
    #[strum(serialize = "7", props(score = "7"))]
    Seven,
    #[strum(serialize = "8", props(score = "8"))]
    Eight,
    #[strum(serialize = "9", props(score = "9"))]
    Nine,
    #[strum(serialize = "10", props(score = "10"))]
    Ten,
    #[strum(serialize = "J", props(score = "10"))]
    Jack,
    #[strum(serialize = "Q", props(score = "10"))]
    Queen,
    #[strum(serialize = "K", props(score = "10"))]
    King,
    #[strum(serialize = "A", props(score = "10"))]
    Ace,
}

impl Rank {
    #[inline]
    pub fn get_score(&self) -> usize {
        self.get_int("score").unwrap()
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Display for Card {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

pub trait Sortable {
    fn sort_by_suit(&mut self);
    fn sort_by_rank(&mut self);
    fn sorted_by_suit(self) -> Self;
    fn sorted_by_rank(self) -> Self;
}

impl Sortable for Vec<Card> {
    #[inline]
    fn sort_by_suit(&mut self) {
        self.sort_by_key(|c| (c.suit, Reverse(c.rank)));
    }

    #[inline]
    fn sort_by_rank(&mut self) {
        self.sort_by_key(|c| (Reverse(c.rank), c.suit));
    }

    #[inline]
    fn sorted_by_suit(self) -> Self {
        let mut cards = self.clone();
        cards.sort_by_suit();
        cards
    }

    #[inline]
    fn sorted_by_rank(self) -> Self {
        let mut cards = self.clone();
        cards.sort_by_rank();
        cards
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