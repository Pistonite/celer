
//! # Celer Compiler Core
//!
//! This is the core crate for celer compiler that is used for both
//! Web Editor (wasm) and Web Server.
//!
//! It has 3 logical layers:
//! 1. API layer [`api`]: Top-level, public API for the interacting with the compiler workflows.
//! 2. Compilation phase layers: (TODO) Implementation for different phases of the compiler:
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

pub mod types;

pub use celerb::*;
