use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default)]
pub struct CycleCursor<T> {
    pub inner: Vec<T>,
    pub pos: Option<usize>,
}

impl<I> From<I> for CycleCursor<I::Item>
where
    I: IntoIterator,
{
    fn from(inner: I) -> Self {
        Self {
            inner: inner.into_iter().collect(),
            pos: None,
        }
    }
}

impl<T> CycleCursor<T> {
    pub fn cycle_next(&mut self) {
        let max_items = self.inner.len();
        let pos = (self.pos.unwrap_or(max_items - 1) + max_items + 1) % max_items;
        self.pos = Some(pos);
    }

    pub fn cycle_prev(&mut self) {
        let max_items = self.inner.len();
        let pos = (self.pos.unwrap_or(max_items) + max_items - 1) % max_items;
        self.pos = Some(pos);
    }
}

impl<T> Deref for CycleCursor<T> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for CycleCursor<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn cursor_cycle_next() {
        let source = vec![1, 2, 3, 4];
        let mut cursor: CycleCursor<usize> = source.into();

        assert_eq!(cursor.pos, None);
        cursor.cycle_next();
        assert_eq!(cursor.inner[cursor.pos.unwrap()], 1);
        cursor.cycle_next();
        cursor.cycle_next();
        cursor.cycle_next();
        assert_eq!(cursor.inner[cursor.pos.unwrap()], 4);
        cursor.cycle_next();
        assert_eq!(cursor.inner[cursor.pos.unwrap()], 1);
    }

    #[test]
    fn cursor_cycle_prev() {
        let source = HashSet::from([1, 2, 3, 4]);
        let mut cursor: CycleCursor<usize> = source.into();

        assert_eq!(cursor.pos, None);
        cursor.cycle_prev();
        assert_eq!(cursor.inner[cursor.pos.unwrap()], 4);
        cursor.cycle_prev();
        cursor.cycle_prev();
        cursor.cycle_prev();
        assert_eq!(cursor.inner[cursor.pos.unwrap()], 1);
        cursor.cycle_prev();
        assert_eq!(cursor.inner[cursor.pos.unwrap()], 4);
    }
}