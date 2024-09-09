use std::error::Error;

use strum::{Display, EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr};

use super::card::{Card, Rank, Sortable};

#[derive(Clone, Copy, Debug, Default, Display, EnumCount, EnumIter, EnumProperty, EnumString, Eq, Hash, IntoStaticStr, Ord, PartialEq, PartialOrd)]
pub enum ScoringHand {
    #[default]
    #[strum(serialize = "")]
    None = 0,
    #[strum(serialize = "Flush Five", props(chips = "160", multiplier = "16"))]
    FlushFive,
    #[strum(serialize = "Flush House", props(chips = "140", multiplier = "14"))]
    FlushHouse,
    #[strum(serialize = "Five of a Kind", props(chips = "120", multiplier = "12"))]
    FiveOfAKind,
    #[strum(serialize = "Royal Flush", props(chips = "100", multiplier = "8"))]
    RoyalFlush,
    #[strum(serialize = "Straight Flush", props(chips = "60", multiplier = "7"))]
    StraightFlush,
    #[strum(serialize = "Four of a Kind", props(chips = "40", multiplier = "4"))]
    FourOfAKind,
    #[strum(serialize = "Full House", props(chips = "35", multiplier = "4"))]
    FullHouse,
    #[strum(serialize = "Flush", props(chips = "30", multiplier = "4"))]
    Flush,
    #[strum(serialize = "Straight", props(chips = "30", multiplier = "3"))]
    Straight,
    #[strum(serialize = "Three of a Kind", props(chips = "20", multiplier = "2"))]
    ThreeOfAKind,
    #[strum(serialize = "Two Pair", props(chips = "20", multiplier = "2"))]
    TwoPair,
    #[strum(serialize = "Pair", props(chips = "10", multiplier = "2"))]
    Pair,
    #[strum(serialize = "High Card", props(chips = "5", multiplier = "1"))]
    HighCard,
}

#[derive(Clone)]
pub struct StraightTestResult {
    pub has_ace: bool,
    pub scored_ranks: Vec<Rank>,
    pub high_ace: Option<bool>,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub struct Scorer;

impl Scorer {
    #[inline]
    pub fn get_chips_and_multiplier(
        scoring_hand: ScoringHand,
    ) -> Result<(usize, usize), Box<dyn Error>> {
        Ok((
            scoring_hand.get_int("chips").unwrap(),
            scoring_hand.get_int("multiplier").unwrap(),
        ))
    }

    fn test_straight(cards: &Vec<Card>) -> Option<StraightTestResult> {
        let has_ace = cards.iter().any(|card| card.rank == Rank::Ace);
        let mut ranks_without_ace = cards
            .iter()
            .map(|card| card.rank)
            .filter(|rank| rank != &Rank::Ace)
            .collect::<Vec<_>>();
        let comparator = ranks_without_ace
            .iter()
            .map(|rank| ranks_without_ace[0] - *rank)
            .collect::<Vec<_>>();

        if comparator.len() < 4 {
            return None;
        }

        if comparator.eq(&[0, 1, 2, 3, 4]) {
            return Some(StraightTestResult {
                has_ace,
                high_ace: None,
                scored_ranks: ranks_without_ace,
            });
        }

        let is_partial_straight = comparator.eq(&[0, 1, 2, 3]);
        let is_low_straight =
            has_ace && is_partial_straight && cards.iter().any(|card| card.rank == Rank::Two);
        let is_high_straight =
            has_ace && is_partial_straight && cards.iter().any(|card| card.rank == Rank::King);

        if is_low_straight {
            ranks_without_ace.push(Rank::Ace);
            return Some(StraightTestResult {
                has_ace,
                high_ace: Some(false),
                scored_ranks: ranks_without_ace,
            });
        }

        if is_high_straight {
            ranks_without_ace.insert(0, Rank::Ace);
            return Some(StraightTestResult {
                has_ace,
                high_ace: Some(true),
                scored_ranks: ranks_without_ace,
            });
        }

        None
    }

    pub fn get_scoring_hand(cards: &Vec<Card>) -> Result<(ScoringHand, Vec<Rank>), Box<dyn Error>> {
        let sorted_cards = cards.sorted_by_rank();
        let suit_groups = sorted_cards.grouped_by_suit();
        let rank_groups = sorted_cards.grouped_by_rank();
        let straight_test_result = Self::test_straight(&sorted_cards);

        if suit_groups.is_empty() || rank_groups.is_empty() {
            return Ok((ScoringHand::None, vec![]));
        }

        if suit_groups[0].1 == 5 && rank_groups[0].1 == 5 {
            return Ok((
                ScoringHand::FlushFive,
                vec![rank_groups[0].0; rank_groups[0].1],
            ));
        }

        if rank_groups.len() >= 2
            && suit_groups[0].1 == 5
            && rank_groups[0].1 == 3
            && rank_groups[1].1 == 2
        {
            let mut played_ranks = vec![];
            played_ranks.append(&mut vec![rank_groups[0].0; rank_groups[0].1]);
            played_ranks.append(&mut vec![rank_groups[1].0; rank_groups[1].1]);
            return Ok((ScoringHand::FlushHouse, played_ranks));
        }

        if rank_groups[0].1 == 5 {
            return Ok((
                ScoringHand::FiveOfAKind,
                vec![rank_groups[0].0; rank_groups[0].1],
            ));
        }

        if suit_groups[0].1 == 5 && straight_test_result.as_ref().is_some_and(|result| result.has_ace) {
            return Ok((ScoringHand::RoyalFlush, straight_test_result.unwrap().scored_ranks));
        }

        if suit_groups[0].1 == 5 && straight_test_result.as_ref().is_some() {
            return Ok((
                ScoringHand::StraightFlush,
                straight_test_result.unwrap().scored_ranks,
            ));
        }

        if rank_groups[0].1 == 4 {
            return Ok((
                ScoringHand::FourOfAKind,
                vec![rank_groups[0].0; rank_groups[0].1],
            ));
        }

        if rank_groups.len() >= 2 && rank_groups[0].1 == 3 && rank_groups[1].1 == 2 {
            let mut played_ranks = vec![];
            played_ranks.append(&mut vec![rank_groups[0].0; rank_groups[0].1]);
            played_ranks.append(&mut vec![rank_groups[1].0; rank_groups[1].1]);
            return Ok((ScoringHand::FullHouse, played_ranks));
        }

        if suit_groups[0].1 == 5 {
            return Ok((
                ScoringHand::Flush,
                cards.iter().map(|card| card.rank).collect(),
            ));
        }

        if straight_test_result.as_ref().is_some() {
            return Ok((ScoringHand::Straight, straight_test_result.unwrap().scored_ranks));
        }

        if rank_groups[0].1 == 3 {
            return Ok((
                ScoringHand::ThreeOfAKind,
                vec![rank_groups[0].0; rank_groups[0].1],
            ));
        }

        if rank_groups.len() >= 2 && rank_groups[0].1 == 2 && rank_groups[1].1 == 2 {
            let mut played_ranks = vec![];
            played_ranks.append(&mut vec![rank_groups[0].0; rank_groups[0].1]);
            played_ranks.append(&mut vec![rank_groups[1].0; rank_groups[1].1]);
            return Ok((ScoringHand::TwoPair, played_ranks));
        }

        if rank_groups[0].1 == 2 {
            return Ok((ScoringHand::Pair, vec![rank_groups[0].0; rank_groups[0].1]));
        }

        Ok((
            ScoringHand::HighCard,
            vec![rank_groups[0].0; rank_groups[0].1],
        ))
    }

    pub fn score_cards(cards: &Vec<Card>) -> Result<usize, Box<dyn Error>> {
        let (scoring_hand, scored_ranks) = Self::get_scoring_hand(&cards)?;
        let (base_chips, multiplier) = Self::get_chips_and_multiplier(scoring_hand)?;
        let chips_increment = Self::score_chips_from_ranks(&scored_ranks)?;
        Ok((base_chips + chips_increment) * multiplier)
    }

    #[inline]
    pub fn score_chips_from_ranks(ranks: &Vec<Rank>) -> Result<usize, Box<dyn Error>> {
        Ok(ranks.iter().fold(0, |acc, rank| acc + rank.get_score()))
    }
}

// TODO: Add more tests for core functionality

#[cfg(test)]
mod tests {
    use crate::core::card::Suit;

    use super::*;

    #[test]
    fn score_flush_five() {
        let test_cards = vec![
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::FlushFive
        );
    }

    #[test]
    fn score_flush_house() {
        let test_cards = vec![
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Club,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::FlushHouse
        );
    }

    #[test]
    fn score_five_of_a_kind() {
        let test_cards = vec![
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::FiveOfAKind
        );
    }

    #[test]
    fn score_royal_flush() {
        let test_cards = vec![
            Card {
                rank: Rank::Queen,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Club,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::RoyalFlush
        );
    }

    #[test]
    fn score_straight_flush() {
        let test_cards = vec![
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Club,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::StraightFlush
        );
    }

    #[test]
    fn score_four_of_a_kind() {
        let test_cards = vec![
            Card {
                rank: Rank::Seven,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Club,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::FourOfAKind
        );
    }

    #[test]
    fn score_full_house() {
        let test_cards = vec![
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::FullHouse
        );
    }

    #[test]
    fn score_flush() {
        let test_cards = vec![
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Club,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::Flush
        );
    }

    #[test]
    fn score_non_ace_straight() {
        let test_cards = vec![
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Club,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::Straight
        );
    }

    #[test]
    fn score_low_ace_straight() {
        let test_cards = vec![
            Card {
                rank: Rank::Four,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Club,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::Straight
        );
    }

    #[test]
    fn score_high_ace_straight() {
        let test_cards = vec![
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Jack,
                suit: Suit::Club,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::Straight
        );
    }

    #[test]
    fn score_mid_ace_straight_false_positive() {
        let test_cards = vec![
            Card {
                rank: Rank::Two,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::King,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Club,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::HighCard
        );
    }

    #[test]
    fn score_three_of_a_kind() {
        let test_cards = vec![
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::ThreeOfAKind
        );
    }

    #[test]
    fn score_two_pair() {
        let test_cards = vec![
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::TwoPair
        );
    }

    #[test]
    fn score_pair() {
        let test_cards = vec![
            Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::Pair
        );
    }

    #[test]
    fn score_high_card() {
        let test_cards = vec![
            Card {
                rank: Rank::Jack,
                suit: Suit::Club,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
            },
            Card {
                rank: Rank::Seven,
                suit: Suit::Heart,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Spade,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            },
        ];

        assert_eq!(
            Scorer::get_scoring_hand(&test_cards).unwrap().0,
            ScoringHand::HighCard
        );
    }
}
