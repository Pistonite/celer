//! # Celer Compiler Core
//!
//! Implementation of the core compiler phases:
//!    - [`prep`] Prepare the project metadata (cachable)
//!    - [`pack`] Resolve all resources and packs them into a single object
//!    - [`comp`] Compile the route into sections and lines
//!    - [`exec`] Process the route data into a renderable object
//!    - [`expo`] Export artifacts from the route

pub mod plugin;

pub mod comp;
pub mod exec;
pub mod expo;
pub mod pack;
pub mod prep;

// public API re-exports
pub use comp::CompDoc;
pub use exec::{ExecContext, ExecDoc};
pub use expo::{ExpoContext, ExpoDoc, ExportRequest};
pub use pack::{CompileContext, Compiler};
pub use prep::{ContextBuilder, PreparedContext};

pub use celerb::*;
