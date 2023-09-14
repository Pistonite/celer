use cached::proc_macro::cached;
use serde_json::Value;

use crate::pack::PackerResult;

use super::ResourceLoader;

/// A loader that caches loaded resources in memory. The cache is global.
pub struct GlobalCacheLoader<L> where L: ResourceLoader {
    delegate: L,
}

impl<L> GlobalCacheLoader<L> where L: ResourceLoader {
    pub fn new(delegate: L) -> Self {
        Self {
            delegate,
        }
    }
}

#[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
#[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
impl<L> ResourceLoader for GlobalCacheLoader<L> where L: ResourceLoader {

    async fn load_raw(&self, r: &str) -> PackerResult<Vec<u8>> {
        self.delegate.load_raw(r).await
    }

    async fn load_image_url(&self, path: &str) -> PackerResult<String> {
        self.delegate.load_image_url(path).await
    }

    async fn load_structured(&self, path: &str) -> PackerResult<Value> {
        // associative function not supported by cached crate
        // so we need to use a helper
        load_structured_internal(&self.delegate, path).await
    }
}

#[cached(
    size=512,
    key="String",
    convert = r#"{ path.to_string() }"#,
    // TTL of 5 minutes
    time=300,
)]
async fn load_structured_internal(loader: &dyn ResourceLoader, path: &str) -> PackerResult<Value> {
    loader.load_structured(path).await
}

