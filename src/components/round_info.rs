use ratatui::{buffer::Buffer, layout::{Constraint, Flex, Layout, Margin, Rect}, style::{Color, Stylize}, text::Line, widgets::{StatefulWidget, Widget}};

use crate::{core::round::Round, tui::get_line_with_chips};

use super::{blind_badge::BlindBadgeWidget, text_box::TextBoxWidget};

const ROUND_INFO_CONTENT_HEIGHT: u16 = 9;
const KERNING_MULTIPLIER: u16 = 2;

#[derive(Debug, Default, Clone, Copy)]
pub struct RoundInfoWidget { }

impl RoundInfoWidget {
    #[inline]
    pub fn new() -> Self {
        RoundInfoWidget { }
    }
}

// TODO: Add pub(crate) qualifications
// TODO: Replace constraints Length with Max and Fill with Min.

impl StatefulWidget for RoundInfoWidget {
    type State = Round;

    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Prepare variables
        let round_info_content = [
            Line::from("Score at least").centered(),
            // TODO: Consider using BigText here
            get_line_with_chips(state.properties.blind.target.to_string(), Color::Red).centered(),
            // TODO: Use reward field from round here.
            Line::from(vec!["Reward: ".into(), "$$$$".yellow().bold()]).centered(),
        ];

        // Prepare areas
        let [inner_area] = Layout::vertical([
            Constraint::Length(ROUND_INFO_CONTENT_HEIGHT),
        ]).flex(Flex::Center).areas(area);
        let [blind_badge_area, round_info_area] = Layout::horizontal([
            Constraint::Length(ROUND_INFO_CONTENT_HEIGHT * KERNING_MULTIPLIER),
            Constraint::Fill(1),
        ]).areas(inner_area);

        // Render widgets
        BlindBadgeWidget::new().render(blind_badge_area.inner(Margin::new(2, 1)), buf, &mut state.properties.blind);
        // TODO: Move to TextBox that will also display border
        TextBoxWidget::bordered(round_info_content).flex(Flex::SpaceAround).render(round_info_area, buf);
    }
}
