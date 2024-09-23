use ratatui::{
    style::{Color, Style, Styled},
    text::{Line, Span},
};

/// Returns line widget with chip icon prepended
pub(crate) fn get_line_with_chips<'widget, T: Into<Span<'widget>>>(
    content: T,
    color: Color,
) -> Line<'widget> {
    Line::from(vec![
        "\u{26c0}".set_style(Style::new().fg(color)),
        "  ".into(),
        content.into(),
    ])
}
