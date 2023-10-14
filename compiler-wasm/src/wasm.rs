//! Utils for gluing WASM and JS

// macro_rules! ffi {
// (
//     $(
//         $(#[doc = $doc:literal])*
//         pub async fn $name:ident($($arg:ident: $arg_ty:ty),*) -> $ret_ty:ty $body:block
//     )*
// ) => {
//     pub fn generate_d_ts() -> String {
//         let mut d_ts = String::new();
//         $(
//             d_ts.push_str(concat!(
//                 $(
//                     "///", $doc, "\n",
//                 )*
//                 "export function ", stringify!($name), "(", $(stringify!($arg), ": ", stringify!($arg_ty), ",", )*
//                 "): Promise<", stringify!($ret_ty), ">;\n\n"
//             ));
//         )*
//         d_ts
//     }
//     $(
//         $(#[doc = $doc])*
//         #[allow(non_snake_case)]
//         #[wasm_bindgen(js_name = $name, skip_typescript)]
//         pub async fn $name($($arg: wasm_bindgen::JsValue),*) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
//             $(
//                 let $arg = <$arg_ty>::from_wasm($arg)?;
//             )*
//             let result = $body;
//             result.into_wasm()
//         }
//     )*
// }
// }
//
// pub(crate) use ffi;

// pub trait WasmFrom: Sized {
//     fn from_wasm(value: JsValue) -> Result<Self, JsValue>;
// }
// impl WasmFrom for JsValue {
//     fn from_wasm(value: JsValue) -> Result<Self, JsValue> {
//         Ok(value)
//     }
// }
// impl WasmFrom for Function {
//     fn from_wasm(value: JsValue) -> Result<Self, JsValue> {
//         value.dyn_into()
//     }
// }

// pub trait WasmInto {
//     fn into_wasm(self) -> Result<JsValue, JsValue>;
// }
// impl WasmInto for JsValue {
//     #[inline]
//     fn into_wasm(self) -> Result<JsValue, JsValue> {
//         Ok(self)
//     }
// }
// impl WasmInto for Result<JsValue, JsValue> {
//     #[inline]
//     fn into_wasm(self) -> Result<JsValue, JsValue> {
//         self
//     }
// }
// impl<T> WasmInto for Option<T>
// where
//     T: WasmInto,
// {
//     fn into_wasm(self) -> Result<JsValue, JsValue> {
//         match self {
//             Some(v) => v.into_wasm(),
//             None => Ok(JsValue::UNDEFINED),
//         }
//     }
// }

// macro_rules! from {
//     ($ty:ty) => {
//         impl WasmFrom for $ty {
//             fn from_wasm(value: wasm_bindgen::JsValue) -> Result<Self, wasm_bindgen::JsValue> {
//                 let x = serde_wasm_bindgen::from_value::<Self>(value)?;
//                 Ok(x)
//             }
//         }
//     };
// }
// pub(crate) use from;
//
// from! {String}
//
// macro_rules! into {
//     ($ty:ty) => {
//         impl WasmInto for $ty {
//             fn into_wasm(self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
//                 let serializer =
//                     serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
//                 let result = <Self as serde::Serialize>::serialize(&self, &serializer)?;
//                 Ok(result)
//             }
//         }
//     };
//     ($ty:ty, $( $life:tt),* ) => {
//         impl<$($life)*> WasmInto for $ty {
//             fn into_wasm(self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
//                 let serializer =
//                     serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
//                 let result = <Self as serde::Serialize>::serialize(&self, &serializer)?;
//                 Ok(result)
//             }
//         }
//     };
// }
// pub(crate) use into;

// impl WasmFrom for bool {
//     fn from_wasm(value: JsValue) -> Result<Self, JsValue> {
//         let b: Boolean = value.dyn_into()?;
//         Ok(b.into())
//     }
// }

// Take a promise and return a future
// pub async fn into_future(promise: JsValue) -> Result<JsValue, JsValue> {
//     let promise: Promise = promise.dyn_into()?;
//     JsFuture::from(promise).await
// }


