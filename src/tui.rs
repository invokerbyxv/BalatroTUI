//! This module provides a [`Tui`] wrapper, implementing standard entry and
//! exit procedures to prepare rendering on the terminal.

// TODO: Convert this module to use color_eyre instead of panic and hook

use std::{
    io::{stderr, Stderr},
    ops::{Deref, DerefMut},
    panic::set_hook,
    process::exit,
};

use color_eyre::{config::HookBuilder, Result};
use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend as Backend,
    layout::Rect,
    style::{Color, Style, Styled},
    text::{Line, Span},
    Frame, Terminal,
};
use tracing::error;

use crate::event::Event;

/// [`Tui`] is a thin wrapper over [`ratatui`] with [`crossterm`] backend
/// providing methods to handle terminal based operations.
#[derive(Debug)]
pub struct Tui {
    /// Crossterm backend terminal instance.
    pub terminal: Terminal<Backend<Stderr>>,
}

// TODO: Use wrap_err wherever context needs to be added

impl Tui {
    /// Creates new Tui instance with crossterm backend.
    #[inline]
    pub fn new() -> Result<Self> {
        Ok(Self {
            terminal: Terminal::new(Backend::new(stderr()))?,
        })
    }

    /// Activates Tui instance and registers panic handlers.
    pub fn enter(&self) -> Result<()> {
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

    /// Deactivates Tui instance.
    pub fn exit(&self) -> Result<()> {
        crossterm::execute!(
            stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            cursor::Show
        )?;
        disable_raw_mode()?;
        Ok(())
    }

    /// Suspends Tui instance.
    pub fn suspend(&self) -> Result<()> {
        self.exit()?;
        #[cfg(not(windows))]
        raise(SIGTSTP)?;
        Ok(())
    }

    /// Resumes Tui instance.
    pub fn resume(&self) -> Result<()> {
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
        assert!(
            self.exit().is_ok(),
            "Failed to drop tui as exit call failed."
        );
    }
}

/// Trait to implement screen drawing and event handling methods.
#[deprecated(note = "Consider using widgets instead. This trait will be slowly phased out.")]
pub trait TuiComponent {
    /// Draws widgets and content on the screen `frame`, constrained within a
    /// `rect`.
    fn draw(&mut self, frame: &mut Frame<'_>, rect: Rect) -> Result<()>;
    /// Handle Tui captured events for the component.
    fn handle_events(&mut self, _event: Event) -> Result<()> {
        Ok(())
    }
}

// TODO: Move to a utility module
/// Returns line widget with chip icon prepended
pub fn get_line_with_chips<'widget, T: Into<Span<'widget>>>(
    content: T,
    color: Color,
) -> Line<'widget> {
    Line::from(vec![
        // TODO: Consider using BigText here
        "\u{26c0}".set_style(Style::new().fg(color)),
        "  ".into(),
        content.into(),
    ])
}

/// Installs custom panic hook to work with `color_eyre`, `human_panic` and
/// `better_panic`.
fn init_panic_hook() -> Result<()> {
    // TODO: Use this reporting text with color_eyre as well
    let (panic_hook, eyre_hook) = HookBuilder::default()
        .panic_section(format!(
            "This is a bug. Consider reporting it at {}.",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .display_location_section(true)
        .display_env_section(true)
        .into_hooks();

    eyre_hook.install()?;

    set_hook(Box::new(move |panic_info| {
        if let Ok(tui) = Tui::new() {
            if let Err(err) = tui.exit() {
                error!("Unable to exit Terminal: {:?}.", err);
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
            print_msg(file_path, &meta)
                .expect("human-panic: printing error message to console failed");
        }

        error!("Error: {}", strip_ansi_escapes::strip_str(msg));

        #[cfg(debug_assertions)]
        {
            use better_panic::{Settings, Verbosity};
            // Better Panic stacktrace that is only enabled when debugging.
            Settings::auto()
                .most_recent_first(false)
                .lineno_suffix(true)
                .verbosity(Verbosity::Full)
                .create_panic_handler()(panic_info);
        }

        #[expect(
            clippy::exit,
            reason = "Intended: Calling exit inside panic hook is acceptable."
        )]
        exit(libc::EXIT_FAILURE);
    }));

    Ok(())
}
