use cached::proc_macro::cached;
use serde_json::Value;

use crate::pack::PackerResult;

use super::{ArcLoader, ResourceLoader};

/// A loader that caches loaded resources in memory. The cache is global.
pub struct GlobalCacheLoader {
    delegate: ArcLoader,
}

impl GlobalCacheLoader {
    pub fn new(delegate: ArcLoader) -> Self {
        Self { delegate }
    }
}

#[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
#[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
impl ResourceLoader for GlobalCacheLoader {
    async fn load_raw(&self, r: &str) -> PackerResult<Vec<u8>> {
        load_raw_internal(&self.delegate, r).await
    }

    async fn load_image_url(&self, path: &str) -> PackerResult<String> {
        load_image_url_internal(&self.delegate, path).await
    }

    async fn load_structured(&self, path: &str) -> PackerResult<Value> {
        load_structured_internal(&self.delegate, path).await
    }
}

// associative function not supported by cached crate
// so we need to use helpers

#[cached(
    size=256,
    key="String",
    convert = r#"{ path.to_string() }"#,
    // TTL of 5 minutes
    time=300,
)]
async fn load_raw_internal(loader: &ArcLoader, path: &str) -> PackerResult<Vec<u8>> {
    loader.load_raw(path).await
}

#[cached(
    size=256,
    key="String",
    convert = r#"{ path.to_string() }"#,
    // TTL of 5 minutes
    time=300,
)]
async fn load_image_url_internal(loader: &ArcLoader, path: &str) -> PackerResult<String> {
    loader.load_image_url(path).await
}

#[cached(
    size=256,
    key="String",
    convert = r#"{ path.to_string() }"#,
    // TTL of 5 minutes
    time=300,
)]
async fn load_structured_internal(loader: &ArcLoader, path: &str) -> PackerResult<Value> {
    loader.load_structured(path).await
}
