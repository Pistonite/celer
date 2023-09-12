//! Compiler resource resolver and loader implementation for WASM context

use std::sync::Arc;

use celerc::pack::{PackerResult, ResourcePath, ResourceResolver, Use, Resource, ResourceLoader, ValidUse};
use celerc::util::Path;

use crate::utils::WasmFunction;

pub struct WasmResourceLoader {
    /// Callback function to ask JS to load a file
    ///
    /// Returns a promise that resolves to a Uint8Array that could throw
    load_file: WasmFunction
}

#[async_trait::async_trait]
impl ResourceLoader for WasmResourceLoader {
    async fn load_fs(&self, path: &Path) -> PackerResult<Vec<u8>> {
        todo!()
    }
    async fn load_url(&self, url: &str) -> PackerResult<Vec<u8>> {
        todo!()
    }
}


