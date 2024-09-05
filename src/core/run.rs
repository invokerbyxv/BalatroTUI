use std::error::Error;
use std::sync::{Arc, RwLock};

use rand::distributions::{Alphanumeric, DistString};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::components::round_info::RoundInfoWidget;
use crate::components::run_stats::RunStatsWidget;
use crate::components::scorer_preview::ScorerPreviewWidget;
use crate::event::Event;
use crate::tui::{center_widget, TuiComponent};

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
    pub round: Round,
}

impl Run {
    pub fn new(deck: Arc<RwLock<Deck>>, properties: RunProperties) -> Run {
        Run {
            deck: deck.clone(),
            properties: properties.clone(),
            money: properties.starting_money,
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

// TODO: Move into component chunks and un-implement TuiComponent for Run.
impl TuiComponent for Run {
    #[inline]
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let [mut meta_area, play_area] = Layout::horizontal([Constraint::Percentage(25), Constraint::Fill(1)]).areas(rect);
        frame.render_widget(Block::new().borders(Borders::LEFT | Borders::RIGHT), meta_area);

        // TODO: Split/Flex all widgets in meta_area evenly.
        meta_area = meta_area.inner(&Margin::new(1, 0));

        let [round_info_area, round_score_area, scoring_area, run_details_area] = Layout::vertical([
            Constraint::Fill(4),
            Constraint::Length(5),
            Constraint::Fill(3),
            Constraint::Fill(4),
        ]).areas(meta_area);

        let [round_score_text_area, round_score_value_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]).areas(round_score_area.inner(&Margin::new(1, 1)));

        let [run_info_button_area, run_stats_area] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(2),
        ]).areas(run_details_area);

        frame.render_stateful_widget(RoundInfoWidget::new(), round_info_area, &mut self.round);
        frame.render_widget(Block::bordered().border_type(BorderType::Rounded), round_info_area);

        // TODO: Add chip image next to round score.
        frame.render_widget(Block::bordered().border_type(BorderType::Rounded), round_score_area);
        frame.render_widget(Paragraph::new("Round Score"), center_widget(round_score_text_area, Constraint::Percentage(50), Constraint::Length(1)));
        frame.render_widget(Paragraph::new(self.round.score.to_string()), center_widget(round_score_value_area, Constraint::Percentage(50), Constraint::Length(1)));

        frame.render_stateful_widget(ScorerPreviewWidget::new(), scoring_area, &mut self.round.hand.peek_selected().unwrap());
        // TODO: Load ScoringWidget here.
        // TODO: Load RunInfoButtonWidget here.
        frame.render_stateful_widget(RunStatsWidget::new(), run_stats_area, self);

        self.round.draw(frame, play_area);
    }

    #[inline]
    fn handle_events(&mut self, event: Event) {
        self.round.handle_events(event);
    }
}