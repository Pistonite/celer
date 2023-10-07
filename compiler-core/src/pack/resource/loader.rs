use serde_json::Value;

use crate::pack::{PackerError, PackerResult};

/// Loader that loads resources from external place
#[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
#[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
pub trait ResourceLoader {
    /// Load a resource as raw bytes
    async fn load_raw(&self, path: &str) -> PackerResult<Vec<u8>>;

    async fn load_utf8(&self, path: &str) -> PackerResult<String> {
        let bytes = self.load_raw(path).await?;
        match String::from_utf8(bytes) {
            Ok(v) => Ok(v),
            Err(e) => Err(PackerError::InvalidUtf8(path.to_string())),
        }
    }

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
            Err(e) => Err(PackerError::InvalidFormat(path.to_string(), e.to_string())),
        }
    }

    async fn load_json(&self, path: &str) -> PackerResult<Value> {
        let bytes = self.load_raw(path).await?;
        match serde_json::from_slice(&bytes) {
            Ok(v) => Ok(v),
            Err(e) => Err(PackerError::InvalidFormat(path.to_string(), e.to_string())),
        }
    }

    /// Load an image resource as URL
    async fn load_image_url(&self, path: &str) -> PackerResult<String>;
}
