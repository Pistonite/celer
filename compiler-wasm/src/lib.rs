use js_sys::Function;
use log::info;
use wasm_bindgen::prelude::*;

use celerc::prep::EntryPointsSorted;

mod interop;
use interop::OpaqueExecDoc;
mod compile;
mod loader;
mod logger;

/// Initialize bindings with WASM
#[wasm_bindgen]
pub fn init(
    site_origin: String,
    info_fn: Function,
    warn_fn: Function,
    error_fn: Function,
    load_file: Function,
    load_url: Function,
) {
    let _ = logger::bind(info_fn, warn_fn, error_fn);
    info!("initializing compiler...");
    loader::bind(load_file, load_url);
    let _ = celerc::env::init_site_origin(site_origin);

    info!("compiler initialized");
}

/// Return the entry points (only paths, not aliases) defined in the root project
///
/// If there is any error, this returns 0 entry points
#[wasm_bindgen]
pub async fn get_entry_points() -> Result<EntryPointsSorted, JsValue> {
    let context_builder = compile::new_context_builder();
    let entry_points = match context_builder.get_entry_points().await {
        Ok(x) => x.path_only().into(),
        Err(_) => Default::default(),
    };
    Ok(entry_points)
}

/// Compile a document from web editor
#[wasm_bindgen]
#[inline]
pub async fn compile_document(
    entry_path: Option<String>,
    use_cache: bool,
) -> Result<OpaqueExecDoc, JsValue> {
    compile::compile_document(entry_path, use_cache).await
}
