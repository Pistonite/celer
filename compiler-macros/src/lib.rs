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
/// `serde::Serialize` and `serde::Deserialize` will be automatically added
/// regardless of the `wasm` feature.
///
/// Under the hood, this uses `serde_wasm_bindgen` and `tsify` to generate the WASM ABI.
/// Make sure these two crates and the `tsify/js` feature is activated with the `wasm` feature.
///
/// Additionally, this macro also implements a `try_to_js_value` method for the type that converts
/// it to a `JsValue` using a borrowed reference.
///
/// # Example
/// ```ignore
/// #[derive(Debug, Clone)]
/// #[derive_wasm]
/// pub struct MyStruct {
///    pub a_thing: i32,
/// }
/// ```
#[proc_macro_attribute]
pub fn derive_wasm(_attr: TokenStream, input: TokenStream) -> TokenStream {
    derive_wasm_impl::expand(input)
}
mod derive_wasm_impl;

/// A wrapper of a raw JS value converted from a lifetime-bounded type.
///
/// Sometimes we want to return a lifetime-bounded value from Rust to WASM.
/// The value must be converted to a JS value when it's still alive. However,
/// this way it would lose the type information. This macro will wrap the JsValue
/// to keep the type information.
///
/// # Example
/// ```ignore
/// // the wrapped type needs to be have `derive_wasm`
/// #[derive_opaque(ExecDoc)] 
/// pub struct OpaqueExecDoc<'a>;
/// // the derived type actually doesn't have lifetime
/// // the lifetime is used to annotate the wrapped type
///
/// // convert
/// OpaqueExecDoc::try_from(exec_doc)
/// ```
/// The example will generate an `OpaqueExecDoc` type.
///
/// You can convert to it from `ExecDoc` with `OpaqueExecDoc::try_from(exec_doc)`,
/// and return it across WASM FFI boundary. The typescript type is the same as `ExecDoc`.
#[proc_macro_attribute]
pub fn derive_opaque(attr: TokenStream, input: TokenStream) -> TokenStream {
    derive_opaque_impl::expand(attr, input)
}
mod derive_opaque_impl;
