//! Compiler resource resolver and loader implementation for WASM context

use std::sync::Arc;

use celerc::pack::{PackerResult, ResourcePath, ResourceResolver, Use, Resource, ResourceLoader, ValidUse, load_resource_from_url, PackerError};
use celerc::util::Path;
use js_sys::{Function, Promise, Uint8Array};
use wasm_bindgen::{JsValue, JsCast};

use crate::utils;


pub struct WasmResourceLoader {
    /// Callback function to ask JS if a file is changed
    ///
    /// Takes in a string as argument.
    /// Returns a boolean, could throw
    check_changed: Function,

    /// Callback function to ask JS to load a file
    ///
    /// Takes in a string as argument.
    /// Returns a promise that resolves to a Uint8Array that could throw
    load_file: Function
}

#[async_trait::async_trait(?Send)]
impl ResourceLoader for WasmResourceLoader {
    async fn load_fs(&self, path: &Path) -> PackerResult<Vec<u8>> {
        let result: Result<Uint8Array, JsValue> = async {
            let promise = self.load_file.call1(&JsValue::UNDEFINED, &JsValue::from(path.as_ref()))?;
            let vec: Uint8Array = utils::js_await!(promise).dyn_into()?;
            Ok(vec)
        }.await;
        // see if JS call is successful
        let uint8arr = result.map_err(|_| PackerError::LoadFile(format!("loading {} from JS failed.", path.as_ref())))?;
        Ok(uint8arr.to_vec())
    }
    async fn load_url(&self, url: &str) -> PackerResult<Vec<u8>> {
        load_resource_from_url(url).await
    }
}


