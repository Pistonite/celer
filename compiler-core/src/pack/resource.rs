use serde_json::Value;

use super::{PackerError, PackerResult, Use};

pub trait ResourceResolver: Send + Sync {
    fn resolve(&self, target: &Use) -> PackerResult<ResourcePath>;
    fn get_resolver(&self, target: &Use) -> PackerResult<Box<dyn ResourceResolver>>;
}

pub enum ResourcePath {
    Url(String),
    FsPath(String),
}

impl ResourcePath {
    /// File path or URL
    pub fn name(&self) -> &str {
        match self {
            ResourcePath::Url(url) => &url,
            ResourcePath::FsPath(path) => &path,
        }
    }

    pub async fn load(&self, loader: &dyn ResourceLoader) -> PackerResult<Vec<u8>> {
        match self {
            ResourcePath::Url(url) => loader.load_url(url).await,
            ResourcePath::FsPath(path) => loader.load_fs(path).await,
        }
    }

    /// Load resource as structured data (json, yaml, etc)
    ///
    /// The type depends on the file extension
    pub async fn load_structured(&self, loader: &dyn ResourceLoader) -> PackerResult<Value> {
        let n = self.name();
        if n.ends_with(".yaml") || n.ends_with(".yml") {
            self.load_yaml(loader).await
        } else if n.ends_with(".json") {
            self.load_json(loader).await
        } else {
            Err(PackerError::UnknownFormat(n.to_string()))
        }
    }

    pub async fn load_yaml(&self, loader: &dyn ResourceLoader) -> PackerResult<Value> {
        let bytes = self.load(loader).await?;
        match serde_yaml::from_slice(&bytes) {
            Ok(v) => Ok(v),
            Err(_) => Err(PackerError::InvalidFormat),
        }
    }

    pub async fn load_json(&self, loader: &dyn ResourceLoader) -> PackerResult<Value> {
        let bytes = self.load(loader).await?;
        match serde_json::from_slice(&bytes) {
            Ok(v) => Ok(v),
            Err(_) => Err(PackerError::InvalidFormat),
        }
    }

    /// Load resource as image URL
    ///
    /// Return value can be a data URL
    pub async fn load_image_url(&self, loader: &dyn ResourceLoader) -> PackerResult<String> {
        match self {
            ResourcePath::Url(url) => Ok(url.to_string()),
            ResourcePath::FsPath(path) => Err(PackerError::NotImpl(
                "Local image is not implemented yet.".to_string(),
            )),
        }
    }
}

#[async_trait::async_trait]
pub trait ResourceLoader: Sync {
    async fn load_fs(&self, path: &str) -> PackerResult<Vec<u8>>;
    async fn load_url(&self, path: &str) -> PackerResult<Vec<u8>>;
}
