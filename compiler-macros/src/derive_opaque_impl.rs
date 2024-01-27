
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::DeriveInput;

use crate::util;

type TokenStream2 = proc_macro2::TokenStream;

/// Implementation of [`derive_opaque`](crate::derive_opaque) macro
pub fn expand(attr: TokenStream, input: TokenStream) -> TokenStream {
    if util::compiler_crate().is_none() {
        panic!("derive_opaque should only be used in compiler-wasm");
    }
    let parsed = {
        let input = input.clone();
        syn::parse_macro_input!(input as DeriveInput)
    };

    let attr: TokenStream2 = attr.into();
    // get the name from attr which is a type name
    let attr_str = attr.to_string();

    let vis = &parsed.vis;
    let name = &parsed.ident;
    let generics = &parsed.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let derive_opaque_mod = Ident::new(&format!("__derive_opaque_{}", name), Span::call_site());

    let expanded = quote::quote! {
        #[automatically_derived]
        mod #derive_opaque_mod {
            use super::*;
            use wasm_bindgen::describe::WasmDescribe;
            use wasm_bindgen::prelude::*;

            #vis struct #name(JsValue);

            #[automatically_derived]
            impl #impl_generics TryFrom<#attr #ty_generics> for #name #where_clause {
                type Error = JsValue;
                #[inline]
                fn try_from(inner: #attr #ty_generics) -> Result<Self, Self::Error> {
                    Ok(Self(inner.try_to_js_value()?))
                }
            }
            
            #[automatically_derived]
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(typescript_type = #attr_str)]
                type JsType;
            }

            #[automatically_derived]
            impl #impl_generics WasmDescribe for #name #where_clause {
                #[inline]
                fn describe() {
                    JsType::describe();
                }
            }

            #[automatically_derived]    
            impl #impl_generics From<#name> for JsValue #where_clause {
                #[inline]
                fn from(inner: #name) -> Self {
                    inner.0
                }
            }
        }
        #vis use #derive_opaque_mod::#name;
    };

    expanded.into()
}
