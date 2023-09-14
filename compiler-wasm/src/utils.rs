//! Utils for gluing WASM and JS

use std::future::Future;

use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

/// Import types from compiler-types in the generated TS code
///
/// This defines a function that the build script uses to generate the import block
macro_rules! wasm_import {
( $(import { $($import:ty),* } from $module:literal; )*) => {
    pub fn generate_d_ts_imports() -> String {
        let mut d_ts = String::new();
        $(
            d_ts.push_str("import { ");
            $(
                d_ts.push_str(stringify!($import));
                d_ts.push_str(", ");
            )*
            d_ts.push_str(" } from \"");
            d_ts.push_str($module);
            d_ts.push_str("\";\n\n");
        )*

        d_ts
    }
}
}
pub(crate) use wasm_import;

/// Define the API functions of the wasm module
///
/// # Example
/// ```nocompile
/// /// documentation goes here
/// pub async fn myFunc(input: MyObj) -> MyOutput {
/// }
/// ```
/// This will generate the following TS definition
/// ```typescript
/// /// documentation goes here
/// export function myFunc(input: MyObj): Promise<MyOutput>;
/// ```
/// And the following rust code
/// ```nocompile
/// pub async fn myFunc(input: wasm_bindgen::JsValue) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
///     // inputs are automatically deserialized
///     let input = MyObj::from_wasm(input)?;
///     ...
/// }
/// ```
macro_rules! wasm_api {
(
    $(
        $(#[doc = $doc:literal])*
        pub async fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ret_ty:ty $body:block
    )*
) => {
    pub fn generate_d_ts() -> String {
        let mut d_ts = String::new();
        $(
            d_ts.push_str(concat!(
                $(
                    "///", $doc, "\n",
                )*
                "export function ", stringify!($name), "(", $(stringify!($arg), ": ", stringify!($arg_ty), ",")* ,
                "): Promise<", stringify!($ret_ty), ">;\n\n"
            ));
        )*
        d_ts
    }
    $(
        $(#[doc = $doc])*
        #[allow(non_snake_case)]
        #[wasm_bindgen(js_name = $name, skip_typescript)]
        pub async fn $name($($arg: wasm_bindgen::JsValue),*) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
            $(
                let $arg = <$arg_ty>::from_wasm($arg)?;
            )*
            let result = $body;
            let serializer = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
            let result = result.serialize(&serializer)?;
            Ok(result)
        }
    )*
}
}

pub(crate) use wasm_api;

pub trait WasmFrom : Sized {
    fn from_wasm(value: wasm_bindgen::JsValue) -> Result<Self, wasm_bindgen::JsValue>;
}
impl WasmFrom for wasm_bindgen::JsValue {
    fn from_wasm(value: wasm_bindgen::JsValue) -> Result<Self, wasm_bindgen::JsValue> {
        Ok(value)
    }
}

macro_rules! wasm_from {
($ty:ty) => {
    impl WasmFrom for $ty {
        fn from_wasm(value: wasm_bindgen::JsValue) -> Result<Self, wasm_bindgen::JsValue> {
            let x = serde_wasm_bindgen::from_value::<Self>(value)?;
            Ok(x)
        }
    }
};
}
pub(crate) use wasm_from;

/// Execute in JS context, everything is JsValue
#[inline]
pub async fn js_async<F>(f: F) -> Result<JsValue, JsValue> where F: Future<Output = Result<JsValue, JsValue>> {
    f.await
}

// macro_rules! js_call {}

macro_rules! js_await {
    ($($call:tt)*) => {
        {
            let promise = $($call)*;
            let promise: js_sys::Promise = wasm_bindgen::JsCast::dyn_into(promise)?;
            wasm_bindgen_futures::JsFuture::from(promise).await?
        }
    };
}
pub(crate) use js_await;
