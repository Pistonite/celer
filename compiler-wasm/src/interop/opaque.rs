//! Workaround for wasm-bindgen-futures currently not allowing lifetime in function signatures

use wasm_bindgen::prelude::*;

use celerc::macros::derive_opaque;
use celerc::ExecContext;

#[derive_opaque(ExecContext)]
pub struct OpaqueExecContext<'p>;
