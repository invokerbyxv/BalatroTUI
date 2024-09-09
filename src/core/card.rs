use std::{cmp::{Ordering, Reverse}, collections::HashMap, fmt::{Display, Formatter, Result as FmtResult}, ops::{Add, Sub}};

use itertools::Itertools;
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

#[derive(Clone, Copy, Debug, EnumDisplay, EnumCount, EnumIter, EnumProperty, EnumString, Eq, Hash, IntoStaticStr, PartialEq)]
pub enum Rank {
    #[strum(serialize = "A", props(score = "10"))]
    Ace = 1,
    #[strum(serialize = "2", props(score = "2"))]
    Two,
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
}

impl Rank {
    #[inline]
    pub fn get_score(&self) -> usize {
        self.get_int("score").unwrap()
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        if *self == Rank::Ace {
            return Ordering::Greater;
        }
        if *other == Rank::Ace {
            return Ordering::Less;
        }
        (*self as usize).cmp(&(*other as usize))
    }
}

impl Add for Rank {
    type Output = usize;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        (self as usize) + (rhs as usize)
    }
}

impl Sub for Rank {
    type Output = isize;

    fn sub(self, rhs: Self) -> Self::Output {
        (self as isize) - (rhs as isize)
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
    #[must_use]
    fn sorted_by_suit(&self) -> Self;
    #[must_use]
    fn sorted_by_rank(&self) -> Self;
    #[must_use]
    fn grouped_by_suit(&self) -> Vec<(Suit, usize)>;
    #[must_use]
    fn grouped_by_rank(&self) -> Vec<(Rank, usize)>;
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
    fn sorted_by_suit(&self) -> Self {
        let mut cards = self.clone();
        cards.sort_by_suit();
        cards
    }

    #[inline]
    fn sorted_by_rank(&self) -> Self {
        let mut cards = self.clone();
        cards.sort_by_rank();
        cards
    }

    fn grouped_by_suit(&self) -> Vec<(Suit, usize)> {
        let group = self
            .iter()
            .fold(HashMap::new(), |mut groups, card| {
                groups.entry(card.suit).and_modify(|e| *e += 1).or_insert(1);
                groups
            })
            .into_iter()
            .sorted_by(|a, b| b.1.cmp(&a.1))
            .collect();
        group
    }

    fn grouped_by_rank(&self) -> Vec<(Rank, usize)> {
        let group = self
            .iter()
            .fold(HashMap::new(), |mut groups, card| {
                groups.entry(card.rank).and_modify(|e| *e += 1).or_insert(1);
                groups
            })
            .into_iter()
            .sorted_by(|a, b| b.1.cmp(&a.1))
            .collect();
        group
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_ranks() {
        let mut unsorted_ranks = vec![Rank::Seven, Rank::King, Rank::Two, Rank::Ace];

        let sorted_ranks = vec![Rank::Ace, Rank::King, Rank::Seven, Rank::Two];

        unsorted_ranks.sort();
        unsorted_ranks.reverse();

        assert_eq!(unsorted_ranks, sorted_ranks);
    }
}
