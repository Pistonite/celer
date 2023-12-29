//! # Compile (comp) phase
//!
//! This phase transforms the route from raw JSON into structured data (a [`CompDoc`]).
//! It also may handle errors from earlier phases and show them in the document.
//!
//! # Input
//! The input is [`Compiler`] from pack phase
//!
//! # Work
//! 1. Call plugin onBeforeCompile
//! 2. Traverse the route and compile route structure
//! 3. Call plugin onAfterCompile
//!
//! # Output
//! The output is a [`CompDoc`]

use crate::lang::{DocRichText, DocDiagnostic, IntoDiagnostic};
use crate::json::{RouteBlobError, RouteBlobArrayIterResult, RouteBlobRef};
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
            RouteBlobArrayIterResult::Ok(sections) => {
                for section in sections {
                    yield_budget(64).await;
                    self.compile_section_or_preface(section, &mut route, &mut preface, &mut diagnostics).await;
                }
            },
            RouteBlobArrayIterResult::NotArray => {
                diagnostics.push(CompError::InvalidRouteType.into_diagnostic());
            }
            RouteBlobArrayIterResult::Err(e) => {
                diagnostics.push(PackError::BuildRouteError(e).into_diagnostic());
            },
        }

        // // pass 2 (sequential) - coordinates are propagated
        // for section in &mut route {
        //     yield_budget(64).await;
        //     section.sequential_pass(&mut self);
        // }

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

    async fn compile_section_or_preface(
        &self, 
        section_ref: RouteBlobRef<'p>,
        route: &mut Vec<CompSection>,
        prefaces: &mut Vec<DocRichText>,
        diagnostics: &mut Vec<DocDiagnostic>,
    ) {
        match self.compile_section(section_ref.clone(), route, diagnostics).await {
            Some(section) => route.push(section),
            None => {
                match self.compile_preface(section_ref) {
                    Ok(preface) => prefaces.push(preface),
                    Err(e) => {
                        let e = PackError::BuildRouteSectionError(e);
                        // since error is in the preface
                        // add to overall diagnostics
                        diagnostics.push(e.into_diagnostic());
                    }
                }
            }
        }
    }
}
