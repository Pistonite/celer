use instant::Instant;

use crate::pack::{self, PackerResult, Resource};

use super::{CompilerContext, Setting};

//TODO #25: Entry point for just getting the metadata and not route

/// Prepare the compiler context to compile a project.
///
/// This is referred to as the "pack phase 0". The output of this phase
/// is used for further compilation. The compiler does not consume
/// the pack phase 0 output, so the output can be cached to speed up
/// future iterations
pub async fn prepare(
    source: &str,
    project_resource: Resource,
    setting: Setting,
) -> PackerResult<CompilerContext> {
    let start_time = Instant::now();
    let mut phase0 = pack::pack_phase0(source, &project_resource, &setting).await?;
    // take the plugins out to run the pre compile phase of the plugins
    let mut plugins = std::mem::take(&mut phase0.meta.plugins);
    let mut context = CompilerContext {
        start_time,
        project_resource,
        setting,
        phase0,
    };
    for plugin in &plugins {
        plugin
            .create_runtime(&context)
            .on_pre_compile(&mut context)
            .await?;
    }
    // put the plugins back, discard changes made to the plugins
    std::mem::swap(&mut context.phase0.meta.plugins, &mut plugins);
    Ok(context)
}
