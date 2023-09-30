use std::cell::RefCell;

use celerc::pack::{PackerError, PackerResult, ResourceLoader};
use celerc::yield_now;
use js_sys::{Function, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};

use crate::wasm::{into_future, stub_function};

/// Loader for loading a URL using a provided JS function
pub struct UrlLoader {
    /// Callback function to ask JS to load a file
    ///
    /// Takes in a string (url) as argument.
    /// Returns a promise that resolves to a Uint8Array that could throw
    fetch: RefCell<Function>,
}

impl UrlLoader {
    pub fn new() -> Self {
        Self {
            fetch: RefCell::new(stub_function()),
        }
    }
    pub fn init(&self, fetch: Function) {
        self.fetch.replace(fetch);
    }
}

#[async_trait::async_trait(?Send)]
impl ResourceLoader for UrlLoader {
    async fn load_raw(&self, url: &str) -> PackerResult<Vec<u8>> {
        yield_now!()?;
        let result: Result<Uint8Array, JsValue> = async {
            let promise = self
                .fetch
                .borrow()
                .call1(&JsValue::UNDEFINED, &JsValue::from(url))?;
            let vec: Uint8Array = into_future(promise).await?.dyn_into()?;
            Ok(vec)
        }
        .await;
        // see if JS call is successful
        let uint8arr =
            result.map_err(|_| PackerError::LoadUrl(format!("loading URL failed: {url}")))?;
        Ok(uint8arr.to_vec())
    }

    async fn load_image_url(&self, url: &str) -> PackerResult<String> {
        // image is already a URL, so just return it
        Ok(url.to_string())
    }
}
