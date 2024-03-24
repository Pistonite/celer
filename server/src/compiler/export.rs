use instant::Instant;

use celerc::{Compiler, ExpoDoc, ExportRequest, PreparedContext};
use celerc::pack::PackError;
use celerc::plugin;

use super::ServerResourceLoader;

pub async fn export(
    prep_ctx: &PreparedContext<ServerResourceLoader>,
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
