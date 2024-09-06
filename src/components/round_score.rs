use ratatui::{buffer::Buffer, layout::{Constraint, Flex, Layout, Rect}, style::Color, text::Line, widgets::{StatefulWidget, Widget}};

use crate::{core::round::Round, tui::get_line_with_chips};

use super::text_box::TextBoxWidget;

const ROUND_SCORE_CONTENT_HEIGHT: u16 = 5;

#[derive(Debug, Default, Clone, Copy)]
pub struct RoundScoreWidget { }

impl RoundScoreWidget {
    #[inline]
    pub const fn new() -> Self {
        RoundScoreWidget { }
    }
}

impl StatefulWidget for RoundScoreWidget {
    type State = Round;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Prepare widgets
        let round_score_content = [Line::from("Round Score").centered()];

        // Prepare areas
        let [inner_area] = Layout::vertical([Constraint::Length(ROUND_SCORE_CONTENT_HEIGHT)]).flex(Flex::Center).areas(area);
        let [round_score_text_area, round_score_value_area] = Layout::horizontal([
            Constraint::Fill(2),
            Constraint::Fill(5),
        ]).areas(inner_area);
        let [round_score_text_area] = Layout::vertical(vec![Constraint::Length(1)]).flex(Flex::SpaceAround).areas(round_score_text_area);

        // Render widgets
        TextBoxWidget::new(round_score_content).render(round_score_text_area, buf);
        TextBoxWidget::bordered([get_line_with_chips(state.score.to_string(), Color::Red).centered()]).render(round_score_value_area, buf);
    }
}
