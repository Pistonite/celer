use celerctypes::ExecDoc;
use js_sys::Boolean;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod api;

mod wasm;
use wasm::*;

mod logger;
mod loader_file;
mod loader_url;

// WASM output types
import! {
    import { ExecDoc } from "low/compiler.g";
    import { Option } from "low/utils";
}

// WASM output type implementation
into! {ExecDoc}

ffi!(
    /// Initialize
    pub async fn initCompiler(logger: JsValue, load_file: Function, fetch: Function) -> void {
        api::init(logger, load_file, fetch);
        JsValue::UNDEFINED
    }

    /// Compile a document from web editor
    ///
    /// If use_cache is true, the compiler will use cached results loaded from URLs
    pub async fn compileDocument(use_cache: JsValue) -> Option<ExecDoc> {
        let b: Boolean = use_cache.dyn_into().expect("use_cache should be a bool");
        api::compile_document(b.into()).await
    }

    /// Request current compilation be cancelled
    pub async fn requestCancel() -> void {
        celerc::api::cancel_current_compilation();
        JsValue::UNDEFINED
    }
);
