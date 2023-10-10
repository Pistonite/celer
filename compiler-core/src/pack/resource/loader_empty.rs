use crate::macros::{async_trait, maybe_send};
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

#[maybe_send(async_trait)]
impl ResourceLoader for EmptyLoader {
    async fn load_raw(&self, _: &str) -> PackerResult<Vec<u8>> {
        Err(Self::throw())
    }

    async fn load_image_url(&self, _: &str) -> PackerResult<String> {
        Err(Self::throw())
    }
}
