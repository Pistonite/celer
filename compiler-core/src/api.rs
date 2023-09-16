use std::collections::HashMap;
use log::{error, info};

use celerctypes::ExecDoc;
use derivative::Derivative;

use crate::comp::{Compiler, CompilerError, CompDoc};
use crate::lang::Preset;
use crate::metrics::CompilerMetrics;
use crate::pack::{
    self, PackedProject,     Resource, ValidUse, PackerError, PackerResult,
};
use crate::plug::run_plugins;

use crate::util::cancel;

/// Output of the compiler API
#[derive(Debug, Clone)]
pub enum CompilerOutput {
    Ok {
        /// The final document to be rendered
        exec_doc: ExecDoc,
        /// The metadata of the compiler
        metadata: CompilerMetadata,
        /// Metrics collected during compilation
        metrics: CompilerMetrics,
    },
    Cancelled,
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

/// Entry point for the compiler
///
/// The root resource should contain the project.yaml file when resolving "./project.yaml"
pub async fn compile(root_resource: &Resource, setting: &Setting) -> CompilerOutput {
    info!("compiling document...");
    let mut metrics = CompilerMetrics::new();
    let (comp_doc, comp_meta) = match pack_phase(root_resource, setting).await {
        Err(e) => {
            if e.is_cancel() {
                return CompilerOutput::Cancelled;
            }
            error!("pack phase failed.");
            Compiler::default().create_empty_doc_for_packer_error(e).await
        }
        Ok(packed_project) => {
            let ms = metrics.pack_done();
            info!("pack phase done in {ms}ms");

            match comp_phase(packed_project, setting).await {
                Err(e) => {
                    if !e.is_cancel() {
                        error!("unexpected compiler error during comp phase! Compiler errors should be surfaced through the diagnostic API");
                    }
                    return CompilerOutput::Cancelled;
                }
                Ok(x) => x
            }
        },
    };
    let ms = metrics.comp_done();
    info!("comp phase done in {ms}ms");

    let comp_doc = run_plugins(comp_doc);
    let ms = metrics.plug_done();
    info!("plug phase done in {ms}ms");

    let exec_doc = match comp_doc.exec().await {
        Err(e) => {
            if !e.is_cancel() {
                error!("unexpected compiler error during exec phase! Compiler errors should be surfaced through the diagnostic API");
            }
            return CompilerOutput::Cancelled;
        }
        Ok(x) => x
    };
    let ms = metrics.exec_done();
    info!("exec phase done in {ms}ms");

    CompilerOutput::Ok {
        exec_doc,
        metadata: comp_meta,
        metrics,
    }
}

async fn pack_phase(root_resource: &Resource, setting: &Setting) -> PackerResult<PackedProject> {
    let project_ref = ValidUse::Relative("./project.yaml".to_string());
    let project_resource = match root_resource.resolve(&project_ref).await {
        Ok(resource) => resource,
        Err(_) => {
            error!("fail to resolve project.yaml");
            return Err(PackerError::InvalidProject.into());
        }
    };

    pack::pack_project(&project_resource, setting).await
}

async fn comp_phase(packed_project: PackedProject, setting: &Setting) -> Result<(CompDoc, CompilerMetadata), CompilerError> {
    let PackedProject {
        route_metadata,
        compiler_metadata,
        route,
    } = packed_project;

    let compiler = Compiler {
        meta: compiler_metadata,
        color: route_metadata.map.initial_color.clone(),
        coord: route_metadata.map.initial_coord.clone(),
        project: route_metadata,
        max_preset_depth: setting.max_preset_ref_depth,
    };

    compiler.comp_doc(route).await
}

#[cfg(feature = "wasm")]
#[inline]
pub fn cancel_current_compilation() {
    cancel();
    info!("cancel requested");
}
