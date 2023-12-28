//! Compiler core logic
//!
//! The compiler takes in the raw route JSON blob and extracts the known properties
//! into native structures. It also computes temporal properties like the current coordinates
//! and color at any given point in the route.

use std::borrow::Cow;
use std::convert::Infallible;
use std::ops::Deref;

use derivative::Derivative;
use instant::Instant;
use serde_json::Value;

use crate::lang::IntoDiagnostic;
use crate::env;
use crate::json::{RouteBlobError, RouteBlobArrayIterResult, RouteBlobRef};
use crate::lang::parse_poor;
use crate::pack::{Compiler, PackError};
use crate::prep::Setting;
use crate::env::yield_budget;

mod error;
pub use error::*;
mod line;
pub use line::*;
mod comp_doc;
pub use comp_doc::*;
mod comp_section;
pub use comp_section::*;
mod comp_line;
pub use comp_line::*;

#[cfg(test)]
mod test_utils;

/// Convenience macro for validating a json value and add error
macro_rules! validate_not_array_or_object {
    ($value:expr, $errors:expr, $property:expr) => {{
        let v = $value;
        if v.is_array() || v.is_object() {
            let e = $errors;
            e.push(CompError::InvalidLinePropertyType($property));
            false
        } else {
            true
        }
    }};
}
pub(crate) use validate_not_array_or_object;


static DEFAULT_SETTING: Setting = Setting::default();

impl<'p> Compiler<'p> {
    /// Entry point for the comp phase
    pub async fn compile(mut self) -> CompDoc<'p> {
        for plugin in &mut self.plugin_runtimes {
            if let Err(e) = plugin.on_before_compile(&mut self.ctx) {
                return CompDoc::from_diagnostic(CompError::PluginBeforeCompileError(e), self.ctx);
            }
        }
        let mut plugins = std::mem::take(&mut self.plugin_runtimes);
        let mut comp_doc = self.compile_document().await;
        for plugin in &mut plugins {
            if let Err(e) = plugin.on_after_compile(&mut comp_doc) {
                let diag = CompError::PluginAfterCompileError(e).into_diagnostic();
                comp_doc.diagnostics.push(diag);
            }
        }

        comp_doc.plugin_runtimes = plugins;
        comp_doc
    }

    async fn compile_document(mut self) -> CompDoc<'p> {
        let route_blob = RouteBlobRef::Blob(self.route.as_ref());

        let mut preface = vec![];
        let mut route = vec![];
        let mut diagnostics = vec![];

        // route entry point must be an array
        match route_blob.try_as_array_iter() {
            RouteBlobArrayIterResult::Ok(mut sections) => {
                for section in sections {
                    yield_budget(64).await;
                    todo!()
    //                 self.add_section_or_preface(&mut preface, &mut route_vec, value)
    //                     .await?;
                }
            },
            RouteBlobArrayIterResult::NotArray => {
                diagnostics.push(CompError::InvalidRouteType.into_diagnostic());
            }
            RouteBlobArrayIterResult::Err(e) => {
                diagnostics.push(PackError::BuildRouteError(e).into_diagnostic());
            },
        }

        CompDoc {
            ctx: self.ctx,
            preface,
            route,
            diagnostics,
            known_props: Default::default(),
            // will be filled in after plugin after compile is called
            // due to mutable borrow constraint
            plugin_runtimes: Default::default(),
        }
    }
}
