use std::collections::HashMap;

use celerctypes::{GameCoord, RouteMetadata};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::lang::Preset;

mod builder;
pub use builder::*;
mod comp_coord;
mod comp_line;
pub use comp_line::*;
mod comp_marker;
pub use comp_marker::*;
mod comp_movement;
pub use comp_movement::*;
mod comp_preset;
mod desugar;
use desugar::*;
mod prop;

#[derive(Derivative, Debug, Clone)]
#[derivative(Default)]
pub struct Compiler {
    project: RouteMetadata,
    presets: HashMap<String, Preset>,
    /// Current color of the map line
    color: String,
    /// Current position on the map
    coord: GameCoord,
    #[derivative(Default(value = "10"))]
    max_preset_depth: usize,
    default_icon_priority: i64,
}

pub type CompilerResult<T> = Result<T, (T, Vec<CompilerError>)>;

#[derive(PartialEq, Debug, Clone)]
pub enum CompilerError {
    /// When an array is specified as a line
    ArrayCannotBeLine,
    /// When an empty object is specified as a line
    EmptyObjectCannotBeLine,
    /// When a line object has more than 2 keys
    TooManyKeysInObjectLine,

    /// When line_object[key] is not an object
    ///
    /// For example:
    /// ```yaml
    /// - line: "red"
    /// ```
    LinePropertiesMustBeObject,

    /// When a line property type is invalid.
    ///
    /// Arg is property name or path
    InvalidLinePropertyType(String),

    /// When a preset string is malformed, like `foo` or `_foo::` or `_bar<foo`
    InvalidPresetString(String),
    /// When a preset is not found
    PresetNotFound(String),
    /// When presets recurse too much
    MaxPresetDepthExceeded(String),
    /// When an unexpected property is specified and not used by compiler
    UnusedProperty(String),
    /// When the counter property has rich text with more than one tag
    TooManyTagsInCounter,
    /// When the value specified as part of movement has invalid type
    InvalidMovementType,
    /// When the coordinate specified as part of movement is not an array
    InvalidCoordinateType(String),
    /// When the coordinate specified as part of movement has too few or too many elements
    InvalidCoordinateArray,
    /// When the coordinate value inside coordinate array is not valid
    InvalidCoordinateValue(String),
    /// When a preset specified as part of a movement does not contain the `movements` property
    InvalidMovementPreset(String),
    /// When the value specified as part of marker is invalid
    InvalidMarkerType,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompilerDiagnostic {
    pub file_name: String,
    pub line: usize,
    pub message: String,
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
