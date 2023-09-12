use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub async fn __yield();
}
/// Async iterator wrapper in WASM
///
/// Each iteration will call window.__yield() to yield control back to the browser
macro_rules! async_for {
    ($v:pat in $iter:expr, $body:stmt) => {{
        for $v in $iter {
            $crate::util::async_for_wasm::__yield().await;
            $body
        }
    }};
}
pub(crate) use async_for;
