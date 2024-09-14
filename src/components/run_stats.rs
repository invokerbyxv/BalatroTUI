use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    text::Line,
    widgets::{StatefulWidget, Widget},
};

use super::text_box::TextBoxWidget;
use crate::core::run::Run;

/// Content height for [`RunStatsWidget`].
const RUN_STATS_CONTENT_HEIGHT: u16 = 15;

/// [`Widget`] to show stats for a [`Run`].
///
/// Widget construction uses builder pattern which can be started using the
/// [`Self::new()`] method.
///
/// ```
/// let area = Rect::new(0, 0, 100, 100);
/// let mut buffer = Buffer::empty(area);
/// let run = Run::default();
///
/// RunStatsWidget::new().render(area, buffer, run)
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct RunStatsWidget;

impl RunStatsWidget {
    /// Create new instance of [`RunStatsWidget`]
    #[must_use = "Created run stats widget instance must be used."]
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for RunStatsWidget {
    type State = Run;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Prepare areas
        let [inner_area] = Layout::vertical([Constraint::Length(RUN_STATS_CONTENT_HEIGHT)])
            .flex(Flex::Center)
            .areas(area.inner(Margin::new(1, 1)));
        let [run_info_button_area, run_stats_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(inner_area);
        let [round_functional_info_area, money_area, round_meta_info_area] =
            Layout::vertical([Constraint::Fill(1); 3])
                .flex(Flex::Center)
                .areas(run_stats_area);
        let [hands_count_area, discards_count_area] =
            Layout::horizontal([Constraint::Fill(1); 2]).areas(round_functional_info_area);
        let [ante_area, round_number_area] =
            Layout::horizontal([Constraint::Fill(1); 2]).areas(round_meta_info_area);

        // Render widgets
        // TODO: Load RunInfoButtonWidget here.
        TextBoxWidget::bordered([Line::from(state.round.hands_count.to_string()).centered()])
            .title("Hands")
            .render(hands_count_area, buf);
        TextBoxWidget::bordered([Line::from(state.round.discards_count.to_string()).centered()])
            .title("Discards")
            .render(discards_count_area, buf);
        TextBoxWidget::bordered([Line::from(format!("{}$", state.money)).centered()])
            .title("Money")
            .render(money_area, buf);
        TextBoxWidget::bordered([Line::from(state.round.properties.ante.to_string()).centered()])
            .title("Ante")
            .render(ante_area, buf);
        TextBoxWidget::bordered([
            Line::from(state.round.properties.round_number.to_string()).centered()
        ])
        .title("Round")
        .render(round_number_area, buf);
    }
}
