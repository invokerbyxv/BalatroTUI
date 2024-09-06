use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Margin, Rect}, symbols::border, text::Line, widgets::{Block, Paragraph, StatefulWidget, Widget}};

use crate::core::card::Card;

use super::text_box::TextBoxWidget;

const CARD_WIDTH: u16 = 12;
const CARD_HEIGHT: u16 = 9;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CardVisualState {
    #[default]
    Normal,
    Hovered,
    Selected,
}

#[derive(Debug, Clone, Copy)]
pub struct CardWidgetState {
    pub card: Card,
    pub visual_state: CardVisualState
}

impl CardWidgetState {
    pub fn new(card: Card, visual_state: CardVisualState) -> Self {
        CardWidgetState { card, visual_state }
    }
}

impl From<Card> for CardWidgetState {
    fn from(value: Card) -> Self {
        CardWidgetState::new(value, CardVisualState::Normal)
    }
}

impl From<&mut Card> for CardWidgetState {
    fn from(value: &mut Card) -> Self {
        CardWidgetState::new(value.clone(), CardVisualState::Normal)
    }
}


#[derive(Debug, Default, Clone, Copy)]
pub struct CardWidget { }

impl CardWidget {
    pub fn new() -> Self {
        CardWidget { }
    }
}

impl StatefulWidget for CardWidget {
    type State = CardWidgetState;

    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Prepare variables
        let border_set = match state.visual_state {
            CardVisualState::Hovered => border::THICK,
            _ => border::ROUNDED,
        };

        // Prepare areas
        let [area] = Layout::vertical([Constraint::Length(CARD_HEIGHT)]).areas(area);
        let [area] = Layout::horizontal([Constraint::Length(CARD_WIDTH)]).areas(area);
        let [top_area, middle_area, bottom_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2)
        ]).areas(area.inner(Margin::new(1, 1)));

        // Render containers
        Block::bordered().border_set(border_set).render(area, buf);

        Paragraph::new(format!("{}\r\n{}", state.card.rank, state.card.suit)).left_aligned().render(top_area, buf);
        // TODO: Mimic actual card suit layout
        TextBoxWidget::new([Line::from(format!("{}{}", state.card.rank, state.card.suit)).centered()]).render(middle_area, buf);
        Paragraph::new(format!("{}\r\n{}", state.card.suit, state.card.rank)).right_aligned().render(bottom_area, buf);
    }
}
