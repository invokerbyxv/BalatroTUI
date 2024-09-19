//! Run is a complete play-through of the game until game over.
//!
//! Across a run, there are multiple rounds played. If any round is failed, the
//! run is over.

use std::sync::{Arc, RwLock};

use color_eyre::Result;
use rand::distributions::{Alphanumeric, DistString};

use super::{deck::Deck, round::Round};

/// Persistent details about the run.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RunProperties {
    /// The number of cards to be fetched in hand during the round.
    pub hand_size: usize,
    /// Maximum discards available per round.
    pub max_discards: usize,
    /// Maximum hands that can be made per round.
    pub max_hands: usize,
    /// Random seed for the run.
    pub seed: String,
    /// Initial amount of money that the run starts with.
    pub starting_money: usize,
}

impl Default for RunProperties {
    #[inline]
    fn default() -> Self {
        Self {
            hand_size: 10,
            max_discards: 3,
            max_hands: 3,
            seed: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
            starting_money: 10,
        }
    }
}

/// [`Run`] struct maintains the working state of a run, along with the rounds
/// that are selected.
///
/// A single run is maintained from the point a deck is selected to the point of
/// game over.
#[derive(Debug)]
pub struct Run {
    /// Persistent properties for the run.
    pub properties: RunProperties,
    /// Current money held by the user.
    pub money: usize,
    /// Shared deck of cards across rounds. [`Run`] simply passes this on to the
    /// [`Round`] instance.
    pub deck: Arc<RwLock<Deck>>,
    // TODO: Make round container optional and generic to be replaced between RoundSelection,
    // Round, Shop and GameOver
    /// An instance of a [`Round`].
    pub round: Round,
    /// Used to keep track of the last played [`Round`] number.
    pub upcoming_round_number: usize,
}

impl Run {
    /// Main entrypoint of the run. It initializes the internal state and spawns
    /// a round.
    #[inline]
    pub fn start(&mut self) -> Result<()> {
        self.round.start()
    }
}

// TODO: Split/Flex all widgets in meta_area evenly.
