use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::Marker,
    text::Line,
    widgets::{
        canvas::{Canvas, Circle},
        StatefulWidget, Widget,
    },
};

use crate::core::blind::Blind;

/// [`Widget`] for depicting [`Blind`] with text inside.
///
/// Widget construction uses builder pattern which can be started using the
/// [`Self::new()`] method.
///
/// ```
/// let area = Rect::new(0, 0, 100, 100);
/// let mut buffer = Buffer::empty(area);
/// let blind = Blind::Small;
///
/// BlindBadgeWidget::new().render(area, buffer, blind)
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct BlindBadgeWidget;

impl BlindBadgeWidget {
    /// Create new instance of [`BlindBadgeWidget`].
    #[must_use = "Created blind badge widget instance must be used."]
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for BlindBadgeWidget {
    type State = Blind;

    // TODO: Use image instead of canvas
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let bound = f64::from(area.height);

        let canvas = Canvas::default()
            .marker(Marker::Braille)
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 0.0,
                    y: 0.0,
                    radius: bound,
                    color: state.get_color().unwrap(),
                });

                let text = match *state {
                    Blind::Small | Blind::Big => state.to_string(),
                    Blind::Boss(boss) => boss.to_string(),
                };

                text.split_whitespace()
                    .map(String::from)
                    .map(|text_chunk| Line::from(text_chunk).centered().yellow())
                    .rev()
                    .enumerate()
                    .for_each(|(idx, line)| {
                        #[expect(
                            clippy::as_conversions,
                            reason = "Intended: Expanding to a larger type for API conformity."
                        )]
                        ctx.print(-1.0, idx as f64, line);
                    });
            })
            .x_bounds([-bound, bound])
            .y_bounds([-bound, bound]);
        canvas.render(area, buf);
    }
}
