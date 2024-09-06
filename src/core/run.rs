use std::error::Error;
use std::sync::{Arc, RwLock};

use rand::distributions::{Alphanumeric, DistString};
use ratatui::layout::{Constraint, Flex, Layout, Margin, Rect};
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::Frame;

use crate::components::round_info::RoundInfoWidget;
use crate::components::round_score::RoundScoreWidget;
use crate::components::run_stats::RunStatsWidget;
use crate::components::scorer_preview::ScorerPreviewWidget;
use crate::event::Event;
use crate::tui::TuiComponent;

use super::blind::{Blind, BlindType};
use super::deck::{Deck, Selectable};
use super::round::{Round, RoundProperties};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct RunProperties {
    pub ante: usize,
    pub hand_size: usize,
    pub max_discards: usize,
    pub max_hands: usize,
    pub seed: String,
    pub starting_money: usize,
}

impl Default for RunProperties {
    fn default() -> Self {
        Self {
            ante: 1,
            hand_size: 10,
            max_discards: 3,
            max_hands: 3,
            seed: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
            starting_money: 10,
        }
    }
}

#[derive(Debug, Default)]
pub struct Run {
    pub properties: RunProperties,
    pub money: usize,
    pub deck: Arc<RwLock<Deck>>,
    // TODO: Make round container optional and generic to be replaced between RoundSelection, Round, Shop and GameOver
    pub round: Round,
    pub upcoming_round_number: usize
}

impl Run {
    pub fn new(deck: Arc<RwLock<Deck>>, properties: RunProperties) -> Run {
        Run {
            deck: deck.clone(),
            properties: properties.clone(),
            money: properties.starting_money,
            upcoming_round_number: 1,
            round: Round {
                deck: deck.clone(),
                properties: RoundProperties {
                    round_number: 1,
                    blind: Blind::new(BlindType::SmallBlind, properties.ante).unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }

    #[inline]
    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.round.start()
    }
}

// TODO: Split/Flex all widgets in meta_area evenly.

// TODO: Move into component chunks and un-implement TuiComponent for Run.
impl TuiComponent for Run {
    #[inline]
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        // Prepare areas
        let [meta_area, play_area] = Layout::horizontal([Constraint::Percentage(25), Constraint::Fill(1)]).areas(rect);
        let [round_info_area, round_score_area, scoring_area, run_stats_area] = Layout::vertical([
            Constraint::Length(15),
            Constraint::Length(9),
            Constraint::Length(10),
            Constraint::Length(15),
        ]).flex(Flex::Center).areas(meta_area.inner(Margin::new(1, 0)));

        // Render containers
        frame.render_widget(Block::new().borders(Borders::LEFT | Borders::RIGHT), meta_area);
        frame.render_widget(Block::bordered().border_type(BorderType::Rounded), round_info_area);
        frame.render_widget(Block::bordered().border_type(BorderType::Rounded), round_score_area);
        frame.render_widget(Block::bordered().border_type(BorderType::Rounded), scoring_area);

        // Render widgets
        frame.render_stateful_widget(RoundInfoWidget::new(), round_info_area.inner(Margin::new(1, 1)), &mut self.round);
        frame.render_stateful_widget(RoundScoreWidget::new(), round_score_area.inner(Margin::new(1, 1)), &mut self.round);
        frame.render_stateful_widget(ScorerPreviewWidget::new(), scoring_area.inner(Margin::new(1, 1)), &mut self.round.hand.peek_selected().unwrap());
        frame.render_stateful_widget(RunStatsWidget::new(), run_stats_area, self);

        self.round.draw(frame, play_area);
    }

    #[inline]
    fn handle_events(&mut self, event: Event) {
        self.round.handle_events(event);
    }
}