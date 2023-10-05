use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::comp::CompDoc;

mod link;
mod operation;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginRuntime {
    plugin: Plugin,
    props: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Plugin {
    BuiltIn(BuiltInPlugin),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum BuiltInPlugin {
    /// Transform link tags to clickable links. See [`link`]
    Link
}

pub fn run_plugins(comp_doc: CompDoc) -> CompDoc {
    // currently just a pass-through
    comp_doc
}

