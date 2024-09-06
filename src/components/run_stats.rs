use ratatui::{buffer::Buffer, layout::{Constraint, Flex, Layout, Margin, Rect}, text::Line, widgets::{StatefulWidget, Widget}};

use crate::core::run::Run;

use super::text_box::TextBoxWidget;

const RUN_STATS_CONTENT_HEIGHT: u16 = 15;

#[derive(Debug, Default, Clone, Copy)]
pub struct RunStatsWidget { }

impl RunStatsWidget {
    #[inline]
    pub fn new() -> Self {
        RunStatsWidget { }
    }
}

// TODO: Use Layout with flex wherever possible.

impl StatefulWidget for RunStatsWidget {
    type State = Run;

    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Prepare areas
        let [inner_area] = Layout::vertical([Constraint::Length(RUN_STATS_CONTENT_HEIGHT)]).flex(Flex::Center).areas(area);
        let [run_info_button_area, run_stats_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(2),
        ]).areas(inner_area);
        let [round_functional_info_area, money_area, round_meta_info_area] = Layout::vertical([Constraint::Max(5); 3]).flex(Flex::Center).areas(run_stats_area.inner(Margin::new(1, 1)));
        let [hands_area, discards_area] = Layout::horizontal([Constraint::Fill(1); 2]).areas(round_functional_info_area);
        let [ante_area, round_number_area] = Layout::horizontal([Constraint::Fill(1); 2]).areas(round_meta_info_area);

        // Render widgets
        // TODO: Load RunInfoButtonWidget here.
        TextBoxWidget::bordered([Line::from(state.round.properties.hands.to_string()).centered()]).title("Hands").render(hands_area, buf);
        TextBoxWidget::bordered([Line::from(state.round.properties.discards.to_string()).centered()]).title("Discards").render(discards_area, buf);
        TextBoxWidget::bordered([Line::from(format!("{}$", state.money)).centered()]).title("Money").render(money_area, buf);
        TextBoxWidget::bordered([Line::from(state.properties.ante.to_string()).centered()]).title("Ante").render(ante_area, buf);
        TextBoxWidget::bordered([Line::from(state.round.properties.round_number.to_string()).centered()]).title("Round").render(round_number_area, buf);
    }
}
