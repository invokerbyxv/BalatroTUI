use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};
use strum::{Display, EnumCount, EnumIter, EnumString, IntoStaticStr};

use super::text_box::TextBoxWidget;
use crate::core::card::Card;

/// Content width for [`CardWidget`].
pub const CARD_CONTENT_WIDTH: u16 = 12;
/// Content height for [`CardWidget`].
pub const CARD_CONTENT_HEIGHT: u16 = 9;

/// Visual state for a card.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    EnumCount,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    IntoStaticStr,
    PartialEq,
)]
pub enum CardVisualState {
    /// Represents the default state for the card.
    #[default]
    Normal,
    /// Represents when card is hovered.
    Hovered,
    /// Represents when card is selected.
    Selected,
}

/// Wrapper state struct for [`CardWidget`].
#[derive(Clone, Copy, Debug)]
pub struct CardWidgetState {
    /// A [`Card`] instance
    pub card: Card,
    /// The [`CardVisualState`] of the [`Card`].
    pub visual_state: CardVisualState,
}

impl CardWidgetState {
    /// Create new instance of [`CardWidgetState`].
    #[must_use = "Created card widget state instance must be used."]
    #[inline]
    pub const fn new(card: Card, visual_state: CardVisualState) -> Self {
        Self { card, visual_state }
    }
}

// TODO: Remove unneeded usages of clone()

// TODO: Explore blanket trait implementation for From over mutability.

impl From<Card> for CardWidgetState {
    #[must_use = "Converted card widget state instance must be used."]
    #[inline]
    fn from(value: Card) -> Self {
        Self::new(value, CardVisualState::Normal)
    }
}

impl From<&Card> for CardWidgetState {
    #[must_use = "Converted card widget state instance must be used."]
    #[inline]
    fn from(value: &Card) -> Self {
        Self::new(*value, CardVisualState::Normal)
    }
}

impl From<&mut Card> for CardWidgetState {
    #[must_use = "Converted card widget state instance must be used."]
    #[inline]
    fn from(value: &mut Card) -> Self {
        Self::new(*value, CardVisualState::Normal)
    }
}

/// [`Widget`] to display a [`Card`].
///
/// Widget construction uses builder pattern which can be started using the
/// [`Self::new()`] method.
///
/// ```
/// let area = Rect::new(0, 0, 100, 100);
/// let mut buffer = Buffer::empty(area);
/// let card = Card {
///     rank: Rank::Ace,
///     suit: Suit::Club,
/// };
///
/// CardWidget::new().render(area, buffer, card)
/// ```
///
/// A hovered card is represented with border as [`border::THICK`], otherwise
/// border is set to [`border::ROUNDED`].
#[derive(Clone, Copy, Debug, Default)]
pub struct CardWidget;

impl CardWidget {
    /// Create new instance of [`CardWidget`]
    #[must_use = "Created card widget instance must be used."]
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for CardWidget {
    type State = CardWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Prepare variables
        let border_set = if state.visual_state == CardVisualState::Hovered {
            border::THICK
        } else {
            border::ROUNDED
        };

        // Prepare areas
        let mut inner_area =
            Layout::vertical([Constraint::Length(CARD_CONTENT_HEIGHT)]).areas::<1>(area)[0];
        inner_area =
            Layout::horizontal([Constraint::Length(CARD_CONTENT_WIDTH)]).areas::<1>(inner_area)[0];
        let [top_area, middle_area, bottom_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(inner_area.inner(Margin::new(1, 1)));

        // Render containers
        Block::bordered()
            .border_set(border_set)
            .render(inner_area, buf);

        Paragraph::new(format!("{}\r\n{}", state.card.rank, state.card.suit))
            .left_aligned()
            .render(top_area, buf);
        // TODO: Mimic actual card suit layout
        TextBoxWidget::new([
            Line::from(format!("{}{}", state.card.rank, state.card.suit)).centered(),
        ])
        .render(middle_area, buf);
        Paragraph::new(format!("{}\r\n{}", state.card.suit, state.card.rank))
            .right_aligned()
            .render(bottom_area, buf);
    }
}
