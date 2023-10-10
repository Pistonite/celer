use crate::pack::{self, Resource, PackerResult, Phase0};

use super::Setting;

/// Result of packing a project
pub struct CompilerContext {
    project_resource: Resource,
    setting: Setting,
    phase0: Phase0,
}

//TODO #25: Entry point for just getting the metadata and not route

/// Prepare the compiler context to compile a project.
///
/// This is referred to as the "pack phase 0". The output of this phase
/// is used for further compilation. The compiler does not consume
/// the pack phase 0 output, so the output can be cached to speed up
/// future iterations
pub async fn prepare(source: &str, project_resource: Resource, setting: Setting) -> PackerResult<CompilerContext> {
    let mut phase0 = pack::pack_phase0(source, &project_resource, &setting).await?;
    for plugin in &phase0.meta.plugins {
        plugin.on_pre_compile(&mut phase0.project).await?;
    }
    Ok(CompilerContext {
        project_resource,
        setting,
        phase0,
    })
}
