//! Celer Compiler API
use instant::Instant;
use log::{error, info};
use std::borrow::Cow;
use std::collections::HashMap;

mod prepare;
pub use prepare::*;
mod compile;
pub use compile::*;

use derivative::Derivative;

use crate::comp::Compiler;
use crate::lang::Preset;
use crate::pack::{PackerError, PackerResult, Phase0, Resource, ValidUse};
use crate::plug::PluginInstance;
use crate::types::{RouteMetadata, ExecDoc};

/// Resolve project.yaml resource under the root resource
pub async fn resolve_project(root_resource: &Resource) -> PackerResult<Resource> {
    let project_ref = ValidUse::Relative("./project.yaml".to_string());
    match root_resource.resolve(&project_ref).await {
        Err(_) => {
            error!("fail to resolve project.yaml");
            Err(PackerError::InvalidProject)
        }
        x => x,
    }
}

pub fn make_project_for_error(source: &str) -> RouteMetadata {
    RouteMetadata {
        title: "[compile error]".to_string(),
        version: "[compile error]".to_string(),
        source: source.to_string(),
        ..Default::default()
    }
}

// TODO #78: Option no longer needed
pub async fn make_doc_for_packer_error(
    source: &str,
    error: PackerError,
) -> Option<ExecDoc<'static>> {
    let comp_doc = Compiler::default()
        .create_empty_doc_for_packer_error(error)
        .await;
    let project = make_project_for_error(source);
    let exec_doc = comp_doc.exec(&project).await.ok()?;
    Some(ExecDoc {
        preface: exec_doc.preface,
        route: exec_doc.route,
        diagnostics: exec_doc.diagnostics,
        project: Cow::Owned(project),
    })
}

pub struct CompilerContext {
    pub start_time: Instant,
    pub project_resource: Resource,
    pub setting: Setting,
    pub phase0: Phase0,
}

impl CompilerContext {
    /// Reset the start time to be now.
    ///
    /// If using a cached compiler context, this should be called so metrics are reported
    /// correctly.
    pub fn reset_start_time(&mut self) {
        self.start_time = Instant::now();
    }

    /// Get the start time of the compilation
    pub fn get_start_time(&self) -> &Instant {
        &self.start_time
    }
}

/// Metadata of the compiler
///
/// This is information needed during compilation,
/// but not needed to render the route.
/// IDEs may also find this useful to provide auto-complete, etc.
#[derive(Default, Debug, Clone)]
pub struct CompilerMetadata {
    pub presets: HashMap<String, Preset>,
    pub plugins: Vec<PluginInstance>,
    pub default_icon_priority: i64,
}

/// Compiler settings
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct Setting {
    /// The maximum depth of `use` properties
    #[derivative(Default(value = "8"))]
    pub max_use_depth: usize,

    /// The maximum depth of object/array levels in the route
    #[derivative(Default(value = "32"))]
    pub max_ref_depth: usize,

    /// The maximum depth of preset namespaces in config
    #[derivative(Default(value = "16"))]
    pub max_preset_namespace_depth: usize,

    /// The maximum depth of preset references in route
    #[derivative(Default(value = "8"))]
    pub max_preset_ref_depth: usize,
}

#[cfg(feature = "wasm")]
#[inline]
pub fn cancel_current_compilation() {
    crate::util::set_cancelled(true);
    info!("cancel requested");
}
