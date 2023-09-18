use std::collections::HashMap;

use celerctypes::{DocDiagnostic, GameCoord, RouteMetadata};
use derivative::Derivative;
use serde_json::Value;

use crate::{
    lang::{parse_poor, Preset},
    pack::{PackerError, PackerValue}, util::WasmError,
};

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
mod compiler;
pub use compiler::*;
mod desugar;
use desugar::*;
pub mod prop;

pub type CompilerResult<T> = Result<T, (T, Vec<CompilerError>)>;

#[derive(PartialEq, Debug, Clone, thiserror::Error)]
pub enum CompilerError {
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
    #[error("Line property {0} has invalid type")]
    InvalidLinePropertyType(String),

    /// When a preset string is malformed, like `foo` or `_foo::` or `_bar<foo`
    #[error("Preset string {0} is malformed")]
    InvalidPresetString(String),

    /// When a preset is not found
    #[error("Preset {0} is not found")]
    PresetNotFound(String),

    /// When presets recurse too much
    #[error("Maximum preset depth exceeded when processing the preset {0}. Did you have circular references in your presets?")]
    MaxPresetDepthExceeded(String),

    /// When an unexpected property is specified and not used by compiler
    #[error("Property {0} is unused. Did you misspell it?")]
    UnusedProperty(String),

    /// When the counter property has rich text with more than one tag
    #[error("Counter property can only have 1 tag.")]
    TooManyTagsInCounter,

    /// When the value specified as part of movement has invalid type
    #[error("Some of the movements specified cannot be processed.")]
    InvalidMovementType,

    /// When the coordinate specified as part of movement is not an array
    #[error("The coordinate specified by {0} is not an array.")]
    InvalidCoordinateType(String),

    /// When the coordinate specified as part of movement has too few or too many elements
    #[error("Some of the coordinates specified may not be valid. Coordinates must have either 2 or 3 components.")]
    InvalidCoordinateArray,

    /// When the coordinate value inside coordinate array is not valid
    #[error("{0} is not a valid coordinate value.")]
    InvalidCoordinateValue(String),

    /// When a preset specified as part of a movement does not contain the `movements` property
    #[error("Preset {0} cannot be used inside hte `movements` property because it does not contain any movement.")]
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

    /// When packer errors need to be propagated
    #[error("Packer errors")]
    PackerErrors(Vec<PackerError>),

    /// When the `route` property is invalid
    #[error("Route data is not the correct type.")]
    InvalidRouteType,

    #[cfg(feature = "wasm")]
    #[error("Wasm execution error: {0}")]
    Wasm(#[from] WasmError),
}

impl CompilerError {
    /// Get the more info url for a compiler error
    pub fn get_info_url(&self) -> String {
        match self {
            CompilerError::ArrayCannotBeLine
            | CompilerError::EmptyObjectCannotBeLine
            | CompilerError::TooManyKeysInObjectLine
            | CompilerError::LinePropertiesMustBeObject
            | CompilerError::InvalidLinePropertyType(_)
            | CompilerError::UnusedProperty(_) => {
                "https://celer.pistonite.org/docs/route/customizing-lines".to_string()
            }
            CompilerError::InvalidPresetString(_)
            | CompilerError::PresetNotFound(_)
            | CompilerError::MaxPresetDepthExceeded(_) => {
                "https://celer.pistonite.org/docs/route/using-presets".to_string()
            }
            CompilerError::TooManyTagsInCounter => {
                "https://celer.pistonite.org/docs/route/customizing-lines#counter".to_string()
            }
            CompilerError::InvalidCoordinateType(_)
            | CompilerError::InvalidCoordinateArray
            | CompilerError::InvalidCoordinateValue(_)
            | CompilerError::InvalidMovementType => {
                "https://celer.pistonite.org/docs/route/customizing-movements".to_string()
            }
            CompilerError::InvalidMovementPreset(_) => {
                "https://celer.pistonite.org/docs/route/customizing-movements#presets".to_string()
            }
            CompilerError::InvalidMarkerType => {
                "https://celer.pistonite.org/docs/route/customizing-lines#markers".to_string()
            }
            CompilerError::IsPreface(_) => {
                "https://celer.pistonite.org/docs/route/route-structure#preface".to_string()
            }
            CompilerError::PackerErrors(_) | CompilerError::InvalidSectionType => {
                "https://celer.pistonite.org/docs/route/route-structure".to_string()
            }
            CompilerError::InvalidRouteType => {
                "https://celer.pistonite.org/docs/route/route-structure#entry-point".to_string()
            }
            CompilerError::Wasm(_) => "".to_string(),
        }
    }

    pub fn get_type(&self) -> String {
        let s = match self {
            CompilerError::ArrayCannotBeLine
            | CompilerError::EmptyObjectCannotBeLine
            | CompilerError::TooManyKeysInObjectLine
            | CompilerError::LinePropertiesMustBeObject
            | CompilerError::InvalidLinePropertyType(_)
            | CompilerError::InvalidPresetString(_)
            | CompilerError::PresetNotFound(_)
            | CompilerError::MaxPresetDepthExceeded(_)
            | CompilerError::InvalidMovementType
            | CompilerError::InvalidCoordinateType(_)
            | CompilerError::InvalidCoordinateArray
            | CompilerError::InvalidCoordinateValue(_)
            | CompilerError::InvalidMovementPreset(_)
            | CompilerError::InvalidMarkerType
            | CompilerError::IsPreface(_)
            | CompilerError::InvalidSectionType
            | CompilerError::PackerErrors(_)
            | CompilerError::Wasm(_)
            | CompilerError::InvalidRouteType => "error",

            CompilerError::UnusedProperty(_) | CompilerError::TooManyTagsInCounter => "warn",
        };

        s.to_string()
    }

    pub fn add_to_diagnostics(&self, output: &mut Vec<DocDiagnostic>) {
        match self {
            CompilerError::PackerErrors(errors) => {
                for error in errors {
                    error.add_to_diagnostics(output);
                }
            }
            other => {
                let msg = format!(
                    "{} See {} for more info.",
                    other.to_string(),
                    other.get_info_url()
                );

                output.push(DocDiagnostic {
                    msg: parse_poor(&msg),
                    msg_type: other.get_type(),
                    source: "celerc/compiler".to_string(),
                });
            }
        }
    }

    pub fn is_cancel(&self) -> bool {
        #[cfg(feature = "wasm")]
        let x = matches!(self, Self::Wasm(WasmError::Cancel));
        #[cfg(not(feature = "wasm"))]
        let x = false;
        x
    }
}


/// Convenience macro for validating a json value and add error
macro_rules! validate_not_array_or_object {
    ($value:expr, $errors:ident, $property:expr) => {{
        let v = $value;
        if v.is_array() || v.is_object() {
            $errors.push(CompilerError::InvalidLinePropertyType($property));
            false
        } else {
            true
        }
    }};
}
pub(crate) use validate_not_array_or_object;

#[cfg(test)]
mod test_utils {
    use celerctypes::{Axis, MapCoordMap, MapMetadata};

    use super::*;

    pub fn create_test_compiler_with_coord_transform() -> Compiler {
        let project = RouteMetadata {
            map: MapMetadata {
                coord_map: MapCoordMap {
                    mapping_3d: (Axis::X, Axis::Y, Axis::Z),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let builder = CompilerBuilder::new(project, Default::default(), Default::default());
        builder.build()
    }
}
