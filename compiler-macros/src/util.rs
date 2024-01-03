use proc_macro2::Span;
use proc_macro_crate::FoundCrate;
use syn::Ident;

type TokenStream2 = proc_macro2::TokenStream;

/// Get the compiler crate identifier as string.
///
/// Returns `None` from crates inside the compiler (i.e. should use `crate` instead)
pub fn compiler_crate() -> Option<String> {
    let compiler_core = match proc_macro_crate::crate_name("compiler-core") {
        Ok(name) => name,
        Err(_) => return None,
    };

    match compiler_core {
        FoundCrate::Itself => None,
        FoundCrate::Name(name) => Some(name),
    }
}

/// Get the compiler crate identifier.
pub fn compiler_crate_ident() -> TokenStream2 {
    match compiler_crate() {
        Some(name) => {
            let name = Ident::new(&name, Span::call_site());
            quote::quote! {
                #name
            }
        }
        None => {
            quote::quote! {
                crate
            }
        }
    }
}
