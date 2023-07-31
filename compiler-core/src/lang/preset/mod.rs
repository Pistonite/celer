mod grammar;
mod parse;
mod hydrate;
mod blob;

use std::collections::BTreeMap;

use serde_json::Value;
use serde::{Serialize, Deserialize};

use super::TempStr;

/// A preset is an arbitrary json object blob that can contain template strings
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Preset (BTreeMap<String, PresetBlob>);

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum PresetBlob {
    /// A sub-blob that does not contain any template strings
    NonTemplate(Value),
    /// A template string value
    Template(TempStr),
    /// Array value
    Array(Vec<PresetBlob>),
    /// Object value
    Object(BTreeMap<String, PresetBlob>),
}

/// Instantiation of a preset with the name and args
#[derive(Debug, PartialEq, Clone)]
pub struct PresetInst {
    /// Name of the preset, such as _Foo::Bar
    pub name: String,
    /// Arguments to the preset
    pub args: Vec<String>,
}