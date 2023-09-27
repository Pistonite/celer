use serde_json::Value;

use crate::pack::{PackerError, PackerResult};

use super::ResourceLoader;

/// An empty loader that always fails
pub struct EmptyLoader;

impl EmptyLoader {
    fn throw() -> PackerError {
        PackerError::InvalidPath(
            "resource not allowed in this context (empty loader invoked)".to_string(),
        )
    }
}

#[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
#[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
impl ResourceLoader for EmptyLoader {
    async fn load_raw(&self, _: &str) -> PackerResult<Vec<u8>> {
        Err(Self::throw())
    }

    async fn load_image_url(&self, _: &str) -> PackerResult<String> {
        Err(Self::throw())
    }
}
