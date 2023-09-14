use std::{sync::Mutex, collections::HashMap};

use cached::proc_macro::cached;
use serde_json::Value;

use crate::pack::{PackerResult, PackerError};

use super::ResourceLoader;

pub struct UrlLoader;

#[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
#[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
impl ResourceLoader for UrlLoader {

    async fn load_raw(&self, _: &str) -> PackerResult<Vec<u8>> {
        Err(PackerError::NotImpl(
            "UrlLoader::load_raw is not implemented".to_string(),
        ))
    }

    async fn load_image_url(&self, url: &str) -> PackerResult<String> {
        // image is already a URL, so just return it
        Ok(url.to_string())
    }
}
