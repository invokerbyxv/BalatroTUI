use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend as Backend, layout::{Constraint, Flex, Layout, Rect}, Frame, Terminal};
use std::{error::Error, io::{stderr, Stderr}, ops::{Deref, DerefMut}};

use crate::event::Event;

pub struct Tui {
    pub terminal: Terminal<Backend<Stderr>>,
}

impl Tui {
    #[inline]
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let terminal = Terminal::new(Backend::new(stderr()))?;
        Ok(Self { terminal })
    }

    #[inline]
    pub fn enter(&self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        crossterm::execute!(
            stderr(),
            EnterAlternateScreen,
            EnableMouseCapture,
            cursor::Hide
        )?;
        Ok(())
    }

    #[inline]
    pub fn exit(&self) -> Result<(), Box<dyn Error>> {
        crossterm::execute!(
            stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            cursor::Show
        )?;
        disable_raw_mode()?;
        Ok(())
    }

    #[inline]
    pub fn suspend(&self) -> Result<(), Box<dyn Error>> {
        self.exit()?;
        #[cfg(not(windows))]
        raise(SIGTSTP)?;
        Ok(())
    }

    #[inline]
    pub fn resume(&self) -> Result<(), Box<dyn Error>> {
        self.enter()?;
        Ok(())
    }
}

impl Deref for Tui {
    type Target = ratatui::Terminal<Backend<Stderr>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Tui {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Tui {
    #[inline]
    fn drop(&mut self) {
        self.exit().unwrap();
    }
}

pub trait TuiComponent {
    fn draw(&self, frame: &mut Frame, rect: Rect);
    fn handle_events(&mut self, _event: Event) { }
}

pub fn center_widget(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}