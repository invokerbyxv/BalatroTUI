use std::{error::Error, io::{Error as IOError, ErrorKind}, time::Duration};

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent, MouseEvent};

use futures::{FutureExt, StreamExt};
use tokio::{select, spawn, sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}, task::JoinHandle, time::interval};
use tokio_util::sync::CancellationToken;

/// Terminal events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    sender: UnboundedSender<Event>,
    receiver: UnboundedReceiver<Event>,
    handler: Option<JoinHandle<()>>,
    stop_cancellation_token: CancellationToken,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);

        let (sender, receiver) = unbounded_channel();
        let _sender = sender.clone();

        let stop_cancellation_token = CancellationToken::new();
        let _stop_cancellation_token = stop_cancellation_token.clone();

        let handler = spawn(async move {
            let mut reader = EventStream::new();
            let mut tick = interval(tick_rate);
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                select! {
                    _ = _sender.closed() => {
                        break;
                    }
                    _ = tick_delay => {
                        _sender.send(Event::Tick).unwrap();
                    }
                    _ = _stop_cancellation_token.cancelled() => {
                        break;
                    }
                    Some(Ok(evt)) = crossterm_event => {
                        match evt {
                            CrosstermEvent::Key(key) => {
                                if key.kind == crossterm::event::KeyEventKind::Press {
                                    _sender.send(Event::Key(key)).unwrap();
                                }
                            },
                            CrosstermEvent::Mouse(mouse) => {
                                _sender.send(Event::Mouse(mouse)).unwrap();
                            },
                            CrosstermEvent::Resize(x, y) => {
                                _sender.send(Event::Resize(x, y)).unwrap();
                            },
                            CrosstermEvent::FocusLost => { },
                            CrosstermEvent::FocusGained => { },
                            CrosstermEvent::Paste(_) => { },
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

    // TODO: Handle all errors using single interceptor point. Create custom errors for handling different error/panic types.

    pub async fn next(&mut self) -> Result<Event, Box<dyn Error>> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(IOError::new(
                ErrorKind::Other,
                "This is an IO error",
            )))
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.stop_cancellation_token.cancel();
        if let Some(handle) = self.handler.take() {
            handle.await.unwrap();
        }
        Ok(())
    }
}
