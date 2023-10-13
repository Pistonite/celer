use celerctypes::GameCoord;
use celerctypes::ExecDoc;
use js_sys::Function;
use log::info;
use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::prelude::*;

mod api;

mod wasm;
use wasm::*;

mod loader_file;
mod loader_url;
mod logger;

mod runtime;

// WASM output types
import! {
    import { ExecDoc } from "low/compiler.g";
    import { Option } from "low/utils";
}

// WASM output type implementation
into! {ExecDoc<'a>, 'a}

ffi!(
    /// Initialize
    pub async fn initCompiler(logger: JsValue, load_file: Function, fetch: Function) -> void {
        api::init(logger, load_file, fetch);
        JsValue::UNDEFINED
    }

    /// Compile a document from web editor
    ///
    /// If use_cache is true, the compiler will use cached results loaded from URLs
    pub async fn compileDocument() -> Option<ExecDoc> {
        api::compile_document().await
    }

    /// Request current compilation be cancelled
    pub async fn requestCancel() -> void {
        celerc::api::cancel_current_compilation();
        JsValue::UNDEFINED
    }
);



#[wasm_bindgen]
pub fn sync_text(input: GameCoord) -> GameCoord {
    let x = format!("sync {}, {}, {}", input.0, input.1, input.2);
    web_sys::console::info_1(
        &x.into()
    );
    GameCoord(3.0,4.0,5.0)
}

#[wasm_bindgen]
pub async fn async_text(input: GameCoord) -> Result<GameCoord, JsValue> {
    let x = format!("async {}, {}, {}", input.0, input.1, input.2);
    web_sys::console::info_1(
        &x.into()
    );
    Ok(GameCoord(3.0,4.0,5.0))
}
