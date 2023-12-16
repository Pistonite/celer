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
pub use entry_point::*;

use crate::types::EntryPoints;

#[derive(Debug, thiserror::Error)]
pub enum PrepError {
    #[error("Failed to load resource: {0}")]
    Res(#[from] ResError),
}

pub type PrepResult<T> = Result<T, PrepError>;

pub async fn get_entry_points<L>(project_resource: &Resource<'_, L>) -> PrepResult<EntryPoints> 
where L: Loader {
    todo!()
}


// pub struct PreparedContext
