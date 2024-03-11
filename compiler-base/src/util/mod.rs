//! Utilities

mod string_map;
pub use string_map::*;
mod escape;
pub use escape::*;
mod data_url;
pub use data_url::*;

// re-exports
pub use uni_path::{Component, Path, PathBuf};
