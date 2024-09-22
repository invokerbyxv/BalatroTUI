//! This module provides error definitions for this crate.

use std::sync::{RwLockReadGuard, TryLockError};

use thiserror::Error;

/// Defines errors relating to arithmetic operation failures.
#[derive(Clone, Copy, Debug, Error)]
pub enum ArithmeticError {
    /// Signifies that an arithmetic operation has overflown. Error message
    /// provides which kind of operation is overflowing.
    #[error("Arithmetic operation {0} overflowed")]
    Overflow(&'static str),
}

/// Defines errors relating to card list widget.
#[derive(Clone, Debug, Error)]
pub enum WidgetError {
    /// Provides conversion from [`ArithmeticError`] to [`WidgetError`].
    #[error("Arithmetic error occurred in scorer: {0:?}")]
    ArithmeticError(#[from] ArithmeticError),

    /// Signifies that the selection limit has overflown the source container
    /// size. Error message provides attempted selection limit and maximum
    /// allowed limit.
    #[error("Cannot reduce selection limit if number of selected cards is more than it.")]
    SelectionLimitOverflow {
        /// The attempted value of selection limit
        attempted_selection_limit: usize,
        /// The maximum allowed valid value for selection limit
        max_allowed: usize,
    },

    /// Signifies inability to acquire write lock on shared widget state. This
    /// should result in immediate exit and cleanup.
    #[error("Could not acquire write lock on widget state: {0:?}")]
    WidgetStateLockError(String),
}

impl<'guard, T> From<TryLockError<RwLockReadGuard<'guard, T>>> for WidgetError {
    fn from(source: TryLockError<RwLockReadGuard<'guard, T>>) -> Self {
        Self::WidgetStateLockError(format!("{source:?}"))
    }
}
