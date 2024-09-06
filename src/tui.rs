use better_panic::{Settings, Verbosity};
use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend as Backend, layout::{Constraint, Flex, Layout, Rect}, style::{Color, Style, Styled}, text::{Line, Span, Text}, Frame, Terminal};
use std::{error::Error, io::{stderr, Stderr}, ops::{Deref, DerefMut}, panic::set_hook, process::exit};
use color_eyre::{config::HookBuilder, Result as EyreResult};

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
        init_panic_hook()?;
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
    type Target = Terminal<Backend<Stderr>>;

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
    fn draw(&mut self, frame: &mut Frame, rect: Rect);
    fn handle_events(&mut self, _event: Event) { }
}

pub fn center_widget(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
fn init_panic_hook() -> EyreResult<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default()
        .panic_section(format!("This is a bug. Consider reporting it at {}", env!("CARGO_PKG_REPOSITORY")))
        .display_location_section(true)
        .display_env_section(true)
        .into_hooks();

    eyre_hook.install()?;

    set_hook(Box::new(move |panic_info| {
        if let Ok(t) = Tui::new() {
            if let Err(r) = t.exit() {
                error!("Unable to exit Terminal: {:?}", r);
            }
        }

        let msg = format!("{}", panic_hook.panic_report(panic_info));

        #[cfg(not(debug_assertions))]
        {
            eprintln!("{}", msg); // prints color-eyre stack trace to stderr
            use human_panic::{handle_dump, print_msg, Metadata};
            let meta = Metadata {
                version: env!("CARGO_PKG_VERSION").into(),
                name: env!("CARGO_PKG_NAME").into(),
                authors: env!("CARGO_PKG_AUTHORS").replace(':', ", ").into(),
                homepage: env!("CARGO_PKG_HOMEPAGE").into(),
            };

            let file_path = handle_dump(&meta, panic_info);
            // prints human-panic message
            print_msg(file_path, &meta).expect("human-panic: printing error message to console failed");
        }

        error!("Error: {}", strip_ansi_escapes::strip_str(msg));

        #[cfg(debug_assertions)]
        {
            // Better Panic stacktrace that is only enabled when debugging.
            Settings::auto()
                .most_recent_first(false)
                .lineno_suffix(true)
                .verbosity(Verbosity::Full)
                .create_panic_handler()(panic_info);
        }

        exit(libc::EXIT_FAILURE);
    }));

    Ok(())
}
