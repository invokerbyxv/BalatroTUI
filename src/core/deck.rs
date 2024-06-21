use std::{cmp::{min, Reverse}, error::Error, fmt::{Display, Formatter, Result as DisplayResult}, iter::zip};
use once_cell::sync::Lazy;
use rand::{thread_rng, seq::SliceRandom};
use itertools::{Either, Itertools};

use super::card::{Card, Suit};

// TODO: Use dynamic trait switching to achieve suit and rank sorting. Feed the impl directly to card instead of MultiSortable
// TODO: Impl default traits for all structs

static DEFAULT_DECK: Lazy<Vec<Card>> = Lazy::new(|| Suit::iter().flat_map(
    move |suit| (1..=13).map(
        move |rank| Card {
            suit,
            rank,
            score: min(if rank == 1 { 10 } else { rank }, 10),
        }
    )
).collect());

#[derive(Debug, Default, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
    pub selected: Vec<bool>,
}

impl Deck {
    #[inline]
    pub fn standard() -> Self {
        Self { cards: DEFAULT_DECK.to_vec(), ..Default::default() }
    }

    #[inline]
    pub fn shuffle(&mut self) {
        // TODO: Bias with seed
        self.cards.shuffle(&mut thread_rng());
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter) -> DisplayResult {
        writeln!(f, "")?;
        for (card, is_selected) in zip(self.cards.iter(), self.selected.iter()) {
            write!(f, "\t{}{}", card, if *is_selected { "*" } else { "" })?;
        }
        Ok(())
    }
}

pub trait Drawable {
    fn draw_random(&mut self, hand_size: usize) -> Result<Self, Box<dyn Error>> where Self : Sized;
}

impl Drawable for Deck {
    #[inline]
    fn draw_random(&mut self, hand_size: usize) -> Result<Self, Box<dyn Error>> {
        if hand_size > self.cards.len() {
            // TODO: Define custom error
            Err("Hand size cannot be greater than the source deck.")?
        }
        self.shuffle();
        Ok(Self {
            cards: self.cards.drain(self.cards.len() - hand_size..).collect(),
            selected: vec![false; hand_size],
        })
    }
}

pub trait Selectable where Self : Drawable {
    fn select(&mut self, selection: usize);
    fn deselect(&mut self, selection: usize);
    fn peek_selected(&self) -> Result<Vec<Card>, Box<dyn Error>>;
    fn draw_selected(&mut self) -> Result<Vec<Card>, Box<dyn Error>>;
}

impl Selectable for Deck {
    #[inline]
    fn select(&mut self, selection: usize) {
        self.selected[selection] = true;
    }

    #[inline]
    fn deselect(&mut self, selection: usize) {
        self.selected[selection] = false;
    }

    #[inline]
    fn peek_selected(&self) -> Result<Vec<Card>, Box<dyn Error>> {
        Ok(zip(self.cards.iter().cloned(), self.selected.iter().cloned()).take_while(|(_, is_selected)| *is_selected).map(|(card, _)| card).collect())
    }

    #[inline]
    fn draw_selected(&mut self) -> Result<Vec<Card>, Box<dyn Error>> {
        let (selected, leftover): (Vec<_>, Vec<_>) = zip(self.cards.iter().cloned(), self.selected.iter().cloned()).partition_map(|(card, is_selected)| {
            if is_selected {
                Either::Left(card)
            } else {
                Either::Right(card)
            }
        });
        self.selected = vec![false; leftover.len()];
        self.cards = leftover;
        Ok(selected)
    }
}

pub trait Sortable {
    fn sort_by_suit(&mut self);
    fn sort_by_rank(&mut self);
}

impl Sortable for Deck {
    #[inline]
    fn sort_by_suit(&mut self) {
        self.cards.sort_by_key(|c| (c.suit, Reverse(c.rank)));
    }

    #[inline]
    fn sort_by_rank(&mut self) {
        self.cards.sort_by_key(|c| (Reverse(c.rank), c.suit));
    }
}
