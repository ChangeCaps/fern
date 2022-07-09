use std::ops::{BitOr, BitOrAssign};

use crate::source::SourceId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// Source.
    source: SourceId,
    /// The byte index to start [`Self::source`].
    index: usize,
    /// The length of bytes.
    length: usize,
}

impl Span {
    pub const fn new(source: SourceId, index: usize, length: usize) -> Self {
        Self {
            source,
            index,
            length,
        }
    }

    pub const fn source(&self) -> SourceId {
        self.source
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub const fn length(&self) -> usize {
        self.length
    }

    pub const fn end(&self) -> usize {
        self.index + self.length
    }

    pub const fn null() -> Self {
        Self {
            source: SourceId::null(),
            index: 0,
            length: 0,
        }
    }
}

impl BitOr for Span {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let index = self.index.min(rhs.index);
        let end = self.end().max(rhs.end());
        let length = end - index;

        Self {
            source: self.source,
            index,
            length,
        }
    }
}

impl BitOrAssign for Span {
    fn bitor_assign(&mut self, rhs: Self) {
        let index = self.index.min(rhs.index);
        let end = self.end().max(rhs.end());
        let length = end - index;

        self.index = index;
        self.length = length;
    }
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl Spanned for Span {
    fn span(&self) -> Span {
        *self
    }
}
