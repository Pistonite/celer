//! Implementations for extended utils of [`DocPoorText`]

use serde::{Deserialize, Serialize};

use crate::macros::derive_wasm;

/// Document poor text
///
/// This is a collection of [`DocPoorTextBlock`]s
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct DocPoorText(pub Vec<DocPoorTextBlock>);

/// Document poor text block. Just text or link
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
#[serde(tag = "type", content = "data")]
pub enum DocPoorTextBlock {
    Text(String),
    Link(String),
}

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
