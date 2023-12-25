
//! # Celer Compiler Core
//!
//! Implementation of the core compiler phases:
//!    - [`prep`] Prepare the project metadata (cachable)
//!    - [`pack`] Resolve all resources and packs them into a single object
//!    - [`comp`] Compile the route into sections and lines
//!    - [`exec`] Process the route data into a renderable object
//!    - [`expo`] Export artifacts from the route

mod api;
pub use api::*;

pub mod plugin;

pub mod prep;
pub mod pack;
pub mod comp;
pub mod exec;    // executor, depends on: comp
//pub mod expo;  // export phase (todo)

// public API re-exports
pub use prep::{ContextBuilder, PreparedContext};

pub mod types;

pub use celerb::*;
