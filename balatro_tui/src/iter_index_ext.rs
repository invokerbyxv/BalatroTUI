//! This module extends the standard [`Iterator`] by adding arbitrary indexing
//! based operations.

use bit_set::BitSet;
use color_eyre::eyre::{OptionExt, Result};
use itertools::{Either, Itertools};

/// Provides methods to perform container/iterator methods based on index set.
pub(crate) trait IterIndexExt
where
    Self: IntoIterator + Sized,
{
    /// Returns a cloned [`Vec`] based on arbitrary indices set.
    fn peek_at_index_set(&self, index_set: &BitSet) -> Result<Self>;
    /// Drains the iterator based on arbitrary indices (see [`Vec::drain()`] for
    /// equivalent usage with contiguous range) and returns the drained items in
    /// a [`Vec`].
    fn drain_from_index_set(&mut self, index_set: &BitSet) -> Result<Self>;
}

impl<T: Copy> IterIndexExt for Vec<T> {
    fn peek_at_index_set(&self, index_set: &BitSet) -> Result<Self> {
        index_set
            .iter()
            .map(|idx| {
                self.get(idx)
                    .copied()
                    .ok_or_eyre("Invalid index accessed. Index set may be invalid.")
            })
            .process_results(|iter| iter.collect())
    }

    fn drain_from_index_set(&mut self, index_set: &BitSet) -> Result<Self> {
        let (selected, leftover): (Self, Self) = self
            .iter()
            .enumerate()
            .map(|(idx, &card)| (idx, card))
            .partition_map(|(idx, card)| {
                if index_set.contains(idx) {
                    Either::Left(card)
                } else {
                    Either::Right(card)
                }
            });

        *self = leftover;

        Ok(selected)
    }
}
