//! Workaround for wasm-bindgen-futures currently not allowing lifetime in function signatures

use wasm_bindgen::prelude::*;

use celerc::macros::derive_opaque;
use celerc::ExpoContext;

#[derive_opaque(ExpoContext)]
pub struct OpaqueExpoContext<'p>;
