
mod path;
pub use path::*;

#[cfg(not(feature = "wasm"))]
mod async_for;
#[cfg(not(feature = "wasm"))]
pub(crate) use async_for::async_for;
#[cfg(feature = "wasm")]
pub mod async_for_wasm;
#[cfg(feature = "wasm")]
pub(crate) use async_for_wasm::async_for;

