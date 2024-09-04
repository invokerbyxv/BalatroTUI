use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Layout, Margin, Rect}, symbols::border, widgets::{Block, Paragraph, StatefulWidget, Widget}};

use crate::{core::card::Card, tui::{center_widget, render_centered_text}};

const CARD_WIDTH: usize = 12;
const CARD_HEIGHT: usize = 9;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CardVisualState {
    #[default]
    Normal,
    Hovered,
    Selected,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CardWidget {
    visual_state: CardVisualState
}

impl CardWidget {
    pub fn new() -> Self {
        CardWidget { visual_state: CardVisualState::Normal }
    }

    pub fn hover(&mut self) {
        self.visual_state = CardVisualState::Hovered;
    }

    // TODO: Add event handling with selection of cards
}

impl StatefulWidget for CardWidget {
    type State = Card;

    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let card_outer_layout = center_widget(area, Constraint::Length(CARD_WIDTH as u16), Constraint::Length(CARD_HEIGHT as u16));
        let inner_area = card_outer_layout.inner(&Margin::new(1, 1));
        let [top_area, middle_area, bottom_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2)
        ]).areas(inner_area);
        let border_set = match self.visual_state {
            CardVisualState::Hovered => border::THICK,
            _ => border::ROUNDED,
        };

        Block::bordered().border_set(border_set).render(card_outer_layout, buf);
        Paragraph::new(format!("{}\r\n{}", state.rank, state.suit)).alignment(Alignment::Left).render(top_area, buf);
        // TODO: Mimic actual card suit layout
        render_centered_text(format!("{}{}", state.rank, state.suit), middle_area, buf);
        Paragraph::new(format!("{}\r\n{}", state.suit, state.rank)).alignment(Alignment::Right).render(bottom_area, buf);
    }
}
