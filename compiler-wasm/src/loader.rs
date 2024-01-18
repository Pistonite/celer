//! Compiler resource loader implementation
//!
//! The resource loader is backed by JS functions binded to the WASM instance
//! at init time

use std::borrow::Cow;
use std::cell::RefCell;

use js_sys::{Array, Function, Uint8Array};
use log::info;
use wasm_bindgen::prelude::*;

use celerc::env::yield_budget;
use celerc::macros::async_trait;
use celerc::res::{Loader, ResResult, ResPath, ResError};

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

    /// Callback function to ask JS to load resource from an URL
    ///
    /// Takes in a string (url) as argument.
    /// Returns a promise that resolves to a Uint8Array that could throw
    static LOAD_URL: RefCell<Function> = RefCell::new(interop::stub_function());
}

pub fn bind(load_file: Function, load_url: Function) {
    LOAD_FILE.replace(load_file);
    LOAD_URL.replace(load_url);
}

pub struct LoaderInWasm;

#[async_trait(?Send)]
impl Loader for LoaderInWasm {
    async fn load_raw<'s>(&'s self, path: &ResPath<'_>) -> ResResult<Cow<'s, [u8]>> {
        let result = match path {
            ResPath::Local(path) => {
                info!("loading local {path}");
                load_file(path.as_ref()).await
            }
            ResPath::Remote(prefix, path) => {
                info!("loading {prefix}{path}");
                load_url(&format!("{prefix}{path}")).await
            }
        };
        yield_budget(1).await;
        result.map(Cow::Owned)
    }
}

pub async fn load_url(url: &str) -> ResResult<Vec<u8>> {

    // this is essentially try { Ok(await load_url(url)) } catch (e) { Err(e) }
    let result = async {
        LOAD_URL
            .with_borrow(|f| f.call1(&JsValue::UNDEFINED, &JsValue::from(url)))?
            .into_future()
            .await?
            .dyn_into::<Uint8Array>()
    }.await;

    match result {
        Ok(bytes) => Ok(bytes.to_vec()),
        Err(e) => {
            logger::raw_error(&e);
            Err(ResError::FailToLoadUrl(url.to_string(), "JavaScript Error".to_string()))
        }
    }
}

pub async fn load_file(path: &str) -> ResResult<Vec<u8>> {
    match load_file_internal(path, false).await {
        Ok(LoadFileOutput::Loaded(bytes)) => Ok(bytes),
        Ok(LoadFileOutput::NotModified) => {
            // since we passed false to check_changed, this shouldn't be possible
            Err(ResError::FailToLoadFile(path.to_string(), "JS returned NotModified when not asked to".to_string()))
        }
        Err(e) => Err(e)
    }
}

#[inline]
pub async fn load_file_check_changed(path: &str) -> ResResult<LoadFileOutput> {
    return load_file_internal(path, true).await;
}

pub enum LoadFileOutput {
    Loaded(Vec<u8>),
    NotModified,
}

/// Load a file from using JS binding
async fn load_file_internal(path: &str, check_changed: bool) -> ResResult<LoadFileOutput> {
    // this is essentially
    // try { Ok(await load_file(path, check_changed)) } catch (e) { Err(e) }
    let result = async {
        LOAD_FILE
            .with_borrow(|f| {
                f.call2(
                    &JsValue::UNDEFINED,
                    &JsValue::from(path),
                    &JsValue::from(check_changed),
                )
            })?
            .into_future()
            .await?
            .dyn_into::<Array>()
    }.await;

    let result = result.and_then(|result| {
        let modified = result.get(0).as_bool().unwrap_or_default();
        if !modified {
            return Ok(LoadFileOutput::NotModified);
        }
        let bytes = result.get(1).dyn_into::<Uint8Array>()?.to_vec();
        Ok(LoadFileOutput::Loaded(bytes))
    });

    match result {
        Ok(output) => Ok(output),
        Err(e) => {
            logger::raw_error(&e);
            Err(ResError::FailToLoadFile(path.to_string(), "JavaScript Error".to_string()))
        }
    }

}
