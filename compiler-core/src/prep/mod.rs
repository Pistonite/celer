//! Prepare (prep) phase
//!
//! # Input
//! This is the 0-th phase of the compiler that prepares the metadata for the compilation.
//! It is also responsible for inspecting the project properties such as entry points and
//! title/version.
//!
//! It takes (from the outside) a [`Resource`](crate::res::Resource) that is the root project,
//! as well as the entry point
//!
//! # Work
//! 1. Loading the entry point config (project.yaml). If the entry point contains redirection
//!    through `entrypoints` property, it finds the correct entry point config to load.
//! 2. Build the configuration object
//! 3. Optimize configuration and compile plugins to be cached
//!
//! # Output
//! The output of this phase is a [`PreparedContext`] object that can be used to create
//! the compiler with additional (and optional) plugins.

mod entry_point;
use std::borrow::Cow;

pub use entry_point::*;
mod config;
pub use config::*;

use crate::types::EntryPoints;
use crate::res::{ResError, Loader, Resource};

#[derive(Debug, thiserror::Error)]
pub enum PrepError {
    #[error("Failed to load resource: {0}")]
    Res(#[from] ResError),

    #[error("Project config ({0}): property `{1}` has an invalid type (expected {2})")]
    InvalidConfigPropertyType(ConfigTrace, Cow<'static, str>, &'static str),

    #[error("Project config ({0}): cannot find tag `{1}`")]
    TagNotFound(ConfigTrace, String),
}

pub type PrepResult<T> = Result<T, PrepError>;

pub async fn get_entry_points<L>(project_resource: &Resource<'_, L>) -> PrepResult<EntryPoints> 
where L: Loader {
    todo!()
}


pub struct PreparedContext {
    pub config: RouteConfig,
    pub meta: CompilerMetadata,
    pub route: Value,
}
