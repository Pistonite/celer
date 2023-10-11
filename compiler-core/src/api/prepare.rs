use instant::Instant;
#[cfg(not(feature = "no-async-send"))]
use tokio::task::LocalSet;

#[cfg(not(feature = "no-async-send"))]
use crate::pack::PackerError;
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

    // TODO #78: I don't know why it is fine in compile but not here
    // however it is fine in WASM so we can keep it this way until Alpha 2

    #[cfg(feature = "no-async-send")]
    for plugin in &plugins {
        plugin
            .create_runtime(&context)
            .on_pre_compile(&mut context)
            .await?;
    }
    #[cfg(not(feature = "no-async-send"))]
    let mut context = {
        let local = LocalSet::new();
        let runtimes = plugins
            .iter()
            .map(|plugin| plugin.create_runtime(&context))
            .collect::<Vec<_>>();
        local
            .run_until(async move {
                for mut rt in runtimes {
                    rt.on_pre_compile(&mut context).await?;
                }
                Ok::<CompilerContext, PackerError>(context)
            })
            .await?
    };
    // put the plugins back, discard changes made to the plugins
    std::mem::swap(&mut context.phase0.meta.plugins, &mut plugins);
    Ok(context)
}
