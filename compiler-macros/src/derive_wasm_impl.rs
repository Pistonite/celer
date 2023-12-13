use proc_macro::TokenStream;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Meta,
};

type TokenStream2 = proc_macro2::TokenStream;

/// Implementation of [`derive_wasm`](crate::derive_wasm) macro
pub fn expand(
    feature_attr: TokenStream,
    input: TokenStream,
) -> TokenStream {
    let feature_attr = TokenStream2::from(feature_attr);
    let mut parsed = {
        let input = input.clone();
        syn::parse_macro_input!(input as DeriveInput)
    };

    let derive_tsify = if feature_attr.is_empty() {
        quote::quote! {
            #[derive(tsify::Tsify)]
            #[tsify(into_wasm_abi, from_wasm_abi)]
        }
    } else {
        transform_tsify(&mut parsed, &feature_attr);
        quote::quote! {
            #[cfg_attr(#feature_attr, derive(tsify::Tsify))]
            #[cfg_attr(#feature_attr, tsify(into_wasm_abi, from_wasm_abi))]
        }
    };

    let name = &parsed.ident;
    let generics = &parsed.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let cfg_attribute = if feature_attr.is_empty() {
        TokenStream2::new()
    } else {
        quote::quote! {
            #[cfg(#feature_attr)]
        }
    };

    let expanded = quote::quote! {
        #derive_tsify
        #parsed

        #cfg_attribute
        #[automatically_derived]
        impl #impl_generics #name #ty_generics #where_clause {
            /// Serialize this struct to a JsValue using serde_wasm_bindgen
            #[inline]
            pub fn try_to_js_value(&self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
                let serializer = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
                Ok(self.serialize(&serializer)?)
            }
        }

        #cfg_attribute
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

/// Transforms tsify attributes according to feature flags
fn transform_tsify(data: &mut DeriveInput, feature: &TokenStream2) {
    match &mut data.data {
        Data::Struct(data) => transform_tsify_for_struct(data, feature),
        Data::Enum(data) => transform_tsify_for_enum(data, feature),
        Data::Union(_) => {
            panic!("currently unions are not supported");
        }
    }
}
fn transform_tsify_for_struct(data: &mut DataStruct, feature: &TokenStream2) {
    match &mut data.fields {
        Fields::Unit => {}
        Fields::Unnamed(fields) => {
            for field in fields.unnamed.iter_mut() {
                transform_tsify_for_field(field, feature);
            }
        }
        Fields::Named(fields) => {
            for field in fields.named.iter_mut() {
                transform_tsify_for_field(field, feature);
            }
        }
    }
}

fn transform_tsify_for_enum(data: &mut DataEnum, feature: &TokenStream2) {
    for variant in data.variants.iter_mut() {
        for attr in variant.attrs.iter_mut() {
            transform_tsify_attr(attr, feature);
        }
        for field in variant.fields.iter_mut() {
            transform_tsify_for_field(field, feature);
        }
    }
}

fn transform_tsify_for_field(field: &mut Field, feature: &TokenStream2) {
    for attr in field.attrs.iter_mut() {
        transform_tsify_attr(attr, feature);
    }
}

fn transform_tsify_attr(attr: &mut Attribute, feature: &TokenStream2) {
    if let Meta::List(list) = &mut attr.meta {
        if !list.path.is_ident("tsify") {
            return;
        }
        list.path = syn::parse_str("cfg_attr").expect("syn::parse_str failed to parse cfg_attr");
        let tokens = &list.tokens;
        let tokens = quote::quote! {
            #feature, tsify(#tokens)
        };
        list.tokens = tokens;
    }
}
