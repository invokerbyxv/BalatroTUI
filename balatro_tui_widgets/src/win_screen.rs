use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::Stylize,
    symbols::border,
    widgets::{Block, Clear, StatefulWidget, Widget},
};

use crate::TextBoxWidget;

const WIN_SCREEN_CONTENT_WIDTH: u16 = 40;

/// [`Widget`] to show in game win scenario.
///
/// Widget construction uses builder pattern which can be started using the
/// [`Self::new()`] method.
///
/// ```
/// # use ratatui::{buffer::Buffer, layout::Rect, prelude::Widget, style::Color, text::Line};
/// # use balatro_tui_widgets::WinScreenWidget;
/// let area = Rect::new(0, 0, 100, 100);
/// let mut buffer = Buffer::empty(area);
///
/// WinScreenWidget::new().render(area, &mut buffer, (2, 5));
/// ```
#[derive(Copy, Clone, Debug, Default)]
pub struct WinScreenWidget;

impl WinScreenWidget {
    /// Create new instance of [`WinScreenWidget`].
    #[must_use = "Created win screen widget instance must be used."]
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for WinScreenWidget {
    type State = usize;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Prepare areas
        let [mut inner_area] = Layout::vertical([Constraint::Fill(1)])
            .flex(Flex::SpaceAround)
            .areas(area.inner(Margin::new(4, 4)));
        inner_area = Layout::horizontal([Constraint::Length(WIN_SCREEN_CONTENT_WIDTH)])
            .flex(Flex::SpaceAround)
            .areas::<1>(inner_area)[0];
        let [message_area, details_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(5)]).areas(inner_area);

        // Render widgets
        Clear.render(area, buf);
        Block::bordered()
            .border_set(border::DOUBLE)
            .render(area, buf);
        TextBoxWidget::new([
            "Congratulations".bold().into_centered_line(),
            "You won the game!".italic().into_left_aligned_line(),
        ])
        .render(message_area, buf);
        TextBoxWidget::bordered([vec![
            "Money collected".bold(),
            "\t".into(),
            state.to_string().yellow(),
        ]
        .into()])
        .padding(4)
        .render(details_area, buf);
    }
}
