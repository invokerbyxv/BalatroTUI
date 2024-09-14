use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{StatefulWidget, Widget},
};

use super::{blind_badge::BlindBadgeWidget, text_box::TextBoxWidget};
use crate::{core::round::Round, tui::get_line_with_chips};

/// Content height for [`RoundInfoWidget`].
pub const ROUND_INFO_CONTENT_HEIGHT: u16 = 9;
/// Kerning multiplier for horizontal and vertical print-space equality.
const KERNING_MULTIPLIER: u16 = 2;

/// [`Widget`] to show information about the running [`Round`].
///
/// Following details are shown:
/// - Target score
/// - Reward for defeating blind
///
/// Widget construction uses builder pattern which can be started using the
/// [`Self::new()`] method.
///
/// ```
/// let area = Rect::new(0, 0, 100, 100);
/// let mut buffer = Buffer::empty(area);
/// let round = Round::default();
///
/// RoundInfoWidget::new().render(area, buffer, round)
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct RoundInfoWidget;

impl RoundInfoWidget {
    /// Create new instance of [`RoundInfoWidget`]
    #[must_use = "Created round info widget instance must be used."]
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }
}

// TODO: Add pub(crate) qualifications

impl StatefulWidget for RoundInfoWidget {
    type State = Round;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Prepare variables
        let round_info_content = [
            Line::from("Score at least").centered(),
            // TODO: Consider using BigText here
            get_line_with_chips(
                state
                    .blind
                    .get_target_score(state.properties.ante)
                    .unwrap()
                    .to_string(),
                Color::Red,
            )
            .centered(),
            // TODO: Use reward field from round here.
            Line::from(vec![
                "Reward: ".into(),
                "$".repeat(state.blind.get_reward().unwrap())
                    .yellow()
                    .bold(),
            ])
            .centered(),
        ];

        // Prepare areas
        let [inner_area] = Layout::vertical([Constraint::Length(ROUND_INFO_CONTENT_HEIGHT)])
            .flex(Flex::Center)
            .areas(area);
        let [blind_badge_area, round_info_area] = Layout::horizontal([
            Constraint::Length(ROUND_INFO_CONTENT_HEIGHT * KERNING_MULTIPLIER),
            Constraint::Fill(1),
        ])
        .areas(inner_area);

        // Render widgets
        BlindBadgeWidget::new().render(
            blind_badge_area.inner(Margin::new(2, 1)),
            buf,
            &mut state.blind,
        );
        TextBoxWidget::bordered(round_info_content)
            .flex(Flex::SpaceAround)
            .render(round_info_area, buf);
    }
}
