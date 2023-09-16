use celerctypes::ExecDoc;
use js_sys::Function;
use wasm_bindgen::prelude::*;

mod api;

mod wasm;
use wasm::*;

mod resource;
mod logger;

// WASM output types
import!{
    import { ExecDoc } from "low/compiler.g";
    import { Option } from "low/wasm";
}

// WASM output type implementation
into!{ExecDoc}

ffi!(
    /// Initialize
    pub async fn initCompiler(load_file: Function, check_changed: Function) -> void {
        api::init(load_file, check_changed);
        JsValue::UNDEFINED
    }

    /// Compile a document from web editor
    pub async fn compileDocument() -> Option<ExecDoc> {
        api::compile_document().await
    }
);

