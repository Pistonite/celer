//! Implementations for extended utils of [`DocPoorText`]

use crate::types::{DocPoorText, DocPoorTextBlock};

impl DocPoorText {
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &DocPoorTextBlock> {
        self.0.iter()
    }
}

impl IntoIterator for DocPoorText {
    type Item = DocPoorTextBlock;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
