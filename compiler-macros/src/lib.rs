use proc_macro::TokenStream;

mod util;

/// A wrapper for `async_trait` to add (?Send) in some cases.
///
/// # Examples
/// Use the normal `async_trait` macros
/// when `Send` is explicitly required or need to be explicitly removed:
/// ```no_compile
/// #[async_trait]
/// pub trait XXX {
///     ...
/// }
/// #[async_trait(?Send)]
/// pub trait XXX {
///     ...
/// }
/// ```
/// Use `auto` to remove Send when wasm feature is on
/// ```no_compile
/// #[async_trait(auto)]
/// pub trait XXX {
///     ...
/// }
/// ```
#[proc_macro_attribute]
pub fn async_trait(attr: TokenStream, input: TokenStream) -> TokenStream {
    async_impl::expand("async_trait", attr, input)
}

/// A wrapper for `async_recursion`
/// to add (?Send) in some cases.
///
/// # Examples
/// Use the normal `async_recursion` macros
/// when `Send` is explicitly required or need to be explicitly removed:
/// ```no_compile
/// #[async_recursion]
/// pub async fn foo() {
///     ...
/// }
/// #[async_recursion(?Send)]
/// pub async fn foo() {
///     ...
/// }
/// ```
/// Use `auto` to remove Send when wasm feature is on
/// ```no_compile
/// #[async_recursion(auto)]
/// pub async fn foo() {
///     ...
/// }
/// ```
#[proc_macro_attribute]
pub fn async_recursion(attr: TokenStream, input: TokenStream) -> TokenStream {
    async_impl::expand("async_recursion", attr, input)
}
mod async_impl;

/// A wrapper to add WASM support to a type so it can be send across the WASM ABI boundary.
/// All derived code/attributes are behind the `wasm` feature gate.
///
/// The target type should also derive `serde::Serialize` and `serde::Deserialize`.
///
/// Under the hood, this uses `serde_wasm_bindgen` and `tsify` to generate the WASM ABI.
/// Make sure these two crates and the `tsify/js` feature is activated with the `wasm` feature.
///
/// Additionally, this macro also implements a `try_to_js_value` method for the type that converts
/// it to a `JsValue` using a borrowed reference.
///
/// # Example
/// ```ignore
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// #[derive_wasm]
/// #[serde(rename_all = "camelCase")]
/// pub struct MyStruct {
///    pub a_thing: i32,
/// }
/// ```
#[proc_macro_attribute]
pub fn derive_wasm(_attr: TokenStream, input: TokenStream) -> TokenStream {
    derive_wasm_impl::expand(input)
}
mod derive_wasm_impl;
