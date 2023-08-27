//! # Packer
//!
//! The packer is the first step of compiling a route.
//! It takes a project from a resource context, processes the metadata,
//! and resolves any `use` property defined in the route or metadata.
//!
//! The output of the packer is a [`RouteMetadata`](celerctypes::RouteMetadata)
//! and a json blob of the route.

mod image;
mod pack_config;
pub use pack_config::*;
mod pack_map;
pub use pack_map::*;
mod pack_metadata;
mod pack_project;
pub use pack_project::*;
mod pack_use;
pub use pack_use::*;
mod packer;
pub use packer::*;
mod resource;
pub use resource::*;
use serde_json::Value;


#[derive(Debug, thiserror::Error)]
pub enum PackerError {
    #[error("Invalid `use` value: {0}")]
    InvalidUse(String),

    #[error("Error when parsing JSON: {0}")]
    InvalidJSON(#[from] serde_json::Error),

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

    #[error("Project config at index {0}: the `{1}` property is unused")]
    UnusedConfigProperty(usize, String),

    #[error("Project config at index {0}: defining map when a previous config already defines one")]
    DuplicateMap(usize),

    #[error("Image resource {0} has exceeded the size limit of {1}")]
    ImageTooBig(String, String),

    #[error("{0}")]
    NotImpl(String)
}

pub type PackerResult<T> = Result<T, PackerError>;

/// JSON value tagged with an error
///
/// This is used to expose errors to the compiler, so it can be displayed
/// using the diagnostics API
pub enum PackerValue {
    Ok(Value),
    Err(PackerError, Value),
}

