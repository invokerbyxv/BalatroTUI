// TODO: Add cycle cursor base type instead of Vec directly.

use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default)]
pub struct CycleCursorVec<T> {
    pub inner: Vec<T>,
    pub pos: Option<usize>,
}

impl<T> From<Vec<T>> for CycleCursorVec<T> {
    fn from(inner: Vec<T>) -> Self {
        Self {
            inner,
            pos: None,
        }
    }
}

impl<T> CycleCursorVec<T> {
    #[inline]
    pub fn cycle_next(&mut self) {
        let max_items = self.inner.len();
        let pos = (self.pos.unwrap_or(max_items - 1) + max_items + 1) % max_items;
        self.pos = Some(pos);
    }

    #[inline]
    pub fn cycle_prev(&mut self) {
        let max_items = self.inner.len();
        let pos = (self.pos.unwrap_or(max_items) + max_items - 1) % max_items;
        self.pos = Some(pos);
    }
}

impl<T> Deref for CycleCursorVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for CycleCursorVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}