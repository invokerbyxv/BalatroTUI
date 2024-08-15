use std::{error::Error, sync::{Arc, RwLock}};

use ratatui::{layout::{Constraint, Layout, Rect}, Frame};

use crate::{event::Event, tui::TuiComponent};

use super::{
    blind::{Blind, BlindType},
    deck::{Deck, Drawable, Sortable},
};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct RoundProperties {
    pub blind: Blind,
    pub discards: usize,
    pub hand_size: usize,
    pub hands: usize,
    pub round_number: usize,
}

impl Default for RoundProperties {
    fn default() -> Self {
        Self {
            blind: Blind::new(BlindType::SmallBlind, 1).unwrap(),
            hand_size: 10,
            hands: 3,
            discards: 3,
            round_number: 1,
        }
    }
}

#[derive(Debug, Default)]
pub struct Round {
    pub properties: RoundProperties,
    pub deck: Arc<RwLock<Deck>>,
    pub hand: Deck,
    pub history: Deck,
}

impl Round {
    #[inline]
    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut deck = self.deck.write().unwrap();
        self.hand = deck.draw_random(self.properties.hand_size)?;
        self.properties.round_number = 1;
        self.hand.sort_by_rank();
        Ok(())
    }
}

impl TuiComponent for Round {
    #[inline]
    fn draw(&self, frame: &mut Frame, rect: Rect) {
        let [_play_area, deck_area] = Layout::vertical([Constraint::Fill(1), Constraint::Length(10)]).areas(rect);
        self.hand.draw(frame, deck_area);
    }

    #[inline]
    fn handle_events(&mut self, event: Event) {
        match event {
            _ => ()
        }
        self.hand.handle_events(event);
    }
}