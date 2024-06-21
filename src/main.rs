use core::{deck::Deck, run::{Run, RunProperties}};
use std::sync::{Arc, RwLock};

pub mod core;

fn main() {
    // Select a deck
    let deck = Deck::standard();

    // Start a run
    let mut run = Run {
        deck: Arc::new(RwLock::new(deck)),
        properties: RunProperties::default(),
        ..Default::default()
    };
    run.start().unwrap();
}
