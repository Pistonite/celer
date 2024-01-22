//! # Execute (exec) phase
//!
//! This phase transforms the [`CompDoc`] into a [`ExecDoc`], which
//! calculates the movement stack and separate the document into the doc part
//! and the map part.
//!
//! # Input
//! The input is a [`CompDoc`], which is the result of the comp phase.
//!
//! # Work
//! 1. Transform the [`CompDoc`] into a [`ExecDoc`]
//! 2. Call plugin onAfterExecute
//!
//! # Output
//! The output is a [`ExecContext`], which contains the [`ExecDoc`].

use crate::comp::CompDoc;
use crate::lang::IntoDiagnostic;
use crate::plugin::PluginRuntime;

mod error;
pub use error::*;
mod exec_line;
pub use exec_line::*;
mod map;
pub use map::*;
mod exec_section;
pub use exec_section::*;
mod exec_doc;
pub use exec_doc::*;

/// Output of the exec phase
pub struct ExecContext<'p> {
    /// The exec doc
    pub exec_doc: ExecDoc<'p>,
    /// The plugin runtimes at this point
    /// which can be used to run exporters
    pub plugin_runtimes: Vec<Box<dyn PluginRuntime>>,
}

impl<'p> CompDoc<'p> {
    /// Entry point for the exec phase
    pub async fn execute(mut self) -> ExecContext<'p> {
        let mut plugins = std::mem::take(&mut self.plugin_runtimes);
        let mut exec_doc = self.execute_document().await;
        for plugin in &mut plugins {
            if let Err(e) = plugin.on_after_execute(&mut exec_doc) {
                let diag = e.into_diagnostic();
                exec_doc.diagnostics.push(diag);
            }
        }

        ExecContext {
            exec_doc,
            plugin_runtimes: plugins,
        }
    }
}
