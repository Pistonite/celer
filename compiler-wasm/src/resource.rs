//! Compiler resource resolver and loader implementation for WASM context
use std::cell::RefCell;

use celerc::pack::{PackerResult, ResourceLoader, PackerError};
use celerc::yield_now;
use js_sys::{Function, Uint8Array};
use wasm_bindgen::{JsValue, JsCast};

use crate::wasm::{into_future, stub_function};

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


