//! Resource resolving and loading
use serde_json::Value;

use crate::env::RefCounted;
use crate::macros::async_trait;
use crate::util;

mod path;
pub use path::*;
mod res_use;
pub use res_use::*;
mod res_type;
pub use res_type::*;
mod resolve;

pub mod test_utils;

/// Resource-related error types
#[derive(Debug, thiserror::Error)]
pub enum ResError {
    #[error("Invalid `use` value: `{0}`. If you are specifying a relative path, make sure to start with ./ or ../")]
    InvalidUse(String),

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

    #[error("Cannot resolve resource `{1}` from `{0}`.")]
    CannotResolve(String, String),

    #[error("Cannot load file `{0}`: {1}")]
    FailToLoadFile(String, String),

    #[error("Cannot load url `{0}`: {1}")]
    FailToLoadUrl(String, String),
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
    async fn load_raw(&self, path: &ResPath) -> ResResult<RefCounted<[u8]>>;
}

/// A Resource is an absolute reference to a resource that can be loaded.
/// It can be a local file or a remote URL. It also has an associated ref-counted
/// [`Loader`] that can be used to load the resource.
#[derive(Debug, Clone)]
pub struct Resource<'a, L>
where
    L: Loader,
{
    path: ResPath<'a>,
    loader: RefCounted<L>,
}

impl<'a, L> Resource<'a, L>
where
    L: Loader,
{
    /// Create a new resource
    pub fn new(path: ResPath<'a>, loader: RefCounted<L>) -> Self {
        Self { path, loader }
    }

    /// Create a new resource with the same loader as `self` but with a different path
    pub fn with_path(&self, path: ResPath<'a>) -> Self {
        Self {
            path,
            loader: RefCounted::clone(&self.loader),
        }
    }

    pub fn path(&self) -> &ResPath<'a> {
        &self.path
    }

    /// Load the resource as raw bytes
    pub async fn load_raw(&self) -> ResResult<RefCounted<[u8]>> {
        self.loader.load_raw(&self.path).await
    }

    /// Load the resource as UTF-8 string
    pub async fn load_utf8(&self) -> ResResult<String> {
        let bytes = self.loader.load_raw(&self.path).await?;
        match std::str::from_utf8(&bytes) {
            Ok(v) => Ok(v.to_string()),
            Err(_) => Err(ResError::InvalidUtf8(self.path.to_string())),
        }
    }

    /// Load the resource as structured value for supported formats (JSON, YAML)
    pub async fn load_structured(&self) -> ResResult<Value> {
        match self.path.get_type() {
            Some(ResType::Yaml) => {
                let bytes = self.loader.load_raw(&self.path).await?;
                if std::str::from_utf8(&bytes).is_err() {
                    return Err(ResError::InvalidUtf8(self.path.to_string()));
                }
                match serde_yaml::from_slice(&bytes) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ResError::InvalidYaml(self.path.to_string(), e)),
                }
            }
            Some(ResType::Json) => {
                let bytes = self.loader.load_raw(&self.path).await?;
                if std::str::from_utf8(&bytes).is_err() {
                    return Err(ResError::InvalidUtf8(self.path.to_string()));
                }
                match serde_json::from_slice(&bytes) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ResError::InvalidJson(self.path.to_string(), e)),
                }
            }
            _ => Err(ResError::UnknownDataFormat(self.path.to_string())),
        }
    }

    /// Load the image as either a remote URL or a data URL
    pub async fn load_image_url(&self) -> ResResult<String> {
        if !self.path.is_local() {
            // if path is a URL, just return it
            return Ok(self.path.to_string());
        }
        let image_type = self.path.get_type();
        let media_type = match image_type {
            Some(x) if x.is_image() => x.media_type(),
            _ => return Err(ResError::UnknownImageFormat(self.path.to_string())),
        };
        // load the bytes
        let bytes = self.loader.load_raw(&self.path).await?;
        // encode the bytes as a data url
        let data_url = util::to_data_url_base64(media_type, &bytes);

        Ok(data_url)
    }
}
