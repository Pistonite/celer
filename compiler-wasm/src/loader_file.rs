//! Compiler resource resolver and loader implementation for WASM context
use std::cell::RefCell;

use base64::engine::general_purpose;
use base64::Engine;
use js_sys::{Function, Uint8Array};
use log::info;
use wasm_bindgen::prelude::*;

use celerc::macros::async_trait;
use celerc::pack::{ImageFormat, MarcLoader, PackerError, PackerResult, ResourceLoader};
use celerc::util::{Marc, yield_budget};

use crate::interop::{self, JsIntoFuture};
use crate::logger;

thread_local! {
    /// Callback function to ask JS to load a file
    ///
    /// Takes in a string as argument.
    /// Returns a promise that resolves to a Uint8Array that could throw
    static LOAD_FILE: RefCell<Function> = RefCell::new(interop::stub_function());
}

/// Bind file loader to a `load_file` function
pub fn bind(load_file: Function) {
    LOAD_FILE.replace(load_file);
}

pub fn new_loader() -> MarcLoader {
    Marc::new(FileLoader)
}

/// Loader for files from web editor
pub struct FileLoader;

#[async_trait(?Send)]
impl ResourceLoader for FileLoader {
    async fn load_raw(&self, path: &str) -> PackerResult<Vec<u8>> {
        info!("loading {path}");
        yield_budget(1).await;

        let bytes = async {
            LOAD_FILE
                .with_borrow(|f| f.call1(&JsValue::UNDEFINED, &JsValue::from(path)))?
                .into_future()
                .await?
                .dyn_into::<Uint8Array>()
        }
        .await
        .map_err(|e| {
            logger::raw_error(&e);
            PackerError::LoadFile(format!("loading {path} from JS failed."))
        })?;
        Ok(bytes.to_vec())
    }

    async fn load_image_url(&self, path: &str) -> PackerResult<String> {
        info!("loading {path}");
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
