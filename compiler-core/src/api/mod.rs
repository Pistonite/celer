//! Celer Compiler API
use std::borrow::Cow;

use derivative::Derivative;
use instant::Instant;

use crate::comp::Compiler;
use crate::pack::{PackerError, PackerResult, Phase0};
use crate::res::{Resource, ValidUse, Use, Loader};
use crate::types::{ExecDoc};

mod prepare;
pub use prepare::*;
mod compile;
pub use compile::*;

/// Resolve project.yaml resource under the root resource
pub fn resolve_project<'a, L>(root_resource: &Resource<'a, L>) -> PackerResult<Resource<'a, L>> 
where L: Loader
{
    let project_ref = ValidUse::Relative("./project.yaml".to_string());
    Ok(root_resource.resolve(&project_ref)?)
}

/// Resolve an absolute path from the resource
///
/// Returns Err if the path is not a valid absolute path that can be used with a `use` property
pub async fn resolve_absolute<'a, L>(resource: &Resource<'a, L>, path: String) -> PackerResult<Resource<'a, L>> 
where L: Loader
{
    match Use::from(path) {
        Use::Valid(valid) if matches!(valid, ValidUse::Absolute(_)) => {
            Ok(resource.resolve(&valid)?)
        }
        other => Err(PackerError::InvalidPath(other.to_string())),
    }
}

// pub fn make_project_for_error(source: &str) -> RouteMetadata {
//     RouteMetadata {
//         title: "[compile error]".to_string(),
//         version: "[compile error]".to_string(),
//         source: source.to_string(),
//         ..Default::default()
//     }
// }

pub async fn make_doc_for_packer_error(source: &str, error: PackerError) -> ExecDoc<'static> {
    todo!()
    // let comp_doc = Compiler::default().create_empty_doc_for_packer_error(error);
    // let project = make_project_for_error(source);
    // let exec_doc = comp_doc.exec(&project).await;
    // ExecDoc {
    //     preface: exec_doc.preface,
    //     route: exec_doc.route,
    //     diagnostics: exec_doc.diagnostics,
    //     project: Cow::Owned(project),
    // }
}

pub struct CompilerContext<L> where L: Loader {
    pub start_time: Instant,
    pub project_resource: Resource<'static, L>,
    pub setting: Setting,
    pub phase0: Phase0,
}

impl<L> CompilerContext<L> where L: Loader {
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
