use proc_macro2::Span;
use proc_macro_crate::FoundCrate;
use syn::Ident;

type TokenStream2 = proc_macro2::TokenStream;

pub fn get_compiler_core_crate() -> TokenStream2 {
    let compiler_core = match proc_macro_crate::crate_name("compiler-core") {
        Ok(name) => name,
        Err(_) => {
            panic!("Failed to find compiler-core!");
        }
    };

    match compiler_core {
        FoundCrate::Itself => {
            quote::quote! {
                crate
            }
        },
        FoundCrate::Name(name) => {
            let name = Ident::new(&name, Span::call_site());
            quote::quote! {
                #name
            }
        }
    }
}
