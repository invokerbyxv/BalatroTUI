//! Round comprises of one single game with a selected blind.
//!
//! Once the [`Round::score`] crosses the target score of [`Round::blind`], the
//! round is considered to be won and the reward from [`Round::blind`] is added
//! to the enclosing [`Run::properties`]. If [`Round::hands_count`] reaches zero
//! and the [`Round::score`] does not cross the target score of
//! [`Round::blind`], the round is considered as lost, returning the user to
//! game over screen.

use std::sync::{Arc, RwLock};

use color_eyre::{
    eyre::{bail, OptionExt},
    Result,
};
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use super::{
    blind::Blind,
    card::{Card, Sortable},
    deck::{Deck, DeckExt, TrackableDeck},
    scorer::Scorer,
};
use crate::{event::Event, tui::TuiComponent};

/// Abstracts properties that remain persistent across played hands within a
/// round.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RoundProperties {
    /// Current ante of the rounds. Game ends after beating ante `8`.
    pub ante: usize,
    /// Number of total cards that will be available in hand in a fresh turn.
    pub hand_size: usize,
    /// Current round number. Game ends after round beating `24`.
    pub round_number: usize,
}

impl Default for RoundProperties {
    #[inline]
    fn default() -> Self {
        Self {
            hand_size: 10,
            ante: 1,
            round_number: 1,
        }
    }
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
    pub hand: TrackableDeck,
    /// A drainage for played cards; to be flushed into the main deck at the end
    /// of the round.
    pub history: Deck,
}

impl Round {
    /// Main entrypoint of the round. Once called, this method prepares the
    /// initial state of the round and initializes internal states.
    pub fn start(&mut self) -> Result<()> {
        let deck = self
            .deck
            .write()
            .or_else(|err| bail!("Could not attain write lock for deck to start round: {err}."))?
            .draw_random(self.properties.hand_size)?;
        self.hand.cards.set_container(deck);
        self.hand.cards.sort_by_rank();

        Ok(())
    }

    /// Draws new cards at the end of a hand played or discarded and adds
    /// previous cards to history drain.
    fn deal_cards(&mut self, last_cards: &mut Vec<Card>) -> Result<()> {
        let mut new_cards = self
            .deck
            .write()
            .or_else(|err| bail!("Could not attain write lock for deck to deal cards: {err}."))?
            .draw_random(last_cards.len())?;
        self.history.append(last_cards);
        self.hand.cards.append(&mut new_cards);
        self.hand.cards.update_cursor();
        self.hand.cards.sort_by_rank();

        Ok(())
    }

    /// Plays the selected cards and scores the hand.
    fn play_hand(&mut self) -> Result<()> {
        if self.hands_count == 0 {
            bail!("Attempted to play hand with all hands exhausted.");
        }

        self.hands_count = self
            .hands_count
            .checked_sub(1)
            .ok_or_eyre("Subtraction operation overflowed")?;

        let mut played_cards = self.hand.draw_selected()?;

        self.score = self
            .score
            .checked_add(Scorer::score_cards(&played_cards)?)
            .ok_or_eyre("Add operation overflowed")?;

        self.deal_cards(&mut played_cards)?;

        Ok(())
    }

    /// Discards the selected cards and draws equal number of cards as the ones
    /// discarded.
    fn discard_hand(&mut self) -> Result<()> {
        if self.discards_count == 0 {
            bail!("Attempted to discard hand with all discards exhausted.");
        }

        self.discards_count = self
            .discards_count
            .checked_sub(1)
            .ok_or_eyre("Subtraction operation overflowed")?;

        let mut discarded_cards = self.hand.draw_selected()?;

        self.deal_cards(&mut discarded_cards)?;

        Ok(())
    }
}

// TODO: Add a scorer animation area.
// TODO: Remove deep variable access, use accessor functions/split
// responsibilities.

// TODO: Migrate all TuiComponent impl to Widgets
impl TuiComponent for Round {
    fn draw(&mut self, frame: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let [play_area, deck_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(10)]).areas(rect);
        self.hand.draw(frame, deck_area)?;

        Ok(())
    }

    fn handle_events(&mut self, event: Event) -> Result<()> {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "Intended: Unused events may skip implementation as required."
        )]
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => {
                    if self.hands_count != 0 {
                        self.play_hand()?;
                    }
                }
                KeyCode::Char('x') => {
                    if self.discards_count != 0 {
                        self.discard_hand()?;
                    }
                }
                _ => (),
            }
        }
        self.hand.handle_events(event)?;

        Ok(())
    }
}
