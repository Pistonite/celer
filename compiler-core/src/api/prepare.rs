use instant::Instant;
#[cfg(feature = "native")]
use tokio::task::LocalSet;

#[cfg(feature = "native")]
use crate::pack::PackerError;
use crate::pack::{self, PackerResult, Resource};
use crate::types::EntryPoints;

use super::{CompilerContext, Setting};

//TODO #25: Entry point for just getting the metadata and not route

/// Prepare the compiler context to compile a project.
///
/// This is referred to as the "pack phase 0". The output of this phase
/// is used for further compilation. The compiler does not consume
/// the pack phase 0 output, so the output can be cached to speed up
/// future iterations
pub async fn prepare_compiler(
    source: &str,
    project_resource: Resource,
    setting: Setting,
    redirect_to_default_entry_point: bool,
) -> PackerResult<CompilerContext> {
    let start_time = Instant::now();
    let mut phase0 = pack::pack_phase0(
        source,
        &project_resource,
        &setting,
        redirect_to_default_entry_point,
    )
    .await?;
    // take the plugins out to run the pre compile phase of the plugins
    let mut plugins = std::mem::take(&mut phase0.meta.plugins);
    let mut context = CompilerContext {
        start_time,
        project_resource,
        setting,
        phase0,
    };

    // take the presets out for optimization
    let mut presets = std::mem::take(&mut context.phase0.meta.presets);

    while let Some((name, mut preset)) = presets.pop_first() {
        preset
            .optimize(&mut presets, &mut context.phase0.meta.presets)
            .await;
        context.phase0.meta.presets.insert(name, preset);
    }

    // TODO #25: I don't know why it is fine in compile but not here
    // however it is fine in WASM so we can keep it this way until compiler runs on the server

    #[cfg(feature = "wasm")]
    for plugin in &plugins {
        plugin
            .create_runtime(&context)
            .on_pre_compile(&mut context)
            .await?;
    }
    #[cfg(feature = "native")]
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

/// Get the entry points of a project
///
/// If the project object is valid but it does not define the `entry-points` property
/// or if the `entry-points` property is empty, returns `EntryPoints` with 0 entries.
///
/// If the project object or the `entry-points` property is invalid, returns error.
pub async fn prepare_entry_points(project_resource: &Resource) -> PackerResult<EntryPoints> {
    pack::pack_project_entry_points(project_resource).await
}
