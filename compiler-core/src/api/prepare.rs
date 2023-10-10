use instant::Instant;

use crate::pack::{self, Resource, PackerResult, Phase0};

use super::{Setting, CompilerContext};

//TODO #25: Entry point for just getting the metadata and not route

/// Prepare the compiler context to compile a project.
///
/// This is referred to as the "pack phase 0". The output of this phase
/// is used for further compilation. The compiler does not consume
/// the pack phase 0 output, so the output can be cached to speed up
/// future iterations
pub async fn prepare(source: &str, project_resource: Resource, setting: Setting) -> PackerResult<CompilerContext> {
    let start_time = Instant::now();
    let phase0 = pack::pack_phase0(source, &project_resource, &setting).await?;
    let mut context = CompilerContext {
        start_time,
        project_resource,
        setting,
        phase0,
    };
    for plugin in &context.phase0.meta.plugins {
        plugin.create_runtime(&context).on_pre_compile(&mut context.phase0.project).await?;
    }
    Ok(context)
}
