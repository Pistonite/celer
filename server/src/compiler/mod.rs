use celerc::pack::PackError;
use instant::Instant;

use celerc::plugin::Options as PluginOptions;
use celerc::res::{self, ResPath, ResResult, Resource};
use celerc::{CompDoc, CompileContext, Compiler, ContextBuilder, ExpoContext, PrepCtx};

mod loader;
pub use loader::*;
mod export;
pub use export::*;
mod plugin;
pub use plugin::*;
mod prepctx_cache;
pub use prepctx_cache::*;
mod resource_cache;
pub use resource_cache::*;

/// Create a context builder for a project
pub fn new_context_builder(
    owner: &str,
    repo: &str,
    reference: Option<&str>,
) -> ResResult<ContextBuilder<ServerResourceLoader>> {
    let resource = new_root_resource(owner, repo, reference);
    let source = format!("{}/{}/{}", owner, repo, reference.unwrap_or("main"));
    Ok(ContextBuilder::new(source, resource?))
}

/// Create a root resource for a project
pub fn new_root_resource(
    owner: &str,
    repo: &str,
    reference: Option<&str>,
) -> ResResult<Resource<'static, ServerResourceLoader>> {
    let loader = loader::get_loader();
    let base_url = res::base_url(owner, repo, reference);
    let res_path = ResPath::new_remote_unchecked(base_url, "project.yaml");
    Ok(Resource::new(res_path, loader?))
}

pub async fn compile(
    prep_ctx: &PrepCtx<ServerResourceLoader>,
    start_time: Option<Instant>,
    plugin_options: Option<PluginOptions>,
) -> ExpoContext {
    let mut comp_ctx = prep_ctx.new_compilation(start_time).await;
    if let Err(e) = comp_ctx.configure_plugins(plugin_options).await {
        return compile_with_pack_error(comp_ctx, e).await;
    }
    match prep_ctx.create_compiler(comp_ctx).await {
        Ok(x) => compile_with_compiler(x).await,
        Err((e, comp_ctx)) => compile_with_pack_error(comp_ctx, e).await,
    }
}

async fn compile_with_pack_error(context: CompileContext<'_>, error: PackError) -> ExpoContext {
    let comp_doc = CompDoc::from_diagnostic(error, context);
    let exec_ctx = comp_doc.execute().await;
    exec_ctx.prepare_exports().await
}

async fn compile_with_compiler(compiler: Compiler<'_>) -> ExpoContext {
    let comp_doc = compiler.compile().await;
    let exec_ctx = comp_doc.execute().await;
    exec_ctx.prepare_exports().await
}
