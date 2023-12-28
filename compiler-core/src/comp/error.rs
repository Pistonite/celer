use std::borrow::Cow;

use crate::{lang::IntoDiagnostic, plugin::PluginError};

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
    IsPreface,

    /// When a section is invalid
    #[error("Section data is not the correct type.")]
    InvalidSectionType,

    /// When the `route` property is invalid
    #[error("Route data is not the correct type.")]
    InvalidRouteType,

    #[error("Failed to run plugins before compile: {0}")]
    PluginBeforeCompileError(PluginError),

    #[error("Failed to run plugins before compile: {0}")]
    PluginAfterCompileError(PluginError),
}

pub type CompResult<T> = Result<T, CompError>;

impl IntoDiagnostic for CompError {
    fn source(&self) -> Cow<'static, str> {
        "celerc/comp".into()
    }

    fn is_error(&self) -> bool {
        match self {
            CompError::UnusedProperty(_) | CompError::TooManyTagsInCounter => false,
            _ => true,
        }
    }

    fn help_path(&self) -> Option<Cow<'static, str>> {
        let path = match self {
            CompError::MaxPresetDepthExceeded(_) => return None,
            CompError::PluginBeforeCompileError(_) => "plugin/getting-started",
            CompError::PluginAfterCompileError(_) => "plugin/getting-started",
            CompError::ArrayCannotBeLine
            | CompError::EmptyObjectCannotBeLine
            | CompError::TooManyKeysInObjectLine
            | CompError::LinePropertiesMustBeObject
            | CompError::UnusedProperty(_) => "route/text-and-notes#line-properties",

            CompError::InvalidLinePropertyType(_) => "route/property-reference",
            CompError::InvalidPresetString(_) => "route/using-presets",
            CompError::PresetNotFound(_) => "route/using-presets",
            CompError::TooManyTagsInCounter => "route/counter-and-splits",
            CompError::InvalidMovementType
            | CompError::InvalidCoordinateType(_)
            | CompError::InvalidCoordinateArray
            | CompError::InvalidCoordinateValue(_)
            | CompError::InvalidMovementPreset(_) => "route/customizing-movements",
            CompError::InvalidMarkerType => "route/customizing-movements#markers",
            CompError::IsPreface => "route/route-structure#preface",
            CompError::InvalidSectionType => "route/route-structure#sections",
            CompError::InvalidRouteType => "route/route-structure#entry-point",
        };
        Some(format!("/docs/{path}").into())
    }
}
