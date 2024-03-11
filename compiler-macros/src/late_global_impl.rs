//! Implementation for expanding `late_global`

use proc_macro::TokenStream;
use syn::ItemMod;

use crate::util;

type TokenStream2 = proc_macro2::TokenStream;

pub fn expand(attr: TokenStream, input: TokenStream) -> TokenStream {
    let celerc = util::compiler_crate_ident();
    let typ: TokenStream2 = attr.into();
    let parsed_mod = syn::parse_macro_input!(input as ItemMod);

    let vis = &parsed_mod.vis;
    let name = &parsed_mod.ident;
    let attrs = parsed_mod
        .attrs
        .into_iter()
        .map(|attr| {
            quote::quote! {
                #attr
            }
        })
        .collect::<TokenStream2>();

    let content = match parsed_mod.content {
        Some((_, content)) => content
            .into_iter()
            .map(|item| {
                quote::quote! {
                    #item
                }
            })
            .collect::<TokenStream2>(),
        None => quote::quote!(),
    };

    // ref counting is used because thread_local does not have static lifetime

    let expanded = quote::quote! {
        #[automatically_derived]
        #attrs
        #vis mod #name {
            use super::*;
            use #celerc::env::RefCounted;
            #[cfg(not(feature = "wasm"))]
            static GLOBAL: std::sync::OnceLock<RefCounted<#typ>> = std::sync::OnceLock::new();
            #[cfg(not(feature = "wasm"))]
            /// Get the late global value
            pub fn get() -> Option<RefCounted<#typ>> {
                GLOBAL.get().map(RefCounted::clone)
            }
            #[cfg(not(feature = "wasm"))]
            /// Set the late global value
            pub fn set(val: RefCounted<#typ>) -> Result<(), RefCounted<#typ>> { GLOBAL.set(val) }

            #[cfg(feature = "wasm")]
            thread_local! {
                static GLOBAL: once_cell::unsync::OnceCell<RefCounted<#typ>> = once_cell::unsync::OnceCell::new();
            }
            #[cfg(feature = "wasm")]
            /// Get the late global value (cloned)
            pub fn get() -> Option<RefCounted<#typ>> {
                GLOBAL.with(|cell| cell.get().map(RefCounted::clone))
            }
            #[cfg(feature = "wasm")]
            /// Set the late global value
            pub fn set(val: RefCounted<#typ>) -> Result<(), RefCounted<#typ>> {
                GLOBAL.with(|cell| cell.set(val))
            }
            #content
        }
    };

    expanded.into()
}
