use proc_macro2::TokenStream;
use quote::quote;

/// A wrapper to remove Send trait to `async_trait` and `async_recursion`
/// if the `wasm` feature flag is enabled.
///
/// # Examples
/// Instead of
/// ```ignore
/// #[async_trait]
/// pub trait XXX {
///     ...
/// }
/// ```
/// Do
/// ```ignore
/// #[maybe_send(async_trait)]
/// pub trait XXX {
///     ...
/// }
/// ```
/// Instead of
/// ```ignore
/// #[async_recursion]
/// pub async fn foo() {
///     ...
/// }
/// ```
/// Do
/// ```ignore
/// #[maybe_send(async_recursion)]
/// pub async fn foo() {
///     ...
/// }
/// ```
#[proc_macro_attribute]
pub fn maybe_send(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = TokenStream::from(attr);
    let input = TokenStream::from(input);
    let tokens = quote! {
        #[cfg_attr(not(feature="wasm"), #attr)]
        #[cfg_attr(feature="wasm", #attr(?Send))]
        #input
    };
    tokens.into()
}

mod derive_wasm_impl;
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
/// If the derive resides in compiler-core, `#[derive_wasm(feature="wasm")]` should be used.
#[proc_macro_attribute]
pub fn derive_wasm(
    feature_attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    derive_wasm_impl::expand(feature_attr, input)
}

/// A wrapper for `#[cfg(test)]` to only define the test when the `test` feature flag is enabled
#[proc_macro_attribute]
pub fn test_suite(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let tokens = quote! {
        #[cfg(feature="test")]
        #[cfg(test)]
        #input
    };
    tokens.into()
}
