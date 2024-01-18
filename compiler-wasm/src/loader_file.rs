//! Compiler resource resolver and loader implementation for WASM context
use std::cell::RefCell;

use base64::engine::general_purpose;
use base64::Engine;
use js_sys::{Array, Function, Uint8Array};
use log::info;
use wasm_bindgen::prelude::*;

// use celerc::macros::async_trait;
// use celerc::pack::{ImageFormat, MarcLoader, PackerError, PackerResult, ResourceLoader};
// use celerc::util::Marc;

use crate::interop::{self, JsIntoFuture};
use crate::logger;

thread_local! {
    /// Callback function to ask JS to load a file
    ///
    /// Arguments:
    /// - path: string
    /// - checkChanged: bool
    /// Returns a promise that resolves to either:
    /// - [true, Uint8Array] if the file was loaded
    /// - [false] if the file was not modified
    ///
    /// The promise is rejected if the file could not be loaded.
    static LOAD_FILE: RefCell<Function> = RefCell::new(interop::stub_function());
}

pub enum LoadOutput {
    Loaded(Vec<u8>),
    NotModified,
}

// /// Bind file loader to a `load_file` function
// pub fn bind(load_file: Function) {
//     LOAD_FILE.replace(load_file);
// }
//
// pub async fn load_file_from_js(path: &str, check_changed: bool) -> Result<LoadOutput, JsValue> {
//     let result = LOAD_FILE
//         .with_borrow(|f| {
//             f.call2(
//                 &JsValue::UNDEFINED,
//                 &JsValue::from(path),
//                 &JsValue::from(check_changed),
//             )
//         })?
//         .into_future()
//         .await?
//         .dyn_into::<Array>()?;
//
//     let modified = result.get(0).as_bool().unwrap_or_default();
//     if !modified {
//         return Ok(LoadOutput::NotModified);
//     }
//     let bytes = result.get(1).dyn_into::<Uint8Array>()?.to_vec();
//     Ok(LoadOutput::Loaded(bytes))
// }
//
// pub fn new_loader() -> MarcLoader {
//     Marc::new(FileLoader)
// }
//
// /// Loader for files from web editor
// pub struct FileLoader;

// #[async_trait(?Send)]
// impl ResourceLoader for FileLoader {
//     async fn load_raw(&self, path: &str) -> PackerResult<Vec<u8>> {
//         info!("loading {path}");
//
//         match load_file_from_js(path, false).await {
//             Ok(LoadOutput::Loaded(bytes)) => Ok(bytes),
//             Ok(LoadOutput::NotModified) => Err(PackerError::LoadFile(format!(
//                 "unreachable: file {path} not modified"
//             ))),
//             Err(e) => {
//                 logger::raw_error(&e);
//                 Err(PackerError::LoadFile(format!(
//                     "loading {path} from JS failed."
//                 )))
//             }
//         }
//     }
//
//     async fn load_image_url(&self, path: &str) -> PackerResult<String> {
//         info!("loading {path}");
//         let image_format = ImageFormat::try_from_path(path)
//             .ok_or_else(|| {
//                 PackerError::LoadFile(format!("Cannot determine image format from path: {path}"))
//             })?
//             .media_type();
//         let mut data_url = format!("data:{image_format};base64,");
//         let vec = self.load_raw(path).await?;
//         general_purpose::STANDARD.encode_string(vec, &mut data_url);
//         Ok(data_url)
//     }
// }
