use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Layout, Rect}, widgets::{Block, BorderType, StatefulWidget, Widget}};

use crate::{core::run::Run, tui::render_centered_text};

#[derive(Debug, Default, Clone, Copy)]
pub struct RunStatsWidget { }

impl RunStatsWidget {
    #[inline]
    pub fn new() -> Self {
        RunStatsWidget { }
    }
}

impl StatefulWidget for RunStatsWidget {
    type State = Run;

    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [round_functional_info_area, money_area, round_meta_info_area] = Layout::vertical([Constraint::Length(5); 3]).areas(area);
        let [hands_area, discards_area] = Layout::horizontal([Constraint::Ratio(1, 2); 2]).areas(round_functional_info_area);
        let [ante_area, round_number_area] = Layout::horizontal([Constraint::Ratio(1, 2); 2]).areas(round_meta_info_area);

        let stat_block = Block::bordered().border_type(BorderType::Rounded).title_alignment(Alignment::Center);

        stat_block.clone().title("Hands").render(hands_area, buf);
        stat_block.clone().title("Discards").render(discards_area, buf);
        stat_block.clone().title("Money").render(money_area, buf);
        stat_block.clone().title("Ante").render(ante_area, buf);
        stat_block.clone().title("Round").render(round_number_area, buf);

        render_centered_text(state.round.properties.hands.to_string(), hands_area, buf);
        render_centered_text(state.round.properties.discards.to_string(), discards_area, buf);
        render_centered_text(format!("{}$", state.money), money_area, buf);
        render_centered_text(state.properties.ante.to_string(), ante_area, buf);
        render_centered_text(state.round.properties.round_number.to_string(), round_number_area, buf);
    }
}
