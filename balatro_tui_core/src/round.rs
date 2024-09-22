//! Round comprises of one single game with a selected blind.
//!
//! Once the [`Round::score`] crosses the target score of [`Round::blind`], the
//! round is considered to be won and the reward from [`Round::blind`] is added
//! to the enclosing [`super::run::RunProperties`]. If [`Round::hands_count`]
//! reaches zero and the [`Round::score`] does not cross the target score of
//! [`Round::blind`], the round is considered as lost, returning the user to
//! game over screen.

use std::{
    num::NonZeroUsize,
    sync::{Arc, RwLock},
};

use super::{
    blind::Blind,
    card::{Card, Sortable},
    deck::{Deck, DeckExt},
    scorer::Scorer,
};
use crate::error::{ArithmeticError, CoreError};

/// Abstracts properties that remain persistent across played hands within a
/// round.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RoundProperties {
    /// Current ante of the rounds. Game ends after beating ante `8`.
    pub ante: NonZeroUsize,
    /// Number of total cards that will be available in hand in a fresh turn.
    pub hand_size: usize,
    /// Current round number. Game ends after round beating `24`.
    pub round_number: NonZeroUsize,
}

/// [`Round`] struct carries the running state of a particular round.
///
/// Once the round is over, this struct is destroyed and a new one is created
/// when a blind is selected again.
#[derive(Debug)]
pub struct Round {
    /// Persistent properties for the round.
    pub properties: RoundProperties,
    // TODO: Rename to shoe.
    /// Shared deck of cards across rounds. Round will start by drawing random
    /// cards from this deck.
    pub deck: Arc<RwLock<Deck>>,
    /// An instance of a [`Blind`].
    pub blind: Blind,
    /// Number of hands that can be discarded and replaced with newly drawn
    /// cards in the round.
    pub discards_count: usize,
    /// Number of hands that can be played in the round.
    pub hands_count: usize,
    /// Score accumulated in a round.
    pub score: usize,
    // TODO: Remove pub access modifier wherever possible to constrict visibility
    /// An internal state for handling the hover and selection of cards in hand.
    pub hand: Arc<RwLock<Deck>>,
    /// A drainage for played cards; to be flushed into the main deck at the end
    /// of the round.
    pub history: Deck,
}

impl Round {
    /// Main entrypoint of the round. Once called, this method prepares the
    /// initial state of the round and initializes internal states.
    pub fn start(&mut self) -> Result<(), CoreError> {
        self.hand = Arc::from(RwLock::from(
            self.deck
                .try_write()?
                .draw_random(self.properties.hand_size)?,
        ));
        self.hand.try_write()?.sort_by_rank();

        Ok(())
    }

    /// Draws new cards at the end of a hand played or discarded and adds
    /// previous cards to history drain.
    fn deal_cards(&mut self, last_cards: &mut Vec<Card>) -> Result<(), CoreError> {
        let mut new_cards = self.deck.try_write()?.draw_random(last_cards.len())?;
        self.history.append(last_cards);
        self.hand.try_write()?.append(&mut new_cards);
        self.hand.try_write()?.sort_by_rank();

        Ok(())
    }

    /// Plays the selected cards and scores the hand.
    pub fn play_hand(&mut self, played_cards: &mut Vec<Card>) -> Result<(), CoreError> {
        if self.hands_count == 0 {
            return Err(CoreError::HandsExhaustedError);
        }

        self.hands_count = self
            .hands_count
            .checked_sub(1)
            .ok_or(ArithmeticError::Overflow("subtraction"))?;

        self.score = self
            .score
            .checked_add(Scorer::score_cards(played_cards)?)
            .ok_or(ArithmeticError::Overflow("addition"))?;

        self.deal_cards(played_cards)?;

        Ok(())
    }

    /// Discards the selected cards and draws equal number of cards as the ones
    /// discarded.
    pub fn discard_hand(&mut self, discarded_cards: &mut Vec<Card>) -> Result<(), CoreError> {
        if self.discards_count == 0 {
            return Err(CoreError::DiscardsExhaustedError);
        }

        self.discards_count = self
            .discards_count
            .checked_sub(1)
            .ok_or(ArithmeticError::Overflow("subtraction"))?;

        self.deal_cards(discarded_cards)?;

        Ok(())
    }
}

// TODO: Add a scorer animation area.
// TODO: Remove deep variable access, use accessor functions/split
// responsibilities.
