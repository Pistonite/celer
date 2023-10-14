use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// A wrapper to add Send trait to `async_trait` and `async_recursion` based on the `no-async-send`
/// feature gate
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
        #[cfg_attr(not(feature="no-async-send"), #attr)]
        #[cfg_attr(feature="no-async-send", #attr(?Send))]
        #input
    };
    tokens.into()
}

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
pub fn derive_wasm(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parsed = {
        let input = input.clone();
        parse_macro_input!(input as DeriveInput)
    };
    let input = TokenStream::from(input);
    let name = parsed.ident;
    let generics = parsed.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        #[cfg_attr(feature="wasm", derive(tsify::Tsify))]
        #[cfg_attr(feature="wasm", tsify(into_wasm_abi, from_wasm_abi))]
        #input

        #[cfg(feature="wasm")]
        #[automatically_derived]
        impl #impl_generics #name #ty_generics #where_clause {
            /// Serialize this struct to a JsValue using serde_wasm_bindgen
            #[inline]
            pub fn try_to_js_value(&self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
                let serializer = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
                Ok(self.serialize(&serializer)?)
            }
        }

        #[cfg(feature="wasm")]
        #[automatically_derived]
        impl #impl_generics Into<wasm_bindgen::JsValue> for #name #ty_generics #where_clause {
            #[inline]
            fn into(self) -> wasm_bindgen::JsValue {
                self.try_to_js_value().unwrap()
            }
        }
    };

    expanded.into()
}
