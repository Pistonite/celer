//! Compiler resource resolver and loader implementation for WASM context
use base64::engine::general_purpose;
use base64::Engine;
use std::cell::RefCell;

use celerc::macros::async_trait;
use celerc::pack::{ImageFormat, PackerError, PackerResult, ResourceLoader, MarcLoader};
use celerc::yield_now;
use js_sys::{Function, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};

use crate::wasm::{self, JsIntoFuture};

thread_local! {
    /// Callback function to ask JS to load a file
    ///
    /// Takes in a string as argument.
    /// Returns a promise that resolves to a Uint8Array that could throw
    static LOAD_FILE: RefCell<Function> = RefCell::new(wasm::stub_function());
}

/// Bind file loader to a `load_file` function
pub fn bind(load_file: Function) {
    LOAD_FILE.replace(load_file);
}

pub fn new() -> MarcLoader {
    Marc::new(FileLoader)
}

/// Loader for files from web editor
pub struct FileLoader;

#[async_trait(?Send)]
impl ResourceLoader for FileLoader {
    async fn load_raw(&self, path: &str) -> PackerResult<Vec<u8>> {
        let _ = yield_now!();
        let result: Result<Uint8Array, JsValue> = async {
            Ok(LOAD_FILE
                .with_borrow(|load_file|{
                    load_file.call1(&JsValue::UNDEFINED, &JsValue::from(path))
                })?.into_future().await?.dyn_into()?
            )
        }
        .await;
        // see if JS call is successful
        let uint8arr =
            result.map_err(|e| PackerError::LoadFile(format!("loading {path} from JS failed: {e}")))?;
        Ok(uint8arr.to_vec())
    }

    async fn load_image_url(&self, path: &str) -> PackerResult<String> {
        let image_format = ImageFormat::try_from_path(path)
            .ok_or_else(|| {
                PackerError::LoadFile(format!("Cannot determine image format from path: {path}"))
            })?
            .media_type();
        let mut data_url = format!("data:{image_format};base64,");
        let vec = self.load_raw(path).await?;
        general_purpose::STANDARD.encode_string(vec, &mut data_url);
        Ok(data_url)
    }
}
