mod path;
pub use path::*;

#[cfg(not(feature = "wasm"))]
mod async_macro;
#[cfg(not(feature = "wasm"))]
pub(crate) use async_macro::*;
#[cfg(feature = "wasm")]
pub mod async_macro_wasm;
#[cfg(feature = "wasm")]
pub(crate) use async_macro_wasm::*;

mod site_origin;
pub use site_origin::*;
