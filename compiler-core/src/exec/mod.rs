//! Execution of the compiled document
//!
//! This is the last stage of the compilation process.
//! The [`CompDoc`] will be transformed into a [`ExecDoc`]
//! for rendering

use std::convert::Infallible;

mod exec_line;
pub use exec_line::*;
mod exec_map;
pub use exec_map::*;
mod exec_section;
pub use exec_section::*;
mod exec_doc;
pub use exec_doc::*;

#[cfg(feature = "wasm")]
use crate::util::WasmError;

#[derive(Debug, Clone, thiserror::Error)]
pub enum ExecError {
    #[cfg(feature = "wasm")]
    #[error("wasm error: {0}")]
    Wasm(#[from] WasmError),
}

pub type ExecResult<T> = Result<T, ExecError>;
impl ExecError {
    pub fn is_cancel(&self) -> bool {
        #[cfg(feature = "wasm")]
        let x = matches!(self, Self::Wasm(WasmError::Cancel));
        #[cfg(not(feature = "wasm"))]
        let x = false;
        x
    }
}

impl From<Infallible> for ExecError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
