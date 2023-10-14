use std::cell::RefCell;

use log::info;
use wasm_bindgen::prelude::*;
use js_sys::{Function, Uint8Array};

use celerc::macros::async_trait;
use celerc::pack::{PackerError, PackerResult, ResourceLoader, MarcLoader};
use celerc::util::Marc;
use celerc::yield_now;

use crate::interop::{self, JsIntoFuture};
use crate::logger;

thread_local! {
    /// Callback function to ask JS to load resource from an URL
    ///
    /// Takes in a string (url) as argument.
    /// Returns a promise that resolves to a Uint8Array that could throw
    static LOAD_URL: RefCell<Function> = RefCell::new(interop::stub_function());
}

/// Bind url loader to a `load_file` function
pub fn bind(load_url: Function) {
    LOAD_URL.replace(load_url);
}

pub fn new_loader() -> MarcLoader {
    Marc::new(UrlLoader)
}

/// Loader for loading a URL
pub struct UrlLoader;

#[async_trait(?Send)]
impl ResourceLoader for UrlLoader {
    async fn load_raw(&self, url: &str) -> PackerResult<Vec<u8>> {
        info!("loading {url}");
        let _ = yield_now!();
        let bytes = async {
            LOAD_URL.with_borrow(|f|{
                f.call1(&JsValue::UNDEFINED, &JsValue::from(url))
            })?.into_future().await?.dyn_into::<Uint8Array>()
        }.await.map_err(|e| {
            logger::raw_error(&e);
            PackerError::LoadUrl(
                format!("loading URL failed: {url}"))
        })?;
        Ok(bytes.to_vec())
    }

    async fn load_image_url(&self, url: &str) -> PackerResult<String> {
        // image is already a URL, so just return it
        Ok(url.to_string())
    }
}
