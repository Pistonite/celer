use instant::Instant;
use log::{error, info};

use celerc::{Compiler, ExpoDoc, ExportRequest, PreparedContext};
use celerc::pack::PackError;
use celerc::plugin;

use crate::loader::LoaderInWasm;

use super::CachedContextGuard;

pub async fn export_document(
    entry_path: Option<String>,
    use_cache: bool,
    req: ExportRequest,
) -> ExpoDoc {
    info!("exporting document");
    let plugin_options = match crate::plugin::get_plugin_options() {
        Ok(x) => x,
        Err(message) => {
            let message = format!("Failed to load user plugin options: {message}");
            error!("{message}");
            return ExpoDoc::Error(message);
        }
    };

    if use_cache {
        if let Some(guard) = CachedContextGuard::acquire(entry_path.as_ref()).await {
            info!("using cached compiler context");
            let start_time = Instant::now();
            return export_in_context(guard.as_ref(), Some(start_time), plugin_options, req).await;
        }
    }

    // create a new context
    info!("creating new compiler context");
    let start_time = Instant::now();
    let prep_ctx = match super::new_context(entry_path).await {
        Ok(x) => x,
        Err(e) => {
            return ExpoDoc::Error(e.to_string());
        }
    };
    let guard = CachedContextGuard::new(prep_ctx);
    export_in_context(guard.as_ref(), Some(start_time), plugin_options, req).await
}

async fn export_in_context(
    prep_ctx: &PreparedContext<LoaderInWasm>,
    start_time: Option<Instant>,
    plugin_options: Option<plugin::Options>,
    req: ExportRequest,
) -> ExpoDoc {
    let mut comp_ctx = prep_ctx.new_compilation(start_time).await;
    match comp_ctx.configure_plugins(plugin_options).await {
        Err(e) => export_with_pack_error(e),
        Ok(_) => match prep_ctx.create_compiler(comp_ctx).await {
            Ok(x) => export_with_compiler(x, req).await,
            Err((e, _)) => export_with_pack_error(e),
        },
    }
}

fn export_with_pack_error(error: PackError) -> ExpoDoc {
    ExpoDoc::Error(error.to_string())
}

async fn export_with_compiler(compiler: Compiler<'_>, req: ExportRequest) -> ExpoDoc {
    let mut comp_doc = compiler.compile().await;
    if let Some(expo_doc) = comp_doc.run_exporter(&req).await {
        return expo_doc;
    }
    let exec_ctx = comp_doc.execute().await;
    exec_ctx.run_exporter(req).await
}
