use serde::{Serialize, Deserialize};

use crate::macros::derive_wasm;
use crate::lang::DocRichText;

/// Document note block
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
#[serde(tag = "type")]
pub enum DocNote {
    Text { content: DocRichText },
    Image { link: String },
    Video { link: String },
}


