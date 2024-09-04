use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Margin, Rect}, widgets::{Block, BorderType, StatefulWidget, Widget}};

use crate::{core::{card::Card, scorer::{Scorer, ScoringHand}}, tui::render_centered_text};

#[derive(Debug, Default, Clone, Copy)]
pub struct ScorerPreviewWidget { }

impl ScorerPreviewWidget {
    #[inline]
    pub fn new() -> Self {
        ScorerPreviewWidget { }
    }
}

impl StatefulWidget for ScorerPreviewWidget {
    type State = Vec<Card>;

    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [area] = Layout::vertical([Constraint::Length(12)]).areas(area);
        let [scoring_hand_area, scoring_area] = Layout::vertical([Constraint::Fill(1); 2]).areas(area.inner(&Margin::new(1, 1)));
        let [chips_area, multiply_sign_area, multiplier_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ]).areas(scoring_area);

        let scoring_hand = Scorer::get_scoring_hand(state.clone()).unwrap();
        let (chips, multiplier) = Scorer::get_chips_and_multiplier(scoring_hand).unwrap();


        // TODO: Use big text to render chips, multiplier, scoring hand and multiply icon.
        Block::bordered().border_type(BorderType::Rounded).render(area, buf);

        if scoring_hand != ScoringHand::None {
            render_centered_text(format!("{} [lvl. {}]", scoring_hand, 1), scoring_hand_area, buf);
        }

        Block::bordered().border_type(BorderType::Rounded).render(chips_area, buf);
        render_centered_text(format!("{}", chips), chips_area, buf);

        render_centered_text("×", multiply_sign_area, buf);

        Block::bordered().border_type(BorderType::Rounded).render(multiplier_area, buf);
        render_centered_text(format!("{}", multiplier), multiplier_area, buf);
    }
}
