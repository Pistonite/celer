//! Compiler resource resolver and loader implementation for WASM context

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use celerc::pack::{PackerResult, ResourceLoader, ValidUse, PackerError};
use celerc::util::Path;
use js_sys::{Function, Promise, Uint8Array};
use serde_json::Value;
use wasm_bindgen::{JsValue, JsCast};

use crate::utils;

pub struct FileCache {
    loader: FileLoader,
    json_cache: RefCell<HashMap<String, Value>>,
    /// Callback function to ask JS if a file is changed
    ///
    /// Takes in a string as argument.
    /// Returns a boolean, could throw
    check_changed: Function,
}

#[async_trait::async_trait(?Send)]
impl ResourceLoader for FileCache {
    async fn load_raw(&self, path: &str) -> PackerResult<Vec<u8>> {
        self.loader.load_raw(path).await
    }
    async fn load_image_url(&self, path: &str) -> PackerResult<String> {
        self.loader.load_image_url(path).await
    }
    async fn load_structured(&self, path: &str) -> PackerResult<Value> {
        let mut cache = self.json_cache.borrow_mut();
        let cached_result = cache.get(path);
        if let Some(v) = cached_result {
            let is_cache_valid: Result<bool, JsValue> = async {
                let changed = self.check_changed.call1(&JsValue::UNDEFINED, &JsValue::from(path))?;
                let changed = changed.as_bool().unwrap_or(false);
                Ok(changed)
            }.await;

            let is_cache_valid = match is_cache_valid {
                Ok(v) => v,
                Err(_) => {
                    // TODO: log error
                    false
                },
            };

            if is_cache_valid {
                return Ok(v.clone());
            }

            cache.remove(path);
        }

        let v = self.loader.load_structured(path).await?;
        cache.insert(path.to_string(), v.clone());
        Ok(v)
    }
}

/// Loader for files from web editor
pub struct FileLoader {

    /// Callback function to ask JS to load a file
    ///
    /// Takes in a string as argument.
    /// Returns a promise that resolves to a Uint8Array that could throw
    load_file: Function
}

#[async_trait::async_trait(?Send)]
impl ResourceLoader for FileLoader {
    
    async fn load_raw(&self, path: &str) -> PackerResult<Vec<u8>> {
        let result: Result<Uint8Array, JsValue> = async {
            let promise = self.load_file.call1(&JsValue::UNDEFINED, &JsValue::from(path))?;
            let vec: Uint8Array = utils::js_await!(promise).dyn_into()?;
            Ok(vec)
        }.await;
        // see if JS call is successful
        let uint8arr = result.map_err(|_| PackerError::LoadFile(format!("loading {path} from JS failed.")))?;
        Ok(uint8arr.to_vec())
    }

    async fn load_image_url(&self, path: &str) -> PackerResult<String> {
        Err(PackerError::NotImpl(
            "FileLoader::load_image_url is not implemented".to_string(),
        ))
    }
}


