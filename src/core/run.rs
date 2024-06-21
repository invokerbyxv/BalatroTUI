use std::error::Error;
use std::sync::{Arc, RwLock};

use rand::distributions::{Alphanumeric, DistString};

use super::blind::{Blind, BlindType};
use super::deck::Deck;
use super::round::{Round, RoundProperties};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct RunProperties {
    pub ante: usize,
    pub hand_size: usize,
    pub max_discards: usize,
    pub max_hands: usize,
    pub seed: String,
}

impl Default for RunProperties {
    fn default() -> Self {
        Self {
            ante: 1,
            hand_size: 10,
            max_discards: 3,
            max_hands: 3,
            seed: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
        }
    }
}

#[derive(Debug, Default)]
pub struct Run {
    pub properties: RunProperties,
    pub deck: Arc<RwLock<Deck>>,
    pub round: Option<Round>,
}

impl Run {
    #[inline]
    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut round = Round {
            deck: self.deck.clone(),
            properties: RoundProperties {
                round_number: 1,
                blind: Blind::new(BlindType::SmallBlind, self.properties.ante)?,
                ..Default::default()
            },
            ..Default::default()
        };
        round.start()?;
        self.round = Some(round);
        Ok(())
    }
}
