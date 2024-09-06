use std::{error::Error, sync::{Arc, RwLock}};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{layout::Rect, Frame};

use crate::{core::{deck::Deck, run::{Run, RunProperties}}, event::{Event, EventHandler}, tui::{Tui, TuiComponent}};

const TICK_RATE: u64 = 144;

pub struct Game {
    pub run: Run,
    pub should_quit: bool,
}

impl Game {
    #[inline]
    pub fn new() -> Self {
        let run = Run::new(
            Arc::new(RwLock::new(Deck::standard())),
            RunProperties::default()
        );
        Game { run, should_quit: false, }
    }

    #[inline]
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // Enter TUI
        let mut tui = Tui::new()?;
        tui.enter()?;

        // Spawn EventHandler
        let mut events = EventHandler::new(TICK_RATE);

        self.run.start()?;

        loop {
            self.handle_events(events.next().await?);
            tui.draw(|frame| {
                self.draw(frame, frame.area());
            })?;
            if self.should_quit {
                break
            }
        }

        // Exit TUI
        tui.exit()?;

        Ok(())
    }
}

impl TuiComponent for Game {
    #[inline]
    fn draw(&mut self, frame: &mut Frame, rect: Rect)  {
        self.run.draw(frame, rect);
    }

    #[inline]
    fn handle_events(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        self.should_quit = true;
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        if key_event.modifiers == KeyModifiers::CONTROL {
                            self.should_quit = true;
                        }
                    }
                    _ => ()
                }
            }
            Event::Resize(x_size, y_size) => {
                if y_size < 40 || x_size < 150 {
                    panic!("Terminal size was less than required to render game");
                }
            }
            _ => ()
        }
        self.run.handle_events(event);
    }
}
