//! Implementation for `async_trait` and `async_recursion` macros

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{Meta, ItemTrait};

use crate::util;

type TokenStream2 = proc_macro2::TokenStream;
pub fn expand(trait_name: &str, attr: TokenStream, input: TokenStream) -> TokenStream {

    let trait_ident = Ident::new(trait_name, Span::call_site());


    // add Send + Sync super traits
    let (input, input_send_sync): (TokenStream2, TokenStream2) = if trait_ident == "async_trait" {
        match syn::parse::<ItemTrait>(input.clone()) {
            Ok(mut trait_syn) => {
                trait_syn.supertraits.push(syn::parse_quote!(Send));
                trait_syn.supertraits.push(syn::parse_quote!(Sync));
                (TokenStream2::from(input), trait_syn.into_token_stream())
            },
            _ => {
                // an impl block, probably
                let input = TokenStream2::from(input);
                (input.clone(), input)
            }
        }

    } else {
        let input = TokenStream2::from(input);
        (input.clone(), input)
    };

    let celerc = util::compiler_crate_ident();
    let macro_use = quote::quote! {
        #celerc::macros::macro_use
    };
    let is_auto = if attr.is_empty() {
        false
    } else {
        match syn::parse::<Meta>(attr.clone()) {
            Ok(Meta::Path(path)) => path.is_ident("auto"),
            _ => false,
        }
    };

    let attr = TokenStream2::from(attr);

    let out = if is_auto {
        quote::quote! {
            #[cfg_attr(not(feature="wasm"), #macro_use::#trait_ident)]
            #[cfg(not(feature="wasm"))]
            #input_send_sync
            #[cfg_attr(feature="wasm", #macro_use::#trait_ident(?Send))]
            #[cfg(feature="wasm")]
            #input
        }
    } else if attr.is_empty() {
        quote::quote! {
            #[#macro_use::#trait_ident]
            #input_send_sync
        }
    } else {
        quote::quote! {
            #[#macro_use::#trait_ident(?Send)]
            #input
        }
    };

    out.into()
}
