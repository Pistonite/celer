//! Preset parsing, hydration and pre-compile optimization

use crate::json::SafeRouteBlob;

use super::TempStr;

mod blob;
mod grammar;
mod hydrate;
pub use hydrate::*;
mod optimize;
mod parse;

/// A preset is an arbitrary json object blob that can contain template strings
#[derive(PartialEq, Debug, Clone)]
pub struct Preset(Vec<(TempStr, PresetBlob)>);

#[derive(PartialEq, Debug, Clone)]
pub enum PresetBlob {
    /// A sub-blob that does not contain any template strings
    NonTemplate(SafeRouteBlob<'static>),
    /// A template string value
    Template(TempStr),
    /// Array value
    Array(Vec<PresetBlob>),
    /// Object value
    Object(Vec<(TempStr, PresetBlob)>),
}

/// Instantiation of a preset with the name and args
#[derive(Debug, PartialEq, Clone)]
pub struct PresetInst {
    /// Name of the preset, such as _Foo::Bar
    pub name: String,
    /// Arguments to the preset
    pub args: Vec<String>,
}
