use ratatui::{buffer::Buffer, layout::{Constraint, Flex, Layout, Margin, Rect}, style::{Color, Stylize}, text::Line, widgets::{Block, BorderType, StatefulWidget, Widget}};

use crate::{core::round::Round, tui::{center_widget, render_centered_lines, render_centered_text}};

use super::{blind_badge::BlindBadgeWidget, chip::ChipWidget};

#[derive(Debug, Default, Clone, Copy)]
pub struct RoundInfoWidget { }

impl RoundInfoWidget {
    #[inline]
    pub fn new() -> Self {
        RoundInfoWidget { }
    }
}

impl StatefulWidget for RoundInfoWidget {
    type State = Round;

    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [blind_badge_outer_area, round_info_outer_area] = Layout::horizontal([
            Constraint::Fill(5),
            Constraint::Fill(7),
        ]).areas(area);

        let blind_badge_inner_area = center_widget(
            blind_badge_outer_area,
            Constraint::Length(15),
            Constraint::Length(9),
        );
        let blind_badge_inner_area = blind_badge_inner_area.inner(&Margin::new(1, 1));

        let round_info_outer_area = round_info_outer_area.inner(&Margin::new(1, 1));
        let round_info_outer_area = center_widget(
            round_info_outer_area.inner(&Margin::new(1, 1)),
            Constraint::Fill(1),
            Constraint::Length(11),
        );
        let round_info_inner_area = round_info_outer_area.inner(&Margin::new(1, 1));
        let [score_pretext_area, score_area, reward_area] = Layout::vertical([Constraint::Fill(1); 3]).areas(round_info_inner_area);
        let [chip_area, score_text_area] = Layout::horizontal([Constraint::Fill(1); 2]).flex(Flex::Center).areas(score_area);

        let reward_line: Line = vec![
            "Reward: ".into(),
            "$$$".yellow().bold(),
        ].into();

        BlindBadgeWidget::new().render(blind_badge_inner_area, buf, &mut state.properties.blind);
        Block::bordered().border_type(BorderType::Rounded).render(round_info_outer_area, buf);
        render_centered_text("Score at least", score_pretext_area, buf);
        // TODO: Add ChipWidget to chip_image_area
        ChipWidget::new(Color::Red).render(chip_area, buf);
        // TODO: Make score text cover 3 lines (BigText)
        render_centered_text(state.properties.blind.target.to_string(), score_text_area, buf);
        render_centered_lines(vec![reward_line], reward_area, buf);
    }
}
