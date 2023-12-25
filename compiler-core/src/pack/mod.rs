//! Packer (first steps of compiling a route)
//!
//! The packer takes a project from a resource context, processes the metadata,
//! and resolves any `use` property defined in the route or metadata.

use std::convert::Infallible;
use std::fmt::{Display, Formatter};

use crate::lang;
use crate::res::ResError;
use crate::types::DocDiagnostic;

mod pack_entry_points;
pub use pack_entry_points::*;
mod pack_project;
pub use pack_project::*;


#[derive(Debug, PartialEq, thiserror::Error)]
pub enum PackerError {
    #[error("Failed to load resource: {0}")]
    Res(#[from] ResError),

    #[error("Resource type is invalid: {0} should be of type {1}")]
    InvalidResourceType(String, String),

    #[error("Project metadata is missing a required property: {0}")]
    MissingMetadataProperty(String),

    #[error("Project property {0} has invalid type")]
    InvalidMetadataPropertyType(String),

    #[error("Project metadata has extra unused property: {0}")]
    UnusedMetadataProperty(String),


    #[error("Entry point `{0}` is invalid: `{1}` is neither an absolute path, nor a name of another entry point.")]
    InvalidEntryPoint(String, String),

    #[error("Entry point `{0}` is nesting too deep! Do you have a recursive loop?")]
    MaxEntryPointDepthExceeded(String),



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

