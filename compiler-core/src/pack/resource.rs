use serde_json::Value;

use super::{PackerError, PackerResult, Use};

pub trait ResourceResolver {
    fn resolve(&self, target: &Use) -> PackerResult<Box<dyn Resource>>;
    fn get_resolver(&self, target: &Use) -> PackerResult<Box<dyn ResourceResolver>>;
}

#[async_trait::async_trait]
pub trait Resource: Sync {
    fn path(&self) -> &ResourcePath;

    /// Name of the resource for display, typically the file path or URL
    fn name(&self) -> &str {
        match self.path() {
            ResourcePath::Url(url) => &url,
            ResourcePath::FsPath(path) => &path,
        }
    }

    async fn load(&self, loader: &dyn ResourceLoader) -> PackerResult<Vec<u8>> {
        match self.path() {
            ResourcePath::Url(url) => loader.load_url(url).await,
            ResourcePath::FsPath(path) => loader.load_fs(path).await,
        }
    }

    /// Load resource as json
    async fn load_structured(&self, loader: &dyn ResourceLoader) -> PackerResult<Value> {
        let bytes = self.load(loader).await?;
        match serde_json::from_slice(&bytes) {
            Ok(v) => Ok(v),
            Err(_) => Err(PackerError::InvalidJSON),
        }
    }

    /// Load resource as image URL
    ///
    /// Return value can be a data URL
    async fn load_image_url(&self, loader: &dyn ResourceLoader) -> PackerResult<String> {
        match self.path() {
            ResourcePath::Url(url) => Ok(url.to_string()),
            ResourcePath::FsPath(path) => Err(PackerError::NotImpl(
                "Local image is not implemented yet.".to_string(),
            )),
        }
    }
}

pub enum ResourcePath {
    Url(String),
    FsPath(String),
}

#[async_trait::async_trait]
pub trait ResourceLoader: Sync {
    async fn load_fs(&self, path: &str) -> PackerResult<Vec<u8>>;
    async fn load_url(&self, path: &str) -> PackerResult<Vec<u8>>;
}
