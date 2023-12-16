//! # compiler-base
//!
//! Low-level utilities and functionalities for the Celer compiler.

pub mod env;
pub mod json;
pub mod lang;
pub mod prop;
pub mod res;
pub mod util;

/// Macro re-exports
pub mod macros {
    pub use celerm::*;

    /// External macros that are used by our macros
    pub mod macro_use {
        pub use async_recursion::async_recursion;
        pub use async_trait::async_trait;

        #[cfg(feature = "wasm")]
        pub mod wasm {
            pub mod tsify {
                pub use tsify::*;
            }
            pub mod wasm_bindgen {
                pub use wasm_bindgen::*;
            }
            pub mod serde_wasm_bindgen {
                pub use serde_wasm_bindgen::*;
            }
        }

    }
}
