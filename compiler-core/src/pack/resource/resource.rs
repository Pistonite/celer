use std::sync::Arc;

use serde_json::Value;

use crate::pack::{PackerResult, PackerError, ValidUse};
use crate::util::Path;

/// A resource contains:
/// - an API to load the resource
/// - an API to resolve another resource from its location
#[derive(Clone)]
pub struct Resource {
    path: ResourcePath,
    loader: Arc<dyn ResourceLoader>,
    resolver: Arc<dyn ResourceResolver>,
}

impl Resource {
    pub fn new(
        path: ResourcePath,
        loader: Arc<dyn ResourceLoader>,
        resolver: Arc<dyn ResourceResolver>,
    ) -> Self {
        Self {
            path,
            loader,
            resolver,
        }
    }

    /// Create a new resource with the same loader
    pub fn create(&self, path: ResourcePath, resolver: Arc<dyn ResourceResolver>) -> Self {
        Self {
            path,
            loader: self.loader.clone(),
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

    pub async fn load(&self) -> PackerResult<Vec<u8>> {
        match &self.path {
            ResourcePath::Url(url) => self.loader.load_url(url).await,
            ResourcePath::FsPath(path) => self.loader.load_fs(path).await,
        }
    }

    /// Load resource as structured data (json, yaml, etc)
    ///
    /// The type depends on the file extension
    pub async fn load_structured(&self) -> PackerResult<Value> {
        let n = self.name();
        if n.ends_with(".yaml") || n.ends_with(".yml") {
            self.load_yaml().await
        } else if n.ends_with(".json") {
            self.load_json().await
        } else {
            Err(PackerError::UnknownFormat(n.to_string()))
        }
    }

    pub async fn load_yaml(&self) -> PackerResult<Value> {
        let bytes = self.load().await?;
        match serde_yaml::from_slice(&bytes) {
            Ok(v) => Ok(v),
            Err(_) => Err(PackerError::InvalidFormat),
        }
    }

    pub async fn load_json(&self) -> PackerResult<Value> {
        let bytes = self.load().await?;
        match serde_json::from_slice(&bytes) {
            Ok(v) => Ok(v),
            Err(_) => Err(PackerError::InvalidFormat),
        }
    }

    /// Load resource as image URL
    ///
    /// Return value can be a data URL
    pub async fn load_image_url(&self) -> PackerResult<String> {
        match &self.path {
            ResourcePath::Url(url) => Ok(url.to_string()),
            ResourcePath::FsPath(path) => Err(PackerError::NotImpl(
                "Local image is not implemented yet.".to_string(),
            )),
        }
    }

    pub async fn resolve(&self, target: &ValidUse) -> PackerResult<Resource> {
        self.resolver.resolve(&self, target).await
    }
}

#[async_trait::async_trait]
pub trait ResourceResolver: Send + Sync {
    /// Resolve a resource from the given `Use` and loader
    async fn resolve(&self, source: &Resource, target: &ValidUse) -> PackerResult<Resource>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourcePath {
    Url(String),
    FsPath(Path),
}

#[async_trait::async_trait]
pub trait ResourceLoader: Send + Sync {
    /// Read a resource from the given file path
    async fn load_fs(&self, path: &Path) -> PackerResult<Vec<u8>>;

    /// Load a resource from the given URL
    async fn load_url(&self, url: &str) -> PackerResult<Vec<u8>>;
}


