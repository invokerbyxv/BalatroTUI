use std::error::Error;

use game::Game;

pub mod components;
pub mod core;
pub mod primitives;
pub mod event;
pub mod game;
pub mod tui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Start Game
    let mut game = Game::new();
    let _ = game.start().await;

    Ok(())
}
