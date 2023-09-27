//! Utils for gluing WASM and JS

use js_sys::{Function, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

/// Import types from compiler-types in the generated TS code
///
/// This defines a function that the build script uses to generate the import block
macro_rules! import {
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
pub(crate) use import;

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
macro_rules! ffi {
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
                "export function ", stringify!($name), "(", $(stringify!($arg), ": ", stringify!($arg_ty), ",", )*
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
            result.into_wasm()
        }
    )*
}
}

pub(crate) use ffi;

pub trait WasmFrom: Sized {
    fn from_wasm(value: JsValue) -> Result<Self, JsValue>;
}
impl WasmFrom for JsValue {
    fn from_wasm(value: JsValue) -> Result<Self, JsValue> {
        Ok(value)
    }
}
impl WasmFrom for Function {
    fn from_wasm(value: JsValue) -> Result<Self, JsValue> {
        value.dyn_into()
    }
}

pub trait WasmInto {
    fn into_wasm(self) -> Result<JsValue, JsValue>;
}
impl WasmInto for JsValue {
    fn into_wasm(self) -> Result<JsValue, JsValue> {
        Ok(self)
    }
}
impl<T> WasmInto for Option<T>
where
    T: WasmInto,
{
    fn into_wasm(self) -> Result<JsValue, JsValue> {
        match self {
            Some(v) => v.into_wasm(),
            None => Ok(JsValue::UNDEFINED),
        }
    }
}

macro_rules! from {
    ($ty:ty) => {
        impl WasmFrom for $ty {
            fn from_wasm(value: wasm_bindgen::JsValue) -> Result<Self, wasm_bindgen::JsValue> {
                let x = serde_wasm_bindgen::from_value::<Self>(value)?;
                Ok(x)
            }
        }
    };
}
pub(crate) use from;

from! {String}

macro_rules! into {
    ($ty:ty) => {
        impl WasmInto for $ty {
            fn into_wasm(self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
                let serializer =
                    serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
                let result = <Self as serde::Serialize>::serialize(&self, &serializer)?;
                Ok(result)
            }
        }
    };
}
pub(crate) use into;

/// Take a promise and return a future
pub async fn into_future(promise: JsValue) -> Result<JsValue, JsValue> {
    let promise: Promise = promise.dyn_into()?;
    JsFuture::from(promise).await
}

/// Create a stub JS function to fill in for a function slot that is not yet initialized
pub fn stub_function() -> Function {
    Function::new_no_args("throw new Error(\"not initialized\")")
}
