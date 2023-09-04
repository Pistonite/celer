use celerctypes::{ExecDoc, RouteMetadata, DocPoorText};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod comp;
mod exec;
mod json;
mod lang;
mod pack;
mod plug;

mod metrics;
mod util;

use comp::{CompLine, CompilerError};
use lang::Preset;
use metrics::CompilerMetrics;
use pack::{PackedProject, PackerError, ResourceLoader, ResourcePath, ResourceResolver};

pub struct CompilerOutput {
    /// The final document to be rendered
    pub exec_doc: ExecDoc,
    /// The metadata of the compiler
    pub metadata: Option<CompilerMetadata>,
    /// Metrics collected during compilation
    pub metrics: CompilerMetrics,
}

pub async fn compile_project(
    project: &ResourcePath,
    resolver: &dyn ResourceResolver,
    loader: &dyn ResourceLoader,
    setting: &Setting,
) -> Result<CompilerOutput, Error> {
    let PackedProject {
        route_metadata,
        compiler_metadata,
        route,
    } = pack::pack_project(project, resolver, loader, setting).await?;
    todo!()
}

/// Metadata of the compiler
///
/// This is information needed during compilation,
/// but not needed to render the route.
/// IDEs may also find this useful to provide auto-complete, etc.
#[derive(Default, Debug, Clone)]
pub struct CompilerMetadata {
    pub presets: HashMap<String, Preset>,
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

    /// The default color of lines
    #[derivative(Default(value = "\"#38f\".to_string()"))]
    pub default_line_color: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pack: {0}")]
    Pack(#[from] PackerError),

    #[error("comp: {0}")]
    Comp(#[from] CompilerError),
}
