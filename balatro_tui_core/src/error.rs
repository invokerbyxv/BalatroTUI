//! This module provides error definitions for this crate.

use std::{
    num::ParseIntError,
    sync::{RwLockWriteGuard, TryLockError},
};

use strum::ParseError;
use thiserror::Error;

/// Defines errors relating to arithmetic operation failures.
#[derive(Clone, Copy, Debug, Error)]
pub enum ArithmeticError {
    /// Signifies that an arithmetic operation has overflown. Error message
    /// provides which kind of operation is overflowing.
    #[error("Arithmetic operation {0} overflowed")]
    Overflow(&'static str),
}

/// Defines errors relating to conversion and parsing between enum, string and
/// integers for the target enum and its properties.
#[derive(Clone, Debug, Error)]
pub enum StrumError {
    /// Provides conversion from [`ParseError`] to [`StrumError`] used in
    /// [`std::str::FromStr`] trait.
    #[error("Parsing enum from string failed: {0:?}")]
    FromStringError(#[from] ParseError),

    /// Provides conversion from [`ParseError`] to [`StrumError`] using
    /// [`str::parse()`].
    #[error("Parsing integer property from enum failed: {0:?}")]
    ParseIntError(#[from] ParseIntError),

    /// Signifies that a property defined using [`strum::EnumProperty`] was not
    /// found on a queried variant.
    #[error("Enum property {property} not found on variant: {variant}")]
    PropertyNotFound {
        /// The queried enum property.
        property: String,
        /// The associated queried variant which did not specify the property.
        variant: String,
    },

    /// Signifies failure to parse the suit when parsing a card from a string.
    #[error("Unpacking suit failed when parsing card: {0:?}")]
    SuitUnpackError(String),
}

/// Defines errors related to scorer and scoring methods.
#[derive(Clone, Debug, Error)]
pub enum ScorerError {
    // TODO: Remove when infinite ante is implemented.
    /// Signifies that the current `ante` has exceeded the maximum possible
    /// `ante` count of 8.
    #[error("Current ante has crossed maximum computable ante: {0}")]
    AnteExceeded(usize),

    /// Provides conversion from [`ArithmeticError`] to [`ScorerError`].
    #[error("Arithmetic error occurred in scorer: {0:?}")]
    ArithmeticError(#[from] ArithmeticError),

    /// Signifies that an empty hand (no cards) were passed in the scorer.
    #[error("Attempted to score a hand with no cards")]
    EmptyHandScoredError,

    /// Provides conversion from [`StrumError`] to [`ScorerError`].
    #[error("Error occurred in parsing enum: {0:?}")]
    StrumError(#[from] StrumError),
}

/// Defines top-level errors for the crate.
#[derive(Clone, Debug, Error)]
pub enum CoreError {
    /// Signifies inability to acquire write lock on shared `deck`. This should
    /// result in immediate exit and cleanup.
    #[error("Could not acquire write lock on deck: {0:?}")]
    DeckLockError(String),

    /// Signifies that a hand discard was attempted when discards were not
    /// available.
    #[error("Attempted to discard hand but no discards remaining")]
    DiscardsExhaustedError,

    /// Signifies that a hand play was attempted when hands were not available.
    #[error("Attempted to play hand but no hands remaining")]
    HandsExhaustedError,

    /// Provides conversion from [`ArithmeticError`] to [`ScorerError`].
    #[error("Arithmetic error occurred in scorer: {0:?}")]
    ArithmeticError(#[from] ArithmeticError),

    /// Provides conversion from [`ScorerError`] to [`ScorerError`].
    #[error("Error occurred in scorer: {0:?}")]
    ScorerError(#[from] ScorerError),

    /// Provides conversion from [`StrumError`] to [`ScorerError`].
    #[error("Error occurred in parsing enum: {0:?}")]
    StrumError(#[from] StrumError),
}

impl<'guard, T> From<TryLockError<RwLockWriteGuard<'guard, T>>> for CoreError {
    fn from(source: TryLockError<RwLockWriteGuard<'guard, T>>) -> Self {
        Self::DeckLockError(format!("{source:?}"))
    }
}
