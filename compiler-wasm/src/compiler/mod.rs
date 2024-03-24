use celerc::lang::DocDiagnostic;
use celerc::prep::PrepResult;
use instant::Instant;
use log::{error, info};
use wasm_bindgen::prelude::*;

use celerc::pack::PackError;
use celerc::plugin;
use celerc::{CompDoc, CompileContext, Compiler, ContextBuilder, ExecContext, PreparedContext};

use crate::interop::OpaqueExpoContext;
use crate::loader::LoaderInWasm;

mod cache;
use cache::CachedContextGuard;
mod export;
pub use export::export_document;

/// Compile a document from web editor
pub async fn compile_document(
    entry_path: Option<String>,
    use_cache: bool,
) -> Result<OpaqueExpoContext, JsValue> {
    let plugin_options = match crate::plugin::get_plugin_options() {
        Ok(x) => x,
        Err(message) => {
            let message = format!("Failed to load user plugin options: {message}");
            error!("{message}");
            let diagnostic = DocDiagnostic::error(&message, "web-editor");
            let exec_ctx = ExecContext::from_diagnostic(diagnostic);
            return OpaqueExpoContext::try_from(exec_ctx.prepare_exports().await);
        }
    };

    if use_cache {
        if let Some(guard) = CachedContextGuard::acquire(entry_path.as_ref()).await {
            info!("using cached compiler context");
            let start_time = Instant::now();
            return compile_in_context(guard.as_ref(), Some(start_time), plugin_options).await;
        }
    }

    // create a new context
    info!("creating new compiler context");
    let start_time = Instant::now();

    let prep_ctx = match new_context(entry_path).await {
        Ok(x) => x,
        Err(e) => {
            let comp_doc = CompDoc::from_prep_error(e, start_time);
            let exec_context = comp_doc.execute().await;
            return OpaqueExpoContext::try_from(exec_context.prepare_exports().await);
        }
    };
    let guard = CachedContextGuard::new(prep_ctx);

    compile_in_context(guard.as_ref(), None, plugin_options).await
}

pub async fn new_context(entry_path: Option<String>) -> PrepResult<PreparedContext<LoaderInWasm>> {
    let mut context_builder = new_context_builder();
    if entry_path.is_some() {
        context_builder = context_builder.entry_point(entry_path);
    }
    context_builder.build_context().await
}

async fn compile_in_context(
    prep_ctx: &PreparedContext<LoaderInWasm>,
    start_time: Option<Instant>,
    plugin_options: Option<plugin::Options>,
) -> Result<OpaqueExpoContext, JsValue> {
    let mut comp_ctx = prep_ctx.new_compilation(start_time).await;
    match comp_ctx.configure_plugins(plugin_options).await {
        Err(e) => compile_with_pack_error(comp_ctx, e).await,
        Ok(_) => match prep_ctx.create_compiler(comp_ctx).await {
            Ok(x) => compile_with_compiler(x).await,
            Err((e, comp_ctx)) => compile_with_pack_error(comp_ctx, e).await,
        },
    }
}

async fn compile_with_pack_error(
    context: CompileContext<'_>,
    error: PackError,
) -> Result<OpaqueExpoContext, JsValue> {
    let comp_doc = CompDoc::from_diagnostic(error, context);
    let exec_ctx = comp_doc.execute().await;
    OpaqueExpoContext::try_from(exec_ctx.prepare_exports().await)
}

async fn compile_with_compiler(compiler: Compiler<'_>) -> Result<OpaqueExpoContext, JsValue> {
    let comp_doc = compiler.compile().await;
    let exec_ctx = comp_doc.execute().await;
    OpaqueExpoContext::try_from(exec_ctx.prepare_exports().await)
}

/// Create a context builder that corresponds to the root project.yaml
pub fn new_context_builder() -> ContextBuilder<LoaderInWasm> {
    let source = "Web Editor".to_string();
    let project_res = super::get_root_project_resource();
    ContextBuilder::new(source, project_res)
}
