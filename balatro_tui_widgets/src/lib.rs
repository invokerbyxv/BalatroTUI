//! Widgets and components for displaying elements of `BalatroTUI` on the
//! terminal.

#![expect(
    clippy::missing_docs_in_private_items,
    reason = "Intended: This module's contents are re-exported."
)]

mod blind_badge;
mod card;
mod card_list;
pub mod error;
mod round_info;
mod round_score;
mod run_stats;
mod scorer_preview;
mod text_box;
mod utility;

pub use blind_badge::*;
pub use card::*;
pub use card_list::*;
pub use round_info::*;
pub use round_score::*;
pub use run_stats::*;
pub use scorer_preview::*;
pub use text_box::*;
