use serde_json::Value;

use crate::pack::{PackerResult, PackerError};

/// Loader that loads resources from external place
#[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
#[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
pub trait ResourceLoader {
    /// Load a resource as raw bytes
    async fn load_raw(&self, path: &str) -> PackerResult<Vec<u8>>;

    /// Load structured value. The type depends on the file extension read from the ref
    async fn load_structured(&self, path: &str) -> PackerResult<Value> {
        let v = if path.ends_with(".yaml") || path.ends_with(".yml") {
            self.load_yaml(path).await?
        } else if path.ends_with(".json") {
            self.load_json(path).await?
        } else {
            return Err(PackerError::UnknownFormat(path.to_string()));
        };
        Ok(v)
    }

    async fn load_yaml(&self, path: &str) -> PackerResult<Value> {
        let bytes = self.load_raw(path).await?;
        match serde_yaml::from_slice(&bytes) {
            Ok(v) => Ok(v),
            Err(_) => Err(PackerError::InvalidFormat),
        }
    }

    async fn load_json(&self, path: &str) -> PackerResult<Value> {
        let bytes = self.load_raw(path).await?;
        match serde_json::from_slice(&bytes) {
            Ok(v) => Ok(v),
            Err(_) => Err(PackerError::InvalidFormat),
        }
    }

    /// Load an image resource as URL
    async fn load_image_url(&self, path: &str) -> PackerResult<String>;
}

