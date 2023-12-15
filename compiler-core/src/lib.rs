
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
//! 3. Low-level Util layers: (TODO) Low-level utilities:
//!    - [`lang`] Language utilities
//!    - [`prop`] Static property names
//!    - [`types`] Public types, especially for interop with WASM
//!    - [`json`] JSON utilities
//!    - [`plugin`] Plugin support
//!    - [`util`] Other utilities
//!    - [`resource`] Resource loading and resolving
//!    - [`macros`] Re-export of macros

mod api;         // depends on: all
pub use api::*;
//pub mod expo;  // export phase (todo)
pub mod exec;    // executor, depends on: comp
pub mod comp;
pub mod pack;

pub mod json;
pub mod lang;
pub mod plugin;
pub mod prop;
pub mod resource;
pub mod types;
pub mod util;

/// Re-exports of macros
pub mod macros {
    pub mod external {
        pub use async_recursion::async_recursion;
        pub use async_trait::async_trait;
    }
    pub use celercmacros::*;
}
