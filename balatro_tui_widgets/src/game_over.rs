use std::num::NonZeroUsize;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::Stylize,
    symbols::border,
    widgets::{Block, Clear, StatefulWidget, Widget},
};

use crate::TextBoxWidget;

const GAME_OVER_CONTENT_WIDTH: u16 = 40;

/// [`Widget`] to show in game over scenario.
///
/// Widget construction uses builder pattern which can be started using the
/// [`Self::new()`] method.
///
/// ```
/// # use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget, style::Color, text::Line};
/// # use balatro_tui_widgets::GameOverWidget;
/// let area = Rect::new(0, 0, 100, 100);
/// let mut buffer = Buffer::empty(area);
///
/// GameOverWidget::new().render(area, &mut buffer, (2, 5));
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct GameOverWidget;

impl GameOverWidget {
    /// Create new instance of [`GameOverWidget`].
    #[must_use = "Created game over widget instance must be used."]
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for GameOverWidget {
    type State = (NonZeroUsize, NonZeroUsize);

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Prepare areas
        let [mut inner_area] = Layout::vertical([Constraint::Fill(1)])
            .flex(Flex::SpaceAround)
            .areas(area.inner(Margin::new(4, 4)));
        inner_area = Layout::horizontal([Constraint::Length(GAME_OVER_CONTENT_WIDTH)])
            .flex(Flex::SpaceAround)
            .areas::<1>(inner_area)[0];
        let [message_area, details_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(7)]).areas(inner_area);

        // Render widgets
        Clear.render(area, buf);
        Block::bordered()
            .border_set(border::DOUBLE)
            .render(area, buf);
        TextBoxWidget::new([
            "Game Over".bold().into_centered_line(),
            "You lost the game!".italic().into_left_aligned_line(),
        ])
        .render(message_area, buf);
        TextBoxWidget::bordered([
            vec![
                "Last round reached".bold(),
                "\t".into(),
                state.0.to_string().yellow(),
            ]
            .into(),
            vec![
                "Last ante reached".bold(),
                " \t".into(),
                state.1.to_string().yellow(),
            ]
            .into(),
        ])
        .padding(4)
        .render(details_area, buf);
    }
}
