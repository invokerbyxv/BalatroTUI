use std::error::Error;
use std::sync::{Arc, RwLock};

use rand::distributions::{Alphanumeric, DistString};
use ratatui::layout::Rect;
use ratatui::Frame;

use crate::event::Event;
use crate::tui::TuiComponent;

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
    pub round: Round,
}

impl Run {
    pub fn new(deck: Arc<RwLock<Deck>>, properties: RunProperties) -> Run {
        Run {
            deck: deck.clone(),
            properties: properties.clone(),
            round: Round {
                deck: deck.clone(),
                properties: RoundProperties {
                    round_number: 1,
                    blind: Blind::new(BlindType::SmallBlind, properties.ante).unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }

    #[inline]
    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.round.start()
    }
}

impl TuiComponent for Run {
    #[inline]
    fn draw(&self, frame: &mut Frame, rect: Rect) {
        self.round.draw(frame, rect);
    }

    #[inline]
    fn handle_events(&mut self, event: Event) {
        match event {
            _ => ()
        }
        self.round.handle_events(event);
    }
}