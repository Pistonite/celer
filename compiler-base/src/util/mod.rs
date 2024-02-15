//! Utilities

mod string_map;
pub use string_map::*;
mod xml_escape;
pub use xml_escape::*;

// re-exports
pub use uni_path::{Component, Path, PathBuf};
