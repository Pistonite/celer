use celerc::{ExpoDoc, ExportRequest, PluginOptionsRaw};
use js_sys::Function;
use log::info;
use wasm_bindgen::prelude::*;

use celerc::env::RefCounted;
use celerc::prep::EntryPointsSorted;
use celerc::res::{ResPath, Resource};

mod interop;
use interop::OpaqueExpoContext;
mod compiler;
mod loader;
use loader::LoaderInWasm;
mod logger;
mod plugin;
use plugin::SetPluginOptionsResult;

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
    let context_builder = compiler::new_context_builder();
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
) -> Result<OpaqueExpoContext, JsValue> {
    compiler::compile_document(entry_path, use_cache).await
}

/// Export a document from web editor
#[wasm_bindgen]
#[inline]
pub async fn export_document(
    entry_path: Option<String>,
    use_cache: bool,
    req: ExportRequest,
) -> Result<ExpoDoc, JsValue> {
    Ok(compiler::export_document(entry_path, use_cache, req).await)
}

/// Set user plugin options
#[wasm_bindgen]
#[inline]
pub async fn set_plugin_options(options: Option<PluginOptionsRaw>) -> SetPluginOptionsResult {
    plugin::set_plugin_options(options).await
}

pub fn get_root_project_resource() -> Resource<'static, LoaderInWasm> {
    Resource::new(
        ResPath::Local("project.yaml".into()),
        RefCounted::new(LoaderInWasm),
    )
}
