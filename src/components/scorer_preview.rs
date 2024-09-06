use ratatui::{buffer::Buffer, layout::{Constraint, Flex, Layout, Rect}, text::Line, widgets::{StatefulWidget, Widget}};

use crate::core::{card::Card, scorer::{Scorer, ScoringHand}};

use super::text_box::TextBoxWidget;

const SCORER_PREVIEW_CONTENT_HEIGHT: u16 = 8;

// TODO: Use nested area constraints for min-maxing.

#[derive(Debug, Default, Clone, Copy)]
pub struct ScorerPreviewWidget { }

impl ScorerPreviewWidget {
    #[inline]
    pub fn new() -> Self {
        ScorerPreviewWidget { }
    }
}

// TODO: Add const modifier to struct creation methods

impl StatefulWidget for ScorerPreviewWidget {
    type State = Vec<Card>;

    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Prepare variables
        // TODO: Pass these from outside or implement caching to avoid needless calls.
        let (scoring_hand, _) = Scorer::get_scoring_hand(&state).unwrap();
        let (chips, multiplier) = Scorer::get_chips_and_multiplier(scoring_hand).unwrap();

        // Prepare areas
        let [inner_area] = Layout::vertical([Constraint::Length(SCORER_PREVIEW_CONTENT_HEIGHT)]).flex(Flex::Center).areas(area);
        let [scoring_hand_area, scoring_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(5),
        ]).flex(Flex::Center).areas(inner_area);
        let [chips_area, multiply_sign_area, multiplier_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1)
        ]).areas(scoring_area);

        // Render widgets
        if scoring_hand != ScoringHand::None {
            // TODO: Add leveling system for scoring hand types
            TextBoxWidget::new([Line::from(format!("{} [lvl. {}]", scoring_hand, 1)).centered()]).render(scoring_hand_area, buf);
        }
        // TODO: Use big text to render chips, multiplier, scoring hand and multiply icon.
        TextBoxWidget::bordered([Line::from(chips.to_string()).centered()]).render(chips_area, buf);
        TextBoxWidget::new([Line::from("×".to_string()).centered()]).render(multiply_sign_area, buf);
        TextBoxWidget::bordered([Line::from(multiplier.to_string()).centered()]).render(multiplier_area, buf);

    }
}
