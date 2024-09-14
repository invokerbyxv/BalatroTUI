//! This module provides [`Deck`] as a primitive alias and [`TrackableDeck`] as
//! a wrapper to provide cycling cursor and selection of cards.
//!
//! This module also provides deck management methods and card tracking for UI
//! states. To utilize methods described on [`Deck`] and [`TrackableDeck`],
//! [`DeckConstExt`] and [`DeckExt`] traits must be brought into scope.

use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

use color_eyre::eyre::{bail, eyre, Context, OptionExt, Result};
use crossterm::event::KeyCode;
use cursorvec::CursorVec;
use itertools::{Either, Itertools};
use once_cell::sync::Lazy;
use rand::{seq::SliceRandom, thread_rng};
use ratatui::{
    layout::{Constraint, Layout, Offset, Rect},
    Frame,
};
use strum::IntoEnumIterator;

use super::card::{Card, Rank, Suit};
use crate::{
    components::{CardVisualState, CardWidget, CardWidgetState},
    event::Event,
    tui::TuiComponent,
};

// TODO: Use dynamic trait switching to achieve suit and rank sorting. Feed the
// impl directly to card instead of MultiSortable.
// TODO: Impl default traits for all structs.

/// Maximum selectable cards to form a hand.
///
/// As per standard rules this is set to `5`.
pub const MAXIMUM_SELECTED_CARDS: usize = 5;

/// Lazy initializer for default deck.
///
/// More decks can be added using lazy initialization with use of
/// [`super::card::SuitIter`] and [`super::card::RankIter`].
pub static DEFAULT_DECK: Lazy<Deck> = Lazy::new(|| {
    Rank::iter()
        .cartesian_product(Suit::iter())
        .map(|(rank, suit)| Card { rank, suit })
        .collect()
});

/// [`Deck`] type is re-exported as an alias to [`Vec<Card>`] for better
/// contextual understanding.
pub type Deck = Vec<Card>;

/// Constructor extension trait for [`Deck`].
///
/// This trait only consists of construction function for various decks. Since,
/// this trait is implemented directly on foreign type [`Vec<Card>`], it gains
/// static methods to create decks on import.
pub trait DeckConstExt {
    /// Return a standard 52 card deck.
    #[must_use = "Created deck must be used."]
    fn standard() -> Deck;
}

/// Extension methods for [`Deck`], directly implemented on top of
/// [`Vec<Card>`].
pub trait DeckExt {
    /// In-place shuffle a deck based on thread/seed rng.
    fn shuffle(&mut self);
    /// Draw random cards from the deck and return new deck.
    #[must_use = "Drawn cards must be used."]
    fn draw_random(&mut self, draw_size: usize) -> Result<Deck>;
}

impl DeckConstExt for Deck {
    #[inline]
    fn standard() -> Self {
        DEFAULT_DECK.to_vec()
    }
}

impl DeckExt for Deck {
    #[inline]
    fn shuffle(&mut self) {
        // TODO: Bias with seed
        self.as_mut_slice().shuffle(&mut thread_rng());
    }

    fn draw_random(&mut self, draw_size: usize) -> Result<Deck> {
        if draw_size > self.len() {
            // TODO: Define custom error
            bail!("Hand size cannot be greater than the source deck.")
        }
        self.shuffle();

        let drain_size = self
            .len()
            .checked_sub(draw_size)
            .ok_or_eyre("Subtraction operation overflowed")?;
        let drawn_cards = self.drain(drain_size..).collect();

        Ok(drawn_cards)
    }
}

/// A deck implementation with tracking for hovered and selected elements.
///
/// Exchange between [`Deck`] and [`TrackableDeck`] is facilitated by
/// implementation of [`From<Deck>`] trait, allowing for direct conversion.
///
/// ```
/// let cards = vec![
///     Card { rank: Rank::Ace, suit: Suit::Club },
///     Card { rank: Rank::Six, suit: Suit::Heart },
///     Card { rank: Rank::King, suit: Suit::Diamond },
///     Card { rank: Rank::Nine, suit: Suit::Spade },
/// ]
///
/// let trackable_deck: TrackableDeck = cards.into();
///
/// trackable_deck.select(0);
/// trackable_deck.select(2);
///
/// assert_eq!(trackable_deck.peek_selected().unwrap(), vec![
///     Card { rank: Rank::Ace, suit: Suit::Club },
///     Card { rank: Rank::King, suit: Suit::Diamond },
/// ])
/// ```
#[derive(Debug)]
pub struct TrackableDeck {
    /// A circular bidirectional iterator over a deck of cards.
    pub cards: CursorVec<Card>,
    /// A cache of selected card indices.
    pub selected: HashSet<usize>,
}

/// [`TrackableDeck`] uses [`Deref`] and [`DerefMut`] to dereference into
/// [`Vec<Card>`]. [`Deck`] associated methods can be used this way. Usage of
/// [`DeckExt`] methods with [`TrackableDeck`] assumes the tracking can be
/// discarded for the operation. Upon call, [`Self::selected`] state and
/// [`Self::cards::cursor`] will be reset to default.
impl DeckExt for TrackableDeck {
    /// Shuffle assumes the cards are no longer needed to be tracked and clears
    /// out tracking information.
    #[inline]
    fn shuffle(&mut self) {
        self.cards.shuffle();
        self.cards.update_cursor();
        self.selected.clear();
    }

    /// Drawing random cards assumes that the hover and selection orders are no
    /// longer needed to be tracked and clears out tracking information.
    ///
    /// Returns a [`Deck`] of cards of required draw size.
    #[inline]
    fn draw_random(&mut self, draw_size: usize) -> Result<Deck> {
        let cards = self.cards.draw_random(draw_size)?;
        self.cards.update_cursor();
        self.selected.clear();
        Ok(cards)
    }
}

impl TrackableDeck {
    /// Selects a [`Card`] at the index to be used in played hand. No-op if the
    /// index is already selected.
    ///
    /// <div class="warning">Changes to underlying data might invalidate the
    /// selection. It is thus better to clear the selection before modifying the
    /// underlying cards.</div>
    #[inline]
    pub fn select(&mut self, selection: usize) {
        _ = self.selected.insert(selection);
    }

    /// Removes selection of a [`Card`] at the index from the playing hand.
    /// No-op if the index is already deselected.
    ///
    /// <div class="warning">Changes to underlying data might invalidate the
    /// selection. It is thus better to clear the selection before modifying the
    /// underlying cards.</div>
    #[inline]
    pub fn deselect(&mut self, selection: usize) {
        _ = self.selected.remove(&selection);
    }

    /// Returns the peek preview clone of the selected cards in a deck. This
    /// does not modify the underlying data. Creates a borrow on cards and
    /// returns a clone deck.
    pub fn peek_selected(&self) -> Result<Deck> {
        self.selected
            .iter()
            .map(|&idx| {
                self.cards
                    .get(idx)
                    .copied()
                    .ok_or_eyre(format!("Cannot get card at the selected index: {idx}"))
            })
            .collect::<Result<Deck>>()
    }

    /// Draws the selected cards into a new deck. The cards are removed from the
    /// original deck. Tracking positions are reset on this call.
    pub fn draw_selected(&mut self) -> Result<Deck> {
        let (selected, leftover): (Deck, Deck) =
            self.cards.iter().enumerate().partition_map(|(idx, card)| {
                if self.selected.contains(&idx) {
                    Either::Left(card)
                } else {
                    Either::Right(card)
                }
            });

        self.cards.set_container(leftover);
        self.cards.update_cursor();
        self.selected.clear();

        Ok(selected)
    }
}

impl From<Deck> for TrackableDeck {
    fn from(value: Deck) -> Self {
        Self {
            cards: CursorVec::new().with_container(value).rotatable(true),
            selected: HashSet::new(),
        }
    }
}

impl Deref for TrackableDeck {
    type Target = Deck;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl DerefMut for TrackableDeck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cards
    }
}

// TODO: Use ListWidget to handle selection instead.

impl TuiComponent for TrackableDeck {
    fn draw(&mut self, frame: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let deck_areas =
            Layout::horizontal(vec![Constraint::Fill(1); self.cards.len()]).split(rect);
        let hover_position = self.cards.get_cursor();

        for (idx, (card, mut area)) in self
            .cards
            .iter_mut()
            .zip(deck_areas.iter().copied())
            .enumerate()
        {
            let mut card_widget_state = CardWidgetState::from(card);

            if hover_position == Some(idx) {
                card_widget_state.visual_state = CardVisualState::Hovered;
            }

            if self.selected.contains(&idx) {
                area = area.offset(Offset { x: 0, y: -5 });
            }

            frame.render_stateful_widget(CardWidget::new(), area, &mut card_widget_state);
        }

        Ok(())
    }

    fn handle_events(&mut self, event: Event) -> Result<()> {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "Intended: Unused events may skip implementation as required."
        )]
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Right => {
                    self.cards
                        .move_next()
                        .map_err(|err| eyre!(err))
                        .wrap_err("Cannot move to the next card.")?;
                }
                KeyCode::Left => {
                    self.cards
                        .move_prev()
                        .map_err(|err| eyre!(err))
                        .wrap_err("Cannot move to the previous card.")?;
                }
                KeyCode::Up => {
                    if self.selected.len() < MAXIMUM_SELECTED_CARDS {
                        if let Some(pos) = self.cards.get_cursor() {
                            self.select(pos);
                        }
                    }
                }
                KeyCode::Down => {
                    if let Some(pos) = self.cards.get_cursor() {
                        self.deselect(pos);
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }
}
