//! Execution of the compiled document
//!
//! This is the last stage of the compilation process.
//! The [`CompDoc`] will be transformed into a [`ExecDoc`]
//! for rendering

use crate::lang::parse_poor;
use crate::types::DocDiagnostic;

mod exec_line;
pub use exec_line::*;
mod exec_map;
pub use exec_map::*;
mod map;
pub use map::*;
mod exec_section;
pub use exec_section::*;
mod exec_doc;
pub use exec_doc::*;

pub type ExecResult<T> = T;

pub fn add_engine_diagnostics(diagnostics: &mut Vec<DocDiagnostic>, msg_type: &str, msg: &str) {
    diagnostics.push(DocDiagnostic {
        msg: parse_poor(msg),
        msg_type: msg_type.to_string(),
        source: "celer/engine".to_string(),
    });
}
