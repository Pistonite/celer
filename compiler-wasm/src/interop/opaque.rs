//! Workaround for wasm-bindgen-futures currently not allowing lifetime in function signatures

use wasm_bindgen::describe::WasmDescribe;
use wasm_bindgen::prelude::*;

use celerc::exec::ExecDoc;

pub struct OpaqueExecDoc(JsValue);
impl OpaqueExecDoc {
    pub fn wrap(exec_doc: ExecDoc<'_>) -> Result<Self, JsValue> {
        Ok(Self(exec_doc.try_to_js_value()?))
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ExecDoc")]
    type JsType;
}

impl WasmDescribe for OpaqueExecDoc {
    fn describe() {
        JsType::describe();
    }
}

impl From<OpaqueExecDoc> for JsValue {
    fn from(x: OpaqueExecDoc) -> Self {
        x.0
    }
}
