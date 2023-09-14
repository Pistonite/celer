use std::sync::Arc;

use serde_json::Value;

use crate::pack::{PackerResult, PackerError, ValidUse};
use crate::util::Path;

use super::ResourceLoader;

#[cfg(not(feature = "wasm"))]
pub type ArcLoader<R> = Arc<dyn ResourceLoader<Ref=R> + Send + Sync>;
#[cfg(not(feature = "wasm"))]
pub type ArcResolver = Arc<dyn ResourceResolver + Send + Sync>;
#[cfg(feature = "wasm")]
pub type ArcLoader = Arc<dyn ResourceLoader>;
#[cfg(feature = "wasm")]
pub type ArcResolver = Arc<dyn ResourceResolver>;

macro_rules! loader_delegate {
    ($func:ident, $type:ty) => {
        #[doc = "Macro-generated loader delegate. See [`ResourceLoader`]"]
        pub async fn $func(&self) -> PackerResult<$type> {
            match &self.path {
                ResourcePath::Url(url) => self.url_loader.$func(url).await,
                ResourcePath::FsPath(path) => self.fs_loader.$func(path.as_ref()).await,
            }
        }
    }
}

/// A resource contains:
/// - an API to load the resource
/// - an API to resolve another resource from its location
#[derive(Clone)]
pub struct Resource {
    path: ResourcePath,
    fs_loader: ArcLoader,
    url_loader: ArcLoader,
    resolver: ArcResolver,
}

impl Resource {
    pub fn new(
        path: ResourcePath,
        fs_loader: ArcLoader,
        url_loader: ArcLoader,
        resolver: ArcResolver,
    ) -> Self {
        Self {
            path,
            fs_loader,
            url_loader,
            resolver,
        }
    }

    /// Create a new resource with the same loader
    pub fn create(&self, path: ResourcePath, resolver: ArcResolver) -> Self {
        Self {
            path,
            fs_loader: self.fs_loader.clone(),
            url_loader: self.url_loader.clone(),
            resolver,
        }
    }

    /// File path or URL
    pub fn name(&self) -> &str {
        match &self.path {
            ResourcePath::Url(url) => &url,
            ResourcePath::FsPath(path) => path.as_ref(),
        }
    }

    loader_delegate!(load_structured, Value);
    loader_delegate!(load_image_url, String);

    // /// Load resource as image URL
    // ///
    // /// Return value can be a data URL
    // pub async fn load_image_url(&self) -> PackerResult<String> {
    //     match &self.path {
    //         ResourcePath::Url(url) => Ok(url.to_string()),
    //         ResourcePath::FsPath(path) => Err(PackerError::NotImpl(
    //             "Local image is not implemented yet.".to_string(),
    //         )),
    //     }
    // }

    pub async fn resolve(&self, target: &ValidUse) -> PackerResult<Resource> {
        self.resolver.resolve(&self, target).await
    }
}

#[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
#[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
pub trait ResourceResolver {
    /// Resolve a resource from the given `Use` and loader
    async fn resolve(&self, source: &Resource, target: &ValidUse) -> PackerResult<Resource>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourcePath {
    Url(String),
    FsPath(Path),
}

