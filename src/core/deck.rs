use std::{cmp::Reverse, collections::HashSet, error::Error};
use crossterm::event::KeyCode;
use once_cell::sync::Lazy;
use rand::{thread_rng, seq::SliceRandom};
use itertools::{Either, Itertools};
use ratatui::{layout::{Constraint, Layout, Offset, Rect}, Frame};
use strum::IntoEnumIterator;

use crate::{components::card::{CardVisualState, CardWidget, CardWidgetState}, event::Event, primitives::cycle_cursor::CycleCursor, tui::TuiComponent};

use super::card::{Card, Rank, Suit};

// TODO: Use dynamic trait switching to achieve suit and rank sorting. Feed the impl directly to card instead of MultiSortable
// TODO: Impl default traits for all structs
// TODO: Migrate Drawable and Selectable to card widget instead

const MAXIMUM_SELECTED_CARDS: usize = 5;

static DEFAULT_DECK: Lazy<Vec<Card>> = Lazy::new(|| Suit::iter().flat_map(
    move |suit| Rank::iter().map(
        move |rank| Card { suit, rank }
    )
).collect());

#[derive(Clone, Debug)]
pub struct Deck {
    pub cards: CycleCursor<Card>,
    pub selected: HashSet<usize>,
}

impl Deck {
    #[inline]
    pub fn standard() -> Self {
        Deck {
            cards: DEFAULT_DECK.to_vec().into(),
            selected: HashSet::new(),
        }
    }

    #[inline]
    pub fn shuffle(&mut self) {
        // TODO: Bias with seed
        self.cards.shuffle(&mut thread_rng());
    }
}

impl Default for Deck {
    #[inline]
    fn default() -> Self {
        Deck::standard()
    }
}

pub trait Drawable {
    fn draw_random(&mut self, hand_size: usize) -> Result<Self, Box<dyn Error>> where Self : Sized;
}

impl Drawable for Deck {
    fn draw_random(&mut self, hand_size: usize) -> Result<Self, Box<dyn Error>> {
        if hand_size > self.cards.len() {
            // TODO: Define custom error
            Err("Hand size cannot be greater than the source deck.")?
        }
        self.shuffle();
        let drain_size = self.cards.len() - hand_size;
        Ok(Self {
            cards: self.cards.drain(drain_size..).collect::<Vec<Card>>().into(),
            selected: HashSet::new(),
        })
    }
}

pub trait Selectable where Self : Drawable {
    fn select(&mut self, selection: usize);
    fn deselect(&mut self, selection: usize);
    fn peek_selected(&self) -> Result<Vec<Card>, Box<dyn Error>>;
    fn draw_selected(&mut self) -> Result<Vec<Card>, Box<dyn Error>>;
}

impl Selectable for Deck {
    #[inline]
    fn select(&mut self, selection: usize) {
        self.selected.insert(selection);
    }

    #[inline]
    fn deselect(&mut self, selection: usize) {
        self.selected.remove(&selection);
    }

    #[inline]
    fn peek_selected(&self) -> Result<Vec<Card>, Box<dyn Error>> {
        Ok(self.selected.iter().map(|&idx| self.cards.inner[idx]).collect())
    }

    #[inline]
    fn draw_selected(&mut self) -> Result<Vec<Card>, Box<dyn Error>> {
        let (selected, leftover): (Vec<_>, Vec<_>) = self.cards.iter().enumerate().partition_map(|(idx, card)| {
            if self.selected.contains(&idx) {
                Either::Left(card)
            } else {
                Either::Right(card)
            }
        });
        self.selected.clear();
        self.cards = leftover.into();
        Ok(selected)
    }
}

pub trait Sortable {
    fn sort_by_suit(&mut self);
    fn sort_by_rank(&mut self);
}

impl Sortable for Deck {
    #[inline]
    fn sort_by_suit(&mut self) {
        self.cards.sort_by_key(|c| (c.suit, Reverse(c.rank)));
    }

    #[inline]
    fn sort_by_rank(&mut self) {
        self.cards.sort_by_key(|c| (Reverse(c.rank), c.suit));
    }
}

// TODO: Use ListWidget to handle selection instead.

impl TuiComponent for Deck {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let deck_layout = Layout::horizontal(vec![Constraint::Fill(1); self.cards.len()]).split(rect);
        let hover_position = self.cards.pos;
        for (idx, card) in self.cards.iter_mut().enumerate() {
            let mut card_widget_state = CardWidgetState::from(card);

            if hover_position == Some(idx) {
                card_widget_state.visual_state = CardVisualState::Hovered;
            }

            let mut area = deck_layout[idx];

            if self.selected.contains(&idx) {
                area = deck_layout[idx].offset(Offset { x: 0, y: -5 });
            }

            frame.render_stateful_widget(CardWidget::new(), area, &mut card_widget_state);
        }
    }

    fn handle_events(&mut self, event: Event) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Right => {
                    self.cards.cycle_next();
                }
                KeyCode::Left => {
                    self.cards.cycle_prev();
                }
                KeyCode::Up => {
                    if self.selected.len() < MAXIMUM_SELECTED_CARDS {
                        if let Some(pos) = self.cards.pos {
                            self.select(pos);
                        }
                    }
                }
                KeyCode::Down => {
                        if let Some(pos) = self.cards.pos {
                            self.deselect(pos);
                        }
                }
                _ => ()
            }
        }
    }
}
