//! [`Game`] is a logical wrapper around the main flow of the game, ie, [`Run`].
//! [`Game`] provides additional functionalities outside of the lifetime of an
//! instance of [`Run`].
//!
//! The entrypoint of game is [`Game::new()`] to create the instance of a new
//! game and [`Game::start()`] to spawn a new instance of a running game.

use std::sync::{Arc, RwLock};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{layout::Rect, Frame};

use crate::{
    core::{
        blind::Blind,
        deck::{Deck, DeckConstExt},
        round::{Round, RoundProperties},
        run::{Run, RunProperties},
    },
    event::{Event, EventHandler},
    tui::{Tui, TuiComponent},
};

/// Tick rate at which the game runs/receives updates.
pub const TICK_RATE: u64 = 144;

/// [`Game`] struct holds the state for the running game, including [`Run`]
/// surrounding states, that allow early closure of a run.
#[derive(Debug)]
pub struct Game {
    /// An instance of a [`Run`]. The run is the actual handler for most
    /// operations. [`Game`] simply forwards the requests to [`Run`] to handle.
    pub run: Run,
    /// A boolean flag denoting whether the game should send out shutdown
    /// signal.
    pub should_quit: bool,
}

impl Game {
    /// Create a new instance of a game.
    ///
    /// This acts as a no-parameter initialization point and should be placed
    /// between user initialization and persistent on-disk configurations.
    #[expect(
        clippy::new_without_default,
        reason = "Intended: Game should only be explicitly created hence default implementation can be skipped."
    )]
    #[must_use = "Created game instance must be used."]
    #[inline]
    pub fn new() -> Self {
        let deck = Arc::new(RwLock::new(Deck::standard()));
        let run_properties = RunProperties::default();
        let round_properties = RoundProperties::default();
        Self {
            run: Run {
                deck: Arc::clone(&deck),
                money: run_properties.starting_money,
                properties: run_properties.clone(),
                round: Round {
                    blind: Blind::Small,
                    deck: Arc::clone(&deck),
                    discards_count: run_properties.max_discards,
                    hand: vec![].into(),
                    hands_count: run_properties.max_hands,
                    history: vec![],
                    properties: round_properties,
                    score: 0,
                },
                upcoming_round_number: 1,
            },
            should_quit: false,
        }
    }

    /// Main entrypoint of the game.
    ///
    /// Creates a new [`Tui`] instance and initializes the [`EventHandler`].
    /// Runs the round initialization routine and the game `update` loop
    pub async fn start(&mut self) -> Result<()> {
        // Enter TUI
        let mut tui = Tui::new()?;
        tui.enter()?;

        // Spawn EventHandler
        let mut events = EventHandler::new(TICK_RATE);

        self.run.start()?;

        loop {
            self.handle_events(events.next().await?)?;
            tui.draw(|frame| {
                // TODO: Remove unwrap and propagate error to game instance.
                self.draw(frame, frame.area())
                    .wrap_err("Could not draw game on the given frame.");
            })
            .wrap_err("Could not draw on Tui buffer.")?;
            if self.should_quit {
                break;
            }
        }

        // Exit TUI
        tui.exit()?;

        Ok(())
    }
}

impl TuiComponent for Game {
    #[inline]
    fn draw(&mut self, frame: &mut Frame<'_>, rect: Rect) -> Result<()> {
        self.run.draw(frame, rect)?;

        Ok(())
    }

    fn handle_events(&mut self, event: Event) -> Result<()> {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "Intended: Unused events may skip implementation as required."
        )]
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.should_quit = true;
                }
                KeyCode::Char('c' | 'C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        self.should_quit = true;
                    }
                }
                _ => (),
            },
            Event::Resize(x_size, y_size) => {
                if y_size < 40 || x_size < 150 {
                    bail!("Terminal size was less than required to render game.");
                }
            }
            _ => (),
        }
        self.run.handle_events(event)?;

        Ok(())
    }
}
