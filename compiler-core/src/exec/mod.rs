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
use crate::macros::derive_wasm;
use crate::plugin::{PluginMetadata, PluginRuntime};

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
#[derive_wasm]
pub struct ExecContext<'p> {
    /// The exec doc
    pub exec_doc: ExecDoc<'p>,
    /// Plugin information collected, including disabled plugins
    pub plugin_metadata: Vec<PluginMetadata>,
    /// The plugin runtimes at this point
    /// which can be used to run exporters
    #[serde(skip)]
    pub plugin_runtimes: Vec<Box<dyn PluginRuntime>>,
}

impl ExecContext<'static> {
    pub fn from_diagnostic<T>(error: T) -> Self
    where
        T: IntoDiagnostic,
    {
        ExecContext {
            exec_doc: ExecDoc::from_diagnostic(error),
            plugin_metadata: vec![],
            plugin_runtimes: vec![],
        }
    }
}

impl<'p> CompDoc<'p> {
    /// Entry point for the exec phase
    pub async fn execute(mut self) -> ExecContext<'p> {
        let mut plugins = std::mem::take(&mut self.plugin_runtimes);
        let plugin_metadata = std::mem::take(&mut self.plugin_meta);
        let mut exec_doc = self.execute_document().await;
        for plugin in &mut plugins {
            if let Err(e) = plugin.on_after_execute(&mut exec_doc).await {
                let diag = e.into_diagnostic();
                exec_doc.diagnostics.push(diag);
            }
        }

        ExecContext {
            exec_doc,
            plugin_runtimes: plugins,
            plugin_metadata,
        }
    }
}
