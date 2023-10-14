//! Interop with JS world

use std::marker::PhantomData;

use celerc::{macros::derive_wasm, types::ExecDoc};
use js_sys::Function;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::{prelude::*, describe::WasmDescribe};

mod opaque;
pub use opaque::*;
mod promise;
pub use promise::*;

/// Create a stub JS function to fill in for a function slot that is not yet initialized
pub fn stub_function() -> Function {
    Function::new_no_args("throw new Error(\"not initialized\")")
}

