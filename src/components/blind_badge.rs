use ratatui::{buffer::Buffer, layout::Rect, style::Stylize, symbols::Marker, text::Line, widgets::{canvas::{Canvas, Circle}, StatefulWidget, Widget}};

use crate::core::blind::Blind;

#[derive(Debug, Default, Clone, Copy)]
pub struct BlindBadgeWidget { }

impl BlindBadgeWidget {
    pub fn new() -> Self {
        BlindBadgeWidget { }
    }
}

impl StatefulWidget for BlindBadgeWidget {
    type State = Blind;

    // TODO: Use image instead of canvas
    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let bound = area.height as f64;

        let canvas = Canvas::default()
            .marker(Marker::Braille)
            .paint(|ctx| {
                ctx.draw(&Circle {
                    x: 0.0,
                    y: 0.0,
                    radius: bound,
                    color: state.get_color(),
                });

                format!("{}", state).split_whitespace()
                    .map(String::from)
                    .map(|text| {
                        Line::from(text).centered().yellow()
                    })
                    .rev()
                    .enumerate()
                    .for_each(|(idx, line)| {
                        ctx.print(-1.0, idx as f64, line);
                    });
            })
            .x_bounds([-bound, bound])
            .y_bounds([-bound, bound]);
        canvas.render(area, buf);
    }
}
