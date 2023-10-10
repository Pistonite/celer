/// currently unused. implementation kept for reference
use cached::proc_macro::cached;
use serde_json::Value;

use crate::macros::{async_recursion, maybe_send};
use crate::pack::PackerResult;

use super::{ArcLoader, ResourceLoader};

/// A loader that caches loaded resources in memory. The cache is global.
pub struct GlobalCacheLoader<L> {
    delegate: L,
}

impl<L> GlobalCacheLoader<L> {
    pub fn new(delegate: L) -> Self {
        Self { delegate }
    }
}

#[maybe_send(async_trait)]
impl<L> ResourceLoader for GlobalCacheLoader<L> where L: ResourceLoader {
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
    // TTL of 10 minutes
    time=600,
)]
async fn load_raw_internal(loader: &dyn ResourceLoader, path: &str) -> PackerResult<Vec<u8>> {
    loader.load_raw(path).await
}

#[cached(
    size=256,
    key="String",
    convert = r#"{ path.to_string() }"#,
    // TTL of 10 minutes
    time=600,
)]
async fn load_image_url_internal(loader: &dyn ResourceLoader, path: &str) -> PackerResult<String> {
    loader.load_image_url(path).await
}

#[cached(
    size=256,
    key="String",
    convert = r#"{ path.to_string() }"#,
    // TTL of 10 minutes
    time=600,
)]
async fn load_structured_internal(loader: &dyn ResourceLoader, path: &str) -> PackerResult<Value> {
    loader.load_structured(path).await
}
