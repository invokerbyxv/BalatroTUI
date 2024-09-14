//! Run is a complete play-through of the game until game over.
//!
//! Across a run, there are multiple rounds played. If any round is failed, the
//! run is over.

use std::sync::{Arc, RwLock};

use color_eyre::Result;
use rand::distributions::{Alphanumeric, DistString};
use ratatui::{
    layout::{Constraint, Flex, Layout, Margin, Rect},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use super::{deck::Deck, round::Round};
use crate::{
    components::{RoundInfoWidget, RoundScoreWidget, RunStatsWidget, ScorerPreviewWidget},
    event::Event,
    tui::TuiComponent,
};

/// Persistent details about the run.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RunProperties {
    /// The number of cards to be fetched in hand during the round.
    pub hand_size: usize,
    /// Maximum discards available per round.
    pub max_discards: usize,
    /// Maximum hands that can be made per round.
    pub max_hands: usize,
    /// Random seed for the run.
    pub seed: String,
    /// Initial amount of money that the run starts with.
    pub starting_money: usize,
}

impl Default for RunProperties {
    #[inline]
    fn default() -> Self {
        Self {
            hand_size: 10,
            max_discards: 3,
            max_hands: 3,
            seed: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
            starting_money: 10,
        }
    }
}

/// [`Run`] struct maintains the working state of a run, along with the rounds
/// that are selected.
///
/// A single run is maintained from the point a deck is selected to the point of
/// game over.
#[derive(Debug)]
pub struct Run {
    /// Persistent properties for the run.
    pub properties: RunProperties,
    /// Current money held by the user.
    pub money: usize,
    /// Shared deck of cards across rounds. [`Run`] simply passes this on to the
    /// [`Round`] instance.
    pub deck: Arc<RwLock<Deck>>,
    // TODO: Make round container optional and generic to be replaced between RoundSelection,
    // Round, Shop and GameOver
    /// An instance of a [`Round`].
    pub round: Round,
    /// Used to keep track of the last played [`Round`] number.
    pub upcoming_round_number: usize,
}

impl Run {
    /// Main entrypoint of the run. It initializes the internal state and spawns
    /// a round.
    #[inline]
    pub fn start(&mut self) -> Result<()> {
        self.round.start()
    }
}

// TODO: Split/Flex all widgets in meta_area evenly.

// TODO: Move into component chunks and un-implement TuiComponent for Run.
impl TuiComponent for Run {
    fn draw(&mut self, frame: &mut Frame<'_>, rect: Rect) -> Result<()> {
        // Prepare areas
        let [meta_area, play_area] =
            Layout::horizontal([Constraint::Percentage(25), Constraint::Fill(1)]).areas(rect);
        let [
            round_info_area,
            round_score_area,
            scoring_area,
            run_stats_area,
        ] = Layout::vertical([
            // TODO: Infer from content length
            Constraint::Length(15),
            Constraint::Length(9),
            Constraint::Length(12),
            Constraint::Length(17),
        ])
        .flex(Flex::Center)
        .areas(meta_area.inner(Margin::new(1, 0)));

        // Render containers
        frame.render_widget(
            Block::new().borders(Borders::LEFT | Borders::RIGHT),
            meta_area,
        );
        frame.render_widget(
            Block::bordered().border_type(BorderType::Rounded),
            round_info_area,
        );
        frame.render_widget(
            Block::bordered().border_type(BorderType::Rounded),
            round_score_area,
        );
        frame.render_widget(
            Block::bordered().border_type(BorderType::Rounded),
            scoring_area,
        );

        // Render widgets
        frame.render_stateful_widget(
            RoundInfoWidget::new(),
            round_info_area.inner(Margin::new(1, 1)),
            &mut self.round,
        );
        frame.render_stateful_widget(
            RoundScoreWidget::new(),
            round_score_area.inner(Margin::new(1, 1)),
            &mut self.round,
        );
        frame.render_stateful_widget(
            ScorerPreviewWidget::new(),
            scoring_area.inner(Margin::new(1, 1)),
            &mut self.round.hand.peek_selected()?,
        );
        frame.render_stateful_widget(RunStatsWidget::new(), run_stats_area, self);

        self.round.draw(frame, play_area)?;

        Ok(())
    }

    fn handle_events(&mut self, event: Event) -> Result<()> {
        self.round.handle_events(event)?;

        Ok(())
    }
}
