use std::collections::HashMap;

use celerctypes::{RouteMetadata, GameCoord};
use serde::{Deserialize, Serialize};

use crate::{lang::Preset, CompLine};

mod builder;
pub use builder::*;
mod comp_line;

#[derive(Default, Debug, Clone)]
pub struct Compiler {
    project: RouteMetadata,
    presets: HashMap<String, Preset>,
    /// Current color of the map line
    color: String,
    /// Current position on the map
    coord: GameCoord,
}

pub type CompilerResult<T> = Result<T, (T, Vec<CompilerError>)>;

#[derive(PartialEq, Debug, Clone)]
pub enum CompilerError {
    ArrayCannotBeLine,
    EmptyObjectCannotBeLine,
    TooManyKeysInObjectLine,
    LinePropertiesMustBeObject,
    InvalidLinePropertyType(String),
    InvalidPresetString(String),
    PresetNotFound(String),
    MaxPresetDepthExceeded(String),
    UnusedProperty(String),
    TooManyTagsInCounter,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompilerDiagnostic {
    pub file_name: String,
    pub line: usize,
    pub message: String,
}
