use std::{error::Error, sync::{Arc, RwLock}};

use crossterm::event::KeyCode;
use ratatui::{layout::{Constraint, Layout, Rect}, Frame};

use crate::{event::Event, tui::TuiComponent};

use super::{
    blind::{Blind, BlindType},
    deck::{Deck, Drawable, Selectable, Sortable}, scorer::Scorer,
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
    pub score: usize,
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

    #[inline]
    pub fn play_hand(&mut self) -> Result<(), Box<dyn Error>> {
        self.properties.hands -= 1;

        let mut played_cards = self.hand.draw_selected()?;

        self.score += Scorer::score_cards(&played_cards)?;

        let mut new_cards = self.deck.write().unwrap().draw_random(played_cards.len())?;

        self.history.cards.append(&mut played_cards);
        self.hand.cards.append(&mut new_cards.cards);
        self.hand.sort_by_rank();

        Ok(())
    }

    #[inline]
    pub fn discard_hand(&mut self) -> Result<(), Box<dyn Error>> {
        self.properties.discards -= 1;

        let mut discarded_cards = self.hand.draw_selected()?;

        let mut new_cards = self.deck.write().unwrap().draw_random(discarded_cards.len())?;

        self.history.cards.append(&mut discarded_cards);
        self.hand.cards.append(&mut new_cards.cards);
        self.hand.sort_by_rank();

        Ok(())
    }
}

// TODO: Add a scorer animation area.
// TODO: Remove deep variable access, use accessor functions/split responsibilities.

impl TuiComponent for Round {
    #[inline]
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let [_play_area, deck_area] = Layout::vertical([Constraint::Fill(1), Constraint::Length(10)]).areas(rect);
        self.hand.draw(frame, deck_area);
    }

    #[inline]
    fn handle_events(&mut self, event: Event) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => self.play_hand().unwrap(),
                KeyCode::Char('x') => {
                    if self.properties.discards == 0 {
                        return;
                    }

                    self.discard_hand().unwrap()
                },
                _ => ()
            }
        }
        self.hand.handle_events(event);
    }
}