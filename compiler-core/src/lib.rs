use std::collections::HashMap;

use comp::CompLine;
use serde::{Serialize, Deserialize};
use celerctypes::RouteMetadata;

mod exec;
mod lang;
mod comp;
mod json;

use lang::Preset;

#[derive(Default, Debug, Clone)]
pub struct CompilerContext {
    pub presets: HashMap<String, Preset>,
}

/// Compiled Document
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompDoc {
    /// Project metadata
    project: RouteMetadata,
    // TODO: compiler info
    route: Vec<CompSection>,
}

/// Compiled Section
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompSection {
    /// Name of the section
    name: String,
    /// The lines in the section
    lines: Vec<CompLine>,
}

const DEFAULT_LINE_COLOR: &str = "#38f";
const DEFAULT_MARKER_COLOR: &str = "#f00";

