use std::collections::HashMap;

use celerctypes::ExecDoc;
use derivative::Derivative;

use crate::comp::{Compiler, CompilerError};
use crate::lang::Preset;
use crate::metrics::CompilerMetrics;
use crate::pack::{
    self, PackedProject, PackerError, ResourceLoader, ResourcePath, ResourceResolver,
};
use crate::plug::run_plugins;

/// Output of the compiler API
pub struct CompilerOutput {
    /// The final document to be rendered
    pub exec_doc: ExecDoc,
    /// The metadata of the compiler
    pub metadata: CompilerMetadata,
    /// Metrics collected during compilation
    pub metrics: CompilerMetrics,
}

/// Metadata of the compiler
///
/// This is information needed during compilation,
/// but not needed to render the route.
/// IDEs may also find this useful to provide auto-complete, etc.
#[derive(Default, Debug, Clone)]
pub struct CompilerMetadata {
    pub presets: HashMap<String, Preset>,
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

pub async fn compile_project(
    project: &ResourcePath,
    resolver: &dyn ResourceResolver,
    loader: &dyn ResourceLoader,
    setting: &Setting,
) -> CompilerOutput {
    let mut metrics = CompilerMetrics::new();
    let pack_result = pack::pack_project(project, resolver, loader, setting).await;
    metrics.pack_done();

    let (comp_doc, comp_meta) = match pack_result {
        Ok(PackedProject {
            route_metadata,
            compiler_metadata,
            route,
        }) => {
            let compiler = Compiler {
                meta: compiler_metadata,
                color: route_metadata.map.initial_color.clone(),
                coord: route_metadata.map.initial_coord.clone(),
                project: route_metadata,
                max_preset_depth: setting.max_preset_ref_depth,
            };

            compiler.comp_doc(route).await
        }
        Err(error) => {
            let compiler = Compiler::default();
            compiler
                .create_empty_doc_for_error(&[CompilerError::PackerErrors(vec![error])])
                .await
        }
    };

    metrics.comp_done();

    let comp_doc = run_plugins(comp_doc);
    metrics.plug_done();

    let exec_doc = comp_doc.exec().await;
    metrics.exec_done();

    CompilerOutput {
        exec_doc,
        metadata: comp_meta,
        metrics,
    }
}
