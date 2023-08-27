use std::collections::HashMap;

use celerctypes::RouteMetadata;
use comp::CompLine;
use serde::{Deserialize, Serialize};

mod comp;
mod exec;
mod json;
mod lang;
mod pack;
mod plug;
mod util;

use pack::Resource;
use lang::Preset;

pub async fn compile(project: &dyn Resource) -> CompDoc {
    todo!()
}

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
