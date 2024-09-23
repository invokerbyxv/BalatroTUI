use std::sync::{Arc, RwLock};

use balatro_tui_core::card::Card;
use bit_set::BitSet;
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Offset, Rect},
    symbols::border,
    widgets::StatefulWidget,
};

use super::CardWidget;
use crate::error::{ArithmeticError, WidgetError};

/// Provide bidirectional circular cursor for iterators with ability to select
/// items.
///
/// By default the list is unfocussed. To move the cursor to the first element,
/// call [`SelectableList::move_next()`].
pub trait SelectableList {
    /// Move the cursor to next item. If next item doesn't exist, cycle back to
    /// the first item.
    fn move_next(&mut self) -> Result<(), WidgetError>;
    /// Move the cursor to previous item. If previous item doesn't exist, cycle
    /// to the last item.
    fn move_prev(&mut self) -> Result<(), WidgetError>;
    /// Add item at current cursor position to the selection list. No-op if the
    /// index is already selected.
    fn select(&mut self) -> Result<bool, WidgetError>;
    /// Remove item at current cursor position from the selection list. No-op if
    /// the index is already deselected.
    fn deselect(&mut self) -> Result<bool, WidgetError>;
    /// Unfocus (blur) the list. This method removes the cursor from the list.
    fn blur(&mut self);
}

// TODO: Move unchanging items out of state for widgets.
// TODO: Create macros for creating widgets.

/// Render state for [`CardListWidget`].
///
/// Holds a atomic mutable reference to a [`Vec<Card>`]. Tracks the current
/// cursor position and selected [`Card`] set.
///
/// [`CardListWidget`] can be created out of a [`Vec<Card>`] reference using the
/// [`Self::from()`] implementation.
///
/// ```
/// # use std::sync::{Arc, RwLock};
/// # use balatro_tui_core::card::{Card, Rank, Suit};
/// # use balatro_tui_widgets::CardListWidgetState;
/// let cards = vec![
///     Card {
///         rank: Rank::Ace,
///         suit: Suit::Diamond,
///     },
///     Card {
///         rank: Rank::Ten,
///         suit: Suit::Heart,
///     },
/// ];
///
/// let list_state = CardListWidgetState::from(Arc::from(RwLock::from(cards)));
/// ```
#[derive(Clone, Debug)]
pub struct CardListWidgetState {
    /// Holds a atomic mutable reference to a [`Vec<Card>`].
    pub cards: Arc<RwLock<Vec<Card>>>,
    /// Cursor position over the [`Self::cards`].
    pub pos: Option<usize>,
    // TODO: Use bit-mask for selected value over usize
    /// A cache of selected card indices.
    pub selected: BitSet,
    /// Optional limit defines the maximum cards that can be selected.
    pub selection_limit: Option<usize>,
}

impl CardListWidgetState {
    /// Update the [`Self::selection_limit`] and return the
    /// [`CardListWidgetState`] instance.
    #[must_use = "Card list widget state builder returned instance must be used."]
    #[inline]
    pub fn selection_limit(mut self, selection_limit: Option<usize>) -> Result<Self, WidgetError> {
        if let Some(limit) = selection_limit {
            if limit < self.selected.len() {
                return Err(WidgetError::SelectionLimitOverflow {
                    attempted_selection_limit: limit,
                    max_allowed: self.selected.len(),
                });
            }
        }

        self.selection_limit = selection_limit;
        Ok(self)
    }

    /// Updates the [`Self::cards`] and reset selected and focussed positions.
    #[inline]
    pub fn set_cards(&mut self, cards: Arc<RwLock<Vec<Card>>>) {
        self.cards = cards;
        self.pos = None;
        self.selected.clear();
    }
}

impl From<Arc<RwLock<Vec<Card>>>> for CardListWidgetState {
    fn from(value: Arc<RwLock<Vec<Card>>>) -> Self {
        Self {
            cards: value,
            pos: None,
            selected: BitSet::new(),
            selection_limit: None,
        }
    }
}

impl SelectableList for CardListWidgetState {
    fn move_next(&mut self) -> Result<(), WidgetError> {
        if let Some(pos) = self.pos {
            let last_index = self
                .cards
                .try_read()?
                .len()
                .checked_sub(1)
                .ok_or(ArithmeticError::Overflow("subtraction"))?;
            self.pos = Some(
                if pos == last_index {
                    0
                } else {
                    pos.checked_add(1)
                        .ok_or(ArithmeticError::Overflow("addition"))?
                },
            );
        } else {
            self.pos = Some(0);
        }

        Ok(())
    }

    fn move_prev(&mut self) -> Result<(), WidgetError> {
        if let Some(pos) = self.pos {
            self.pos = Some(
                (if pos == 0 {
                    self.cards.try_read()?.len()
                } else {
                    pos
                })
                .checked_sub(1)
                .ok_or(ArithmeticError::Overflow("subtraction"))?,
            );
        } else {
            self.pos = Some(0);
        }

        Ok(())
    }

    #[inline]
    fn select(&mut self) -> Result<bool, WidgetError> {
        if self
            .selection_limit
            .is_some_and(|limit| limit <= self.selected.len())
        {
            return Ok(false);
        }

        if let Some(pos) = self.pos {
            return Ok(self.selected.insert(pos));
        }

        Ok(false)
    }

    #[inline]
    fn deselect(&mut self) -> Result<bool, WidgetError> {
        if let Some(pos) = self.pos {
            return Ok(self.selected.remove(pos));
        }

        Ok(false)
    }

    #[inline]
    fn blur(&mut self) {
        self.pos = None;
    }
}

/// [`StatefulWidget`] to display a list of [`Card`].
///
/// Widget construction uses builder pattern which can be started using the
/// [`Self::new()`] method.
///
/// ```
/// # use std::sync::{Arc, RwLock};
/// # use ratatui::{buffer::Buffer, layout::Rect, prelude::*};
/// # use balatro_tui_core::card::{Card, Rank, Suit};
/// # use balatro_tui_widgets::{CardListWidget, CardListWidgetState};
/// let area = Rect::new(0, 0, 100, 100);
/// let mut buffer = Buffer::empty(area);
/// let mut card_list = CardListWidgetState::from(Arc::from(RwLock::from(vec![
///     Card {
///         rank: Rank::Ace,
///         suit: Suit::Club,
///     },
///     Card {
///         rank: Rank::Two,
///         suit: Suit::Heart,
///     },
///     Card {
///         rank: Rank::Ten,
///         suit: Suit::Diamond,
///     },
/// ])));
///
/// CardListWidget::new().render(area, &mut buffer, &mut card_list);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct CardListWidget;

impl CardListWidget {
    /// Create new instance of [`CardListWidget`].
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {}
    }
}

// TODO: Add pub(crate) qualifications
// TODO: Remove unused pub qualifications
// TODO: Optimize card widget construction

impl StatefulWidget for CardListWidget {
    type State = CardListWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Cards
        #[expect(
            clippy::unwrap_used,
            reason = "Intended: Read lock acquisition failure at this point should panic."
        )]
        let cards = state.cards.try_read().unwrap();

        // Prepare areas
        let deck_areas = Layout::horizontal(vec![Constraint::Fill(1); cards.len()]).split(area);

        // Render widgets
        cards
            .clone()
            .into_iter()
            .zip_eq(deck_areas.iter().copied())
            .enumerate()
            .for_each(|(idx, (mut card, mut card_area))| {
                if state.selected.contains(idx) {
                    card_area = card_area.offset(Offset { x: 0, y: -5 });
                }

                CardWidget::bordered(
                    if state.pos == Some(idx) {
                        border::THICK
                    } else {
                        border::ROUNDED
                    },
                )
                .render(card_area, buf, &mut card);
            });
    }
}
