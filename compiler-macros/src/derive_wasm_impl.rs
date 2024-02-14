use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Meta, Type};

use crate::util;

type TokenStream2 = proc_macro2::TokenStream;

/// Implementation of [`derive_wasm`](crate::derive_wasm) macro
pub fn expand(input: TokenStream) -> TokenStream {
    let is_compiler = util::compiler_crate().is_none();
    let celerc = util::compiler_crate_ident();
    let wasm = quote::quote! {
        #celerc::macros::macro_use::wasm
    };

    let mut parsed = {
        let input = input.clone();
        syn::parse_macro_input!(input as DeriveInput)
    };

    transform_input(&mut parsed, is_compiler);
    let derive_tsify = if !is_compiler {
        quote::quote! {
            #[derive(tsify::Tsify)]
            #[tsify(into_wasm_abi, from_wasm_abi)]
        }
    } else {
        quote::quote! {
            #[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
            #[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
        }
    };

    let name = &parsed.ident;
    let generics = &parsed.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let cfg_feature_wasm = if !is_compiler {
        TokenStream2::new()
    } else {
        quote::quote! {
            #[cfg(feature = "wasm")]
        }
    };

    let derive_wasm_mod = Ident::new(&format!("__derive_wasm_{}", name), Span::call_site());

    let vis = &parsed.vis;

    let expanded = quote::quote! {
        #[allow(non_snake_case)]
        #[automatically_derived]
        mod #derive_wasm_mod {
            use super::*;
            use serde::{Serialize, Deserialize};
            #cfg_feature_wasm
            use #wasm::{tsify, wasm_bindgen, serde_wasm_bindgen};

            #[derive(serde::Serialize, serde::Deserialize)]
            #derive_tsify
            #[serde(rename_all(serialize = "camelCase", deserialize = "kebab-case"))]
            #parsed

            #cfg_feature_wasm
            impl #impl_generics #name #ty_generics #where_clause {
                /// Serialize this struct to a JsValue using serde_wasm_bindgen
                #[inline]
                pub fn try_to_js_value(&self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
                    let serializer = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
                    Ok(self.serialize(&serializer)?)
                }
            }

            #cfg_feature_wasm
            #[automatically_derived]
            impl #impl_generics Into<wasm_bindgen::JsValue> for #name #ty_generics #where_clause {
                #[inline]
                fn into(self) -> wasm_bindgen::JsValue {
                    self.try_to_js_value().unwrap()
                }
            }
        }
        #vis use #derive_wasm_mod::#name;
    };

    expanded.into()
}

/// Transforms struct input to add `tsify` and `serde` attributes
///
/// - Adds `#[serde(skip_serializing_if = "Option::is_none")]` for Option fields
fn transform_input(data: &mut DeriveInput, add_cfg_wasm: bool) {
    match &mut data.data {
        Data::Struct(data) => transform_input_for_struct(data, add_cfg_wasm),
        Data::Enum(data) => transform_input_for_enum(data, add_cfg_wasm),
        Data::Union(_) => {
            panic!("currently unions are not supported");
        }
    }
}
fn transform_input_for_struct(data: &mut DataStruct, add_cfg_wasm: bool) {
    match &mut data.fields {
        Fields::Unit => {}
        Fields::Unnamed(fields) => {
            for field in fields.unnamed.iter_mut() {
                transform_input_field(field, add_cfg_wasm);
            }
        }
        Fields::Named(fields) => {
            for field in fields.named.iter_mut() {
                transform_input_field(field, add_cfg_wasm);
            }
        }
    }
}

fn transform_input_for_enum(data: &mut DataEnum, add_cfg_wasm: bool) {
    for variant in data.variants.iter_mut() {
        for attr in variant.attrs.iter_mut() {
            add_cfg_wasm_to_tsify(attr);
        }
        for field in variant.fields.iter_mut() {
            transform_input_field(field, add_cfg_wasm);
        }
    }
}

fn transform_input_field(field: &mut Field, add_cfg_wasm: bool) {
    let mut allow_map = false;
    let mut attrs = Vec::with_capacity(field.attrs.len());
    std::mem::swap(&mut field.attrs, &mut attrs);

    for mut attr in attrs {
        if add_cfg_wasm {
            add_cfg_wasm_to_tsify(&mut attr);
        }
        if attr.path().is_ident("allow_map") {
            allow_map = true;
        } else {
            field.attrs.push(attr);
        }
    }
    if let Type::Path(path) = &field.ty {
        if let Some(seg) = path.path.segments.last() {
            if seg.ident == "Option" {
                let attr_tokens = quote::quote! {
                    #[serde(skip_serializing_if = "Option::is_none")]
                };
                let attr = syn::parse2::<AttrInner>(attr_tokens)
                    .expect("syn::parse2 failed to parse attr_tokens");
                field.attrs.push(attr.0);
            } else if !allow_map {
                #[allow(clippy::collapsible_if)]
                if seg.ident == "BTreeMap" || seg.ident == "HashMap" {
                    let celerc = util::compiler_crate_ident();
                    panic!("Use {celerc}::util::StringMap instead of `BTreeMap` or `HashMap` in types that are exposed through WASM interface, to automatically get the correct TypeScript types.")
                }
            }
        }
    }
}

/// Transforms `#[tsify(...)]` attributes to `#[cfg_attr(feature = "wasm", tsify(...))]` attributes
fn add_cfg_wasm_to_tsify(attr: &mut Attribute) {
    if let Meta::List(list) = &mut attr.meta {
        if !list.path.is_ident("tsify") {
            return;
        }
        list.path = syn::parse_str("cfg_attr").expect("syn::parse_str failed to parse cfg_attr");
        let tokens = &list.tokens;
        let tokens = quote::quote! {
            feature = "wasm", tsify(#tokens)
        };
        list.tokens = tokens;
    }
}

struct AttrInner(pub(crate) Attribute);
impl Parse for AttrInner {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attr: Attribute = input
            .call(Attribute::parse_outer)?
            .into_iter()
            .next()
            .expect("no attribute!");
        Ok(AttrInner(attr))
    }
}
