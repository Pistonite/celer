//! Compiler resource resolver and loader implementation for WASM context

use std::cell::RefCell;
use std::collections::HashMap;

use celerc::{pack::{PackerResult, ResourceLoader, PackerError}, yield_now};
use js_sys::{Function, Uint8Array};
use log::info;
use serde_json::Value;
use wasm_bindgen::{JsValue, JsCast};

use crate::wasm::{into_future, stub_function, console_error};

// pub struct FileCache {
//     loader: RefCell<FileLoader>,
//     json_cache: RefCell<HashMap<String, Value>>,
//     /// Callback function to ask JS if a file is changed
//     ///
//     /// Takes in a string as argument.
//     /// Returns a boolean, could throw
//     check_changed: RefCell<Function>,
// }
//
// impl FileCache {
//     pub fn new() -> Self {
//         Self {
//             loader: RefCell::new(FileLoader {
//                 load_file: stub_function(),
//             }),
//             json_cache: RefCell::new(HashMap::new()),
//             check_changed: RefCell::new(stub_function()),
//         }
//     }
//     pub fn init(&self, load_file: Function, check_changed: Function) {
//         self.loader.replace(FileLoader {
//             load_file,
//         });
//         self.check_changed.replace(check_changed);
//     }
// }
//


// #[async_trait::async_trait(?Send)]
// impl ResourceLoader for FileCache {
//     async fn load_raw(&self, path: &str) -> PackerResult<Vec<u8>> {
//         self.loader.borrow().load_raw(path).await
//     }
//     async fn load_image_url(&self, path: &str) -> PackerResult<String> {
//         self.loader.borrow().load_image_url(path).await
//     }
//     async fn load_structured(&self, path: &str) -> PackerResult<Value> {
//         yield_now!()?;  
//         let mut cache = self.json_cache.borrow_mut();
//         let cached_result = cache.get(path);
//         if let Some(v) = cached_result {
//             let is_cache_valid: Result<bool, JsValue> = async {
//                 let changed = self.check_changed.borrow().call1(&JsValue::UNDEFINED, &JsValue::from(path))?;
//                 let changed = changed.as_bool().unwrap_or(true);
//                 info!("check_changed: {} => {}", path, changed);
//                 Ok(!changed)
//             }.await;
//
//             let is_cache_valid = match is_cache_valid {
//                 Ok(v) => v,
//                 Err(e) => {
//                     console_error(&e);
//                     false
//                 },
//             };
//
//             if is_cache_valid {
//                 return Ok(v.clone());
//             }
//
//             cache.remove(path);
//         }
//
//         let v = self.loader.borrow().load_structured(path).await?;
//         cache.insert(path.to_string(), v.clone());
//         Ok(v)
//     }
// }

/// Loader for files from web editor
pub struct FileLoader {

    /// Callback function to ask JS to load a file
    ///
    /// Takes in a string as argument.
    /// Returns a promise that resolves to a Uint8Array that could throw
    load_file: RefCell<Function>
}

impl FileLoader {
    pub fn new() -> Self {
        Self {
                load_file: RefCell::new(stub_function()),
        }
    }
    pub fn init(&self, load_file: Function) {
        self.load_file.replace(load_file);
    }
}


#[async_trait::async_trait(?Send)]
impl ResourceLoader for FileLoader {
    
    async fn load_raw(&self, path: &str) -> PackerResult<Vec<u8>> {
        yield_now!()?;  
        let result: Result<Uint8Array, JsValue> = async {
            let promise = self.load_file.borrow().call1(&JsValue::UNDEFINED, &JsValue::from(path))?;
            let vec: Uint8Array = into_future(promise).await?.dyn_into()?;
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


