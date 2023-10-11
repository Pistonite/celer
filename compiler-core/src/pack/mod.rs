//! # Packer
//!
//! The packer is the first step of compiling a route.
//! It takes a project from a resource context, processes the metadata,
//! and resolves any `use` property defined in the route or metadata.
//!
//! The output of the packer is a [`RouteMetadata`](celerctypes::RouteMetadata)
//! and a json blob of the route.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::convert::Infallible;

use celerctypes::DocDiagnostic;
mod pack_config;
pub use pack_config::*;
mod pack_coord_map;
pub use pack_coord_map::*;
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

use crate::json::Cast;
use crate::lang::parse_poor;
use crate::macros::{async_recursion, maybe_send};

#[derive(Debug, Clone, PartialEq, thiserror::Error, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
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

    #[error("Project config at index {0} has an invalid type")]
    InvalidConfigType(usize),

    #[error("Project config at index {0}: the `{1}` property is invalid")]
    InvalidConfigProperty(usize, String),

    #[error("Project config at index {0}: the required `{1}` property is missing")]
    MissingConfigProperty(usize, String),

    #[error("Project config at index {0}: the `{1}` property is unused")]
    UnusedConfigProperty(usize, String),

    #[error("Project config at index {0}: The preset {1} is invalid")]
    InvalidPreset(usize, String),

    #[error(
        "Project config at index {0}: defining map when a previous config already defines one"
    )]
    DuplicateMap(usize),

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
            msg: parse_poor(&self.to_string()),
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


pub enum ImageFormat {
    PNG,
    JPEG,
    GIF,
    WEBP,
}

impl ImageFormat {
    pub fn try_from_path(path: &str) -> Option<Self> {
        let path = path.to_lowercase();
        if path.ends_with(".png") {
            Some(Self::PNG)
        } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
            Some(Self::JPEG)
        } else if path.ends_with(".gif") {
            Some(Self::GIF)
        } else if path.ends_with(".webp") {
            Some(Self::WEBP)
        } else {
            None
        }
    }

    pub fn media_type(&self) -> &'static str {
        match self {
            Self::PNG => "image/png",
            Self::JPEG => "image/jpeg",
            Self::GIF => "image/gif",
            Self::WEBP => "image/webp",
        }
    }
}
