use proc_macro2::TokenStream;
use quote::quote;

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
