//! Execution of the compiled document
//!
//! This is the last stage of the compilation process.
//! The [`CompDoc`] will be transformed into a [`ExecDoc`]
//! for rendering

mod error;
pub use error::*;
mod exec_line;
pub use exec_line::*;
mod exec_map;
pub use exec_map::*;
mod map;
pub use map::*;
mod exec_section;
pub use exec_section::*;
mod exec_doc;
pub use exec_doc::*;
