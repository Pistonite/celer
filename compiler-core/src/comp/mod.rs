//! Compiler core logic
//!
//! The compiler takes in the raw route JSON blob and extracts the known properties
//! into native structures. It also computes temporal properties like the current coordinates
//! and color at any given point in the route.

use std::borrow::Cow;
use std::convert::Infallible;

use derivative::Derivative;
use instant::Instant;
use serde_json::Value;

use crate::lang::parse_poor;
use crate::types::{DocDiagnostic};
use crate::env;

mod comp_coord;
pub use comp_coord::*;
mod comp_doc;
pub use comp_doc::*;
mod comp_line;
pub use comp_line::*;
mod comp_marker;
pub use comp_marker::*;
mod comp_movement;
pub use comp_movement::*;
mod comp_preset;
pub use comp_preset::*;
mod comp_section;
pub use comp_section::*;
mod desugar;
use desugar::*;

pub type CompilerResult<T> = Result<T, (T, Vec<CompError>)>;

/// Error in comp phase, related to resolving properties in the route
///
/// These errors don't cause the compilation to abort, but are displayed in the route itself
/// through the diagnostics API.
#[derive(PartialEq, Debug, thiserror::Error)]
pub enum CompError {
    /// When an array is specified as a line
    #[error("A line cannot be an array. Check the formatting of your route.")]
    ArrayCannotBeLine,

    /// When an empty object is specified as a line
    #[error("A line cannot be an empty object.")]
    EmptyObjectCannotBeLine,

    /// When a line object has more than 2 keys
    #[error("Multiple keys for a line found. Did you forget to indent the properties?")]
    TooManyKeysInObjectLine,

    /// When line_object[key] is not an object
    ///
    /// For example:
    /// ```yaml
    /// - line: "red"
    /// ```
    #[error("Line properties must be a mapping. Did you accidentally put a property in the wrong place?")]
    LinePropertiesMustBeObject,

    /// When a line property type is invalid.
    ///
    /// Arg is property name or path
    #[error("Line property `{0}` has invalid type")]
    InvalidLinePropertyType(String),

    /// When a preset string is malformed, like `foo` or `_foo::` or `_bar<foo`
    #[error("Preset string `{0}` is malformed")]
    InvalidPresetString(String),

    /// When a preset is not found
    #[error("Preset `{0}` is not found")]
    PresetNotFound(String),

    /// When presets recurse too much
    #[error("Maximum preset depth exceeded when processing the preset `{0}`. Did you have circular references in your presets?")]
    MaxPresetDepthExceeded(String),

    /// When an unexpected property is specified and not used by compiler
    #[error("Property `{0}` is unused. Did you misspell it?")]
    UnusedProperty(String),

    /// When the counter property has rich text with more than one tag
    #[error("Counter property can only have 1 tag.")]
    TooManyTagsInCounter,

    /// When the value specified as part of movement has invalid type
    #[error("Some of the movements specified cannot be processed.")]
    InvalidMovementType,

    /// When the coordinate specified as part of movement is not an array
    #[error("The coordinate specified by `{0}` is not an array.")]
    InvalidCoordinateType(String),

    /// When the coordinate specified as part of movement has too few or too many elements
    #[error("Some of the coordinates specified may not be valid. Coordinates must have either 2 or 3 components.")]
    InvalidCoordinateArray,

    /// When the coordinate value inside coordinate array is not valid
    #[error("`{0}` is not a valid coordinate value.")]
    InvalidCoordinateValue(String),

    /// When a preset specified as part of a movement does not contain the `movements` property
    #[error("Preset `{0}` cannot be used inside hte `movements` property because it does not contain any movement.")]
    InvalidMovementPreset(String),

    /// When the value specified as part of marker is invalid
    #[error("Some of the markers specified cannot be processed.")]
    InvalidMarkerType,

    /// When a section is a preface.
    ///
    /// This may not be an actual error depending on if the compiler is expecting a preface
    #[error("Preface can only be in the beginning of the route.")]
    IsPreface(Value),

    /// When a section is invalid
    #[error("Section data is not the correct type.")]
    InvalidSectionType,

    /// When the `route` property is invalid
    #[error("Route data is not the correct type.")]
    InvalidRouteType,
}

impl CompError {
    /// Get the more info url path for a compiler error
    ///
    /// The returned path is relative to the site origin (i.e. /docs/...)
    pub fn get_info_url_path(&self) -> &'static str {
        match self {
            CompError::ArrayCannotBeLine
            | CompError::EmptyObjectCannotBeLine
            | CompError::TooManyKeysInObjectLine
            | CompError::LinePropertiesMustBeObject
            | CompError::InvalidLinePropertyType(_)
            | CompError::UnusedProperty(_) => "/docs/route/customizing-lines",
            CompError::InvalidPresetString(_)
            | CompError::PresetNotFound(_)
            | CompError::MaxPresetDepthExceeded(_) => "/docs/route/using-presets",
            CompError::TooManyTagsInCounter => "/docs/route/customizing-lines#counter",
            CompError::InvalidCoordinateType(_)
            | CompError::InvalidCoordinateArray
            | CompError::InvalidCoordinateValue(_)
            | CompError::InvalidMovementType => "/docs/route/customizing-movements",
            CompError::InvalidMovementPreset(_) => "/docs/route/customizing-movements#presets",
            CompError::InvalidMarkerType => "/docs/route/customizing-lines#markers",
            CompError::IsPreface(_) => "/docs/route/route-structure#preface",
            // CompError::PackerErrors(_) | 
                CompError::InvalidSectionType => {
                "/docs/route/route-structure"
            }
            CompError::InvalidRouteType => "/docs/route/route-structure#entry-point",
        }
    }

    pub fn get_type(&self) -> String {
        let s = match self {
            CompError::ArrayCannotBeLine
            | CompError::EmptyObjectCannotBeLine
            | CompError::TooManyKeysInObjectLine
            | CompError::LinePropertiesMustBeObject
            | CompError::InvalidLinePropertyType(_)
            | CompError::InvalidPresetString(_)
            | CompError::PresetNotFound(_)
            | CompError::MaxPresetDepthExceeded(_)
            | CompError::InvalidMovementType
            | CompError::InvalidCoordinateType(_)
            | CompError::InvalidCoordinateArray
            | CompError::InvalidCoordinateValue(_)
            | CompError::InvalidMovementPreset(_)
            | CompError::InvalidMarkerType
            | CompError::IsPreface(_)
            | CompError::InvalidSectionType
            // | CompError::PackerErrors(_)
            | CompError::InvalidRouteType => "error",

            CompError::UnusedProperty(_) | CompError::TooManyTagsInCounter => "warn",
        };

        s.to_string()
    }

    pub fn add_to_diagnostics(&self, output: &mut Vec<DocDiagnostic>) {
        match self {
            // CompError::PackerErrors(errors) => {
            //     for error in errors {
            //         error.add_to_diagnostics(output);
            //     }
            // }
            other_error => {
                let site_origin = env::get_site_origin();
                let help_url_path = other_error.get_info_url_path();
                let msg = format!("{other_error} See {site_origin}{help_url_path} for more info.");

                output.push(DocDiagnostic {
                    msg: parse_poor(&msg),
                    msg_type: other_error.get_type(),
                    source: "celerc/compiler".to_string(),
                });
            }
        }
    }
}

/// Convenience macro for validating a json value and add error
macro_rules! validate_not_array_or_object {
    ($value:expr, $errors:ident, $property:expr) => {{
        let v = $value;
        if v.is_array() || v.is_object() {
            $errors.push(CompError::InvalidLinePropertyType($property));
            false
        } else {
            true
        }
    }};
}
pub(crate) use validate_not_array_or_object;

#[cfg(test)]
mod test_utils;
