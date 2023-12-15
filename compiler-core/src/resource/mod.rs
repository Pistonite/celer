//! Resource resolving and loading
use std::borrow::Cow;

use crate::macros::async_trait;

mod path;
pub use path::*;
mod res_use;
pub use res_use::*;
mod res_type;
pub use res_type::*;
mod resolve;
pub use resolve::*;

mod resource_impl;
pub use resource_impl::*;

#[cfg(test)]
mod test_utils;

/// Resource-related error types
#[derive(Debug, thiserror::Error)]
pub enum ResError {
    #[error("Resource is not valid UTF-8: {0}")]
    InvalidUtf8(String),

    #[error("Error when parsing JSON resource `{0}`: {1}")]
    InvalidJson(String, serde_json::Error),

    #[error("Error when parsing YAML resource `{0}`: {1}")]
    InvalidYaml(String, serde_yaml::Error),

    #[error("Cannot determine the data format for `{0}`.")]
    UnknownDataFormat(String),

    #[error("Cannot determine the image format for `{0}`.")]
    UnknownImageFormat(String),

    #[error("Cannot resolve `{0}` from `{1}`.")]
    CannotResolve(String, String),
}

impl PartialEq for ResError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InvalidUtf8(a), Self::InvalidUtf8(b)) => a == b,
            (Self::InvalidJson(a, _), Self::InvalidJson(b, _)) => a == b,
            (Self::InvalidYaml(a, _), Self::InvalidYaml(b, _)) => a == b,
            (Self::UnknownDataFormat(a), Self::UnknownDataFormat(b)) => a == b,
            (Self::UnknownImageFormat(a), Self::UnknownImageFormat(b)) => a == b,
            (Self::CannotResolve(a, b), Self::CannotResolve(c, d)) => a == c && b == d,
            _ => false,
        }
    }
}

pub type ResResult<T> = Result<T, ResError>;

/// Loader that loads resources from external place
#[async_trait(auto)]
pub trait Loader {
    /// Load a resource as raw bytes
    async fn load_raw<'s>(&'s self, path: &ResPath<'_, '_>) -> ResResult<Cow<'s, [u8]>>;
}
