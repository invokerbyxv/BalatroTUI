//! This module provides [`EventHandler`] struct that handles sending and
//! receiving events asynchronously.
//!
//! The event handler provides a multi-read, multi-write wrapper that returns
//! [`Event`] using [`EventHandler::next()`].

use std::time::Duration;

use color_eyre::eyre::{Context, OptionExt, Result};
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::{
    select, spawn,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
    time::interval,
};
use tokio_util::sync::CancellationToken;

/// This enum specifies the different events that can be sent over the
/// [`EventHandler`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    /// [`Event::Tick`] event corresponds to a tick of the underlying game loop.
    /// This can be used for constant evaluation per tick. Consider creating a
    /// `select!` arm for this when making arbitrary checks on the state of the
    /// game loop.
    Tick,
    /// [`Event::Key`] event is sent when a key is pressed or released on the
    /// event-accepting interface.
    Key(KeyEvent),
    /// [`Event::Mouse`] event is sent when a mouse is moved or mouse buttons
    /// are pressed or released on the event-accepting interface.
    Mouse(MouseEvent),
    /// [`Event::Resize`] event is sent when the event-accepting interface is
    /// resized. Use this for recomputing render requirements.
    Resize(u16, u16),
    // TODO: Use Exit event instead of should_quit boolean flag in game
    /// [`Event::Exit`] event is the last event sent automatically by the event
    /// handler. Use this for gracefully exiting the game loop.
    Exit,
}

/// [`EventHandler`] is a wrapper interface that keeps track of and propagates
/// events from an event-accepting interface to the game loop.
#[derive(Debug)]
pub struct EventHandler {
    /// Sender allows asynchronous, thread-safe sends to the event handler.
    sender: UnboundedSender<Event>,
    /// Receiver allows asynchronous, thread-safe consumption from the event
    /// handler.
    receiver: UnboundedReceiver<Event>,
    /// Taking from handler marks the end of the [`EventHandler`]. It safely
    /// closes both sender and receiver objects.
    handler: Option<JoinHandle<()>>,
    /// Signals cancellation from consumer to exit the [`EventHandler`].
    stop_cancellation_token: CancellationToken,
}

impl EventHandler {
    /// Creates a new [`EventHandler`] instance and creates a handler for
    /// sending [`Event`] instances.
    #[must_use = "Created event handler instance must be used."]
    pub fn new(tick_rate: u64) -> Self {
        let tick_duration = Duration::from_millis(tick_rate);

        let (sender, receiver) = unbounded_channel();
        let local_sender = sender.clone();

        let stop_cancellation_token = CancellationToken::new();
        let local_stop_cancellation_token = stop_cancellation_token.clone();

        let handler = spawn(async move {
            let mut reader = EventStream::new();
            let mut tick = interval(tick_duration);

            #[expect(
                clippy::pattern_type_mismatch,
                clippy::ignored_unit_patterns,
                clippy::integer_division_remainder_used,
                reason = "False positive: Tokio's select! macro has different semantics than match statements."
            )]
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                select! {
                    _ = local_sender.closed() => {
                        break;
                    }
                    _ = tick_delay => {
                        local_sender.send(Event::Tick).wrap_err("Unable to send Tick event over sender channel.");
                    }
                    _ = local_stop_cancellation_token.cancelled() => {
                        break;
                    }
                    Some(Ok(event)) = crossterm_event => {
                        match event {
                            CrosstermEvent::Key(key) => {
                                if key.kind == KeyEventKind::Press {
                                    local_sender.send(Event::Key(key)).wrap_err("Unable to send Key event over sender channel.");
                                }
                            },
                            CrosstermEvent::Mouse(mouse) => {
                                local_sender.send(Event::Mouse(mouse)).wrap_err("Unable to send Mouse event over sender channel.");
                            },
                            CrosstermEvent::Resize(x, y) => {
                                local_sender.send(Event::Resize(x, y)).wrap_err("Unable to send Resize event over sender channel.");
                            },
                            CrosstermEvent::FocusLost
                            | CrosstermEvent::FocusGained
                            | CrosstermEvent::Paste(_) => { },
                        }
                    }
                };
            }
        });

        Self {
            sender,
            receiver,
            handler: Some(handler),
            stop_cancellation_token,
        }
    }

    // TODO: Handle all errors using single interceptor point. Create custom errors
    // for handling different error/panic types.

    /// Sends the next asynchronous [`Event`] instance.
    ///
    /// This requires an `await` call on the returning [`Event`] instance or it
    /// can be chained with other `async` tasks.
    pub async fn next(&mut self) -> Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_eyre("Cannot receive next event.")
    }

    /// Halts the [`Event`] stream via a cancellation token and sends an
    /// [`Event::Exit`] event to gracefully exit the game loop.
    pub async fn stop(&mut self) -> Result<()> {
        self.stop_cancellation_token.cancel();
        self.sender.send(Event::Exit)?;
        if let Some(handle) = self.handler.take() {
            return handle.await.wrap_err("Cannot stop event handle.");
        }
        Ok(())
    }
}
