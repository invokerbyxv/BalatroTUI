use std::{array::IntoIter, cmp::Ordering, fmt::{Display, Formatter, Result}};

use ratatui::{layout::{Alignment, Constraint, Layout, Margin, Rect}, text::Text, widgets::{Block, BorderType}, Frame};

use crate::{event::Event, tui::{center_widget, TuiComponent}};

const CARD_WIDTH: usize = 12;
const CARD_HEIGHT: usize = 9;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, Hash, PartialEq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    const VALUES: [Self; 4] = [Self::Club, Self::Diamond, Self::Heart, Self::Spade];

    #[inline]
    pub fn iter() -> IntoIter<Suit, 4> {
        Self::VALUES.into_iter()
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let suit_display = match *self {
            Suit::Club => "♣",
            Suit::Diamond => "♦",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };
        write!(f, "{}", suit_display)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: usize,
    pub score: usize,
    pub suit: Suit,
}

impl Card {
    #[inline]
    pub fn rank_display(&self) -> String {
        return match self.rank {
            // NOTE: Probably there's a better way to handle this with static str.
            1 => "A".to_owned(),
            13 => "K".to_owned(),
            12 => "Q".to_owned(),
            11 => "J".to_owned(),
            _ => self.rank.to_string(),
        };
    }
}

impl PartialOrd for Card {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank).then(self.suit.cmp(&other.suit))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}", self.suit, self.rank_display())
    }
}

impl TuiComponent for Card {
    #[inline]
    fn draw(&self, frame: &mut Frame, rect: Rect) {
        let card_layout = center_widget(rect, Constraint::Length(CARD_WIDTH as u16), Constraint::Length(CARD_HEIGHT as u16));

        frame.render_widget(Block::bordered().border_type(BorderType::Rounded), card_layout);

        let inner_rect = card_layout.inner(&Margin::new(1, 1));

        let card_layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2)
        ]).split(inner_rect);

        frame.render_widget(Text::raw(
            format!("{}\r\n{}", self.rank_display(), self.suit)
        ).alignment(Alignment::Left), card_layout[0]);

        // TODO: Mimic actual card suit layout
        frame.render_widget(Text::raw(
            format!("{}{}", self.suit, self.rank_display())
        ).alignment(Alignment::Center), center_widget(
            card_layout[1],
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ));

        frame.render_widget(Text::raw(
            format!("{}\r\n{}", self.suit, self.rank_display())
        ).alignment(Alignment::Right), card_layout[2]);
    }

    #[inline]
    fn handle_events(&mut self, _event: Event) { }
}
