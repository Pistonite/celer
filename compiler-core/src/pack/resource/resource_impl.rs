use serde_json::Value;

use crate::macros::{async_trait, maybe_send};
use crate::pack::{PackerResult, ValidUse};
use crate::util::{Path, Marc};

use super::ResourceLoader;

#[cfg(not(feature = "no-async-send"))]
pub type MarcLoader = Marc<dyn ResourceLoader + Send + Sync>;
#[cfg(not(feature = "no-async-send"))]
pub type MarcResolver = Marc<dyn ResourceResolver + Send + Sync>;
#[cfg(feature = "no-async-send")]
pub type MarcLoader = Marc<dyn ResourceLoader>;
#[cfg(feature = "no-async-send")]
pub type MarcResolver = Marc<dyn ResourceResolver>;

macro_rules! loader_delegate {
    ($func:ident, $type:ty) => {
        #[doc = "Macro-generated loader delegate. See [`ResourceLoader`]"]
        pub async fn $func(&self) -> PackerResult<$type> {
            match &self.path {
                ResourcePath::Url(url) => self.url_loader.$func(url).await,
                ResourcePath::FsPath(path) => self.fs_loader.$func(path.as_ref()).await,
            }
        }
    };
}

/// A resource contains:
/// - an API to load the resource
/// - an API to resolve another resource from its location
#[derive(Clone)]
pub struct Resource {
    path: ResourcePath,
    fs_loader: MarcLoader,
    url_loader: MarcLoader,
    resolver: MarcResolver,
}

impl Resource {
    pub fn new(
        path: ResourcePath,
        fs_loader: MarcLoader,
        url_loader: MarcLoader,
        resolver: MarcResolver,
    ) -> Self {
        Self {
            path,
            fs_loader,
            url_loader,
            resolver,
        }
    }

    /// Create a new resource with the same loader
    pub fn create(&self, path: ResourcePath, resolver: MarcResolver) -> Self {
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
            ResourcePath::Url(url) => url,
            ResourcePath::FsPath(path) => path.as_ref(),
        }
    }

    loader_delegate!(load_structured, Value);
    loader_delegate!(load_utf8, String);
    loader_delegate!(load_image_url, String);

    pub async fn resolve(&self, target: &ValidUse) -> PackerResult<Resource> {
        self.resolver.resolve(self, target).await
    }
}

#[maybe_send(async_trait)]
pub trait ResourceResolver {
    /// Resolve a resource from the given `Use` and loader
    async fn resolve(&self, source: &Resource, target: &ValidUse) -> PackerResult<Resource>;

}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourcePath {
    Url(String),
    FsPath(Path),
}
