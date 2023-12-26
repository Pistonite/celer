use serde::{Deserialize, Serialize};

use crate::lang::DocRichText;
use crate::macros::derive_wasm;

/// Document note block
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
#[serde(tag = "type")]
pub enum DocNote {
    Text { content: DocRichText },
    Image { link: String },
    Video { link: String },
}
