//! Packer (first steps of compiling a route)
//!
//! The packer takes a project from a resource context, processes the metadata,
//! and resolves any `use` property defined in the route or metadata.

use std::convert::Infallible;
use std::fmt::{Display, Formatter};

use crate::lang;
use crate::types::DocDiagnostic;

mod pack_config;
pub use pack_config::*;
mod pack_coord_map;
pub use pack_coord_map::*;
mod pack_entry_points;
pub use pack_entry_points::*;
mod pack_map;
pub use pack_map::*;
mod pack_map_layer;
pub use pack_map_layer::*;
mod pack_preset;
pub use pack_preset::*;
mod pack_project;
pub use pack_project::*;
mod pack_route;
pub use pack_route::*;
mod pack_use;
pub use pack_use::*;
mod pack_value;
pub use pack_value::*;
mod resource;
pub use resource::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConfigTrace(Vec<usize>);

impl ConfigTrace {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    #[inline]
    pub fn push(&mut self, v: usize) {
        self.0.push(v)
    }
    #[inline]
    pub fn pop(&mut self) -> Option<usize> {
        self.0.pop()
    }
}

impl Display for ConfigTrace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "trace=[{}]",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl From<&[usize]> for ConfigTrace {
    fn from(v: &[usize]) -> Self {
        Self(v.to_vec())
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum PackerError {
    #[error("The project file (project.yaml) is missing or invalid.")]
    InvalidProject,

    #[error("Invalid `use` value: {0}. If you are specifying a relative path, make sure to start with ./ or ../")]
    InvalidUse(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid url: {0}")]
    InvalidUrl(String),

    #[error("Max depth of {0} levels of `use` is reached. Please make sure there are no circular dependencies.")]
    MaxUseDepthExceeded(usize),

    #[error("Max reference depth of {0} levels is reached. There might be a formatting error in your project files.")]
    MaxRefDepthExceeded(usize),

    #[error("Max preset namespace depth of {0} levels is reached. There might be a formatting error in your project files. If this is intentional, consider making the namespaces less complex.")]
    MaxPresetNamespaceDepthExceeded(usize),

    #[error("The format of resource {0} cannot be determined")]
    UnknownFormat(String),

    #[error("Cannot load file: {0}")]
    LoadFile(String),

    #[error("Cannot load url: {0}")]
    LoadUrl(String),

    #[error("Error when parsing structured data in file {0}: {1}")]
    InvalidFormat(String, String),

    #[error("Error when parsing file {0}: file is not UTF-8")]
    InvalidUtf8(String),

    #[error("")]
    InvalidIcon,

    #[error("Resource type is invalid: {0} should be of type {1}")]
    InvalidResourceType(String, String),

    #[error("Project metadata is missing a required property: {0}")]
    MissingMetadataProperty(String),

    #[error("Project property {0} has invalid type")]
    InvalidMetadataPropertyType(String),

    #[error("Project metadata has extra unused property: {0}")]
    UnusedMetadataProperty(String),

    #[error("Project config ({0}) has an invalid type")]
    InvalidConfigType(ConfigTrace),

    #[error("Project config ({0}): the `{1}` property is invalid")]
    InvalidConfigProperty(ConfigTrace, String),

    #[error("Project config ({0}): the required `{1}` property is missing")]
    MissingConfigProperty(ConfigTrace, String),

    #[error("Project config ({0}): the `{1}` property is unused")]
    UnusedConfigProperty(ConfigTrace, String),

    #[error("Project config ({0}): The preset {1} is invalid")]
    InvalidPreset(ConfigTrace, String),

    #[error("Project config ({0}): defining map when a previous config already defines one")]
    DuplicateMap(ConfigTrace),

    #[error("Project config ({0}): config is nesting too deep!")]
    MaxConfigDepthExceeded(ConfigTrace),

    #[error("Project config ({0}): the tag `{1}` is not defined")]
    TagNotFound(ConfigTrace, String),

    #[error("Entry point `{0}` is invalid: `{1}` is neither an absolute path, nor a name of another entry point.")]
    InvalidEntryPoint(String, String),

    #[error("Entry point `{0}` is nesting too deep! Do you have a recursive loop?")]
    MaxEntryPointDepthExceeded(String),

    #[error("`{0}` is not a valid built-in plugin or reference to a plugin script")]
    InvalidPlugin(String),

    #[error("No map defined in project config")]
    MissingMap,

    #[error("Image resource {0} has exceeded the size limit of {1}")]
    ImageTooBig(String, String),

    #[error("{0}")]
    NotImpl(String),
}

impl PackerError {
    pub fn add_to_diagnostics(&self, output: &mut Vec<DocDiagnostic>) {
        output.push(DocDiagnostic {
            msg: lang::parse_poor(&self.to_string()),
            msg_type: "error".to_string(),
            source: "celerc/packer".to_string(),
        });
    }

    pub fn is_cancel(&self) -> bool {
        false
    }
}

impl From<Infallible> for PackerError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

pub type PackerResult<T> = Result<T, PackerError>;

