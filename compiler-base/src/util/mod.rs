//! Utilities

mod string_map;
pub use string_map::*;
mod escape;
pub use escape::*;

// re-exports
pub use uni_path::{Component, Path, PathBuf};
