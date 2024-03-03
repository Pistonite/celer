use celerc::pack::PackError;
use instant::Instant;

use celerc::res::{self, ResPath, Resource};
use celerc::{
    CompDoc, CompileContext, Compiler, ContextBuilder, ExpoContext, PluginOptions, PreparedContext,
};

mod loader;
pub use loader::*;
mod cache;
pub use cache::*;
mod export;
pub use export::*;
mod plugin;
pub use plugin::*;

/// Create a context builder for a project
pub fn new_context_builder(
    owner: &str,
    repo: &str,
    reference: Option<&str>,
) -> ContextBuilder<ServerResourceLoader> {
    let resource = new_root_resource(owner, repo, reference);
    let source = format!("{}/{}/{}", owner, repo, reference.unwrap_or("main"));
    ContextBuilder::new(source, resource)
}

/// Create a root resource for a project
pub fn new_root_resource(
    owner: &str,
    repo: &str,
    reference: Option<&str>,
) -> Resource<'static, ServerResourceLoader> {
    let loader = loader::get_loader();
    let base_url = res::base_url(owner, repo, reference);
    let res_path = ResPath::new_remote_unchecked(base_url, "project.yaml");
    Resource::new(res_path, loader)
}

pub async fn compile(
    prep_ctx: &PreparedContext<ServerResourceLoader>,
    start_time: Option<Instant>,
    plugin_options: Option<PluginOptions>,
) -> ExpoContext {
    let mut comp_ctx = prep_ctx.new_compilation(start_time).await;
    match comp_ctx.configure_plugins(plugin_options).await {
        Err(e) => compile_with_pack_error(comp_ctx, e).await,
        Ok(_) => match prep_ctx.create_compiler(comp_ctx).await {
            Ok(x) => compile_with_compiler(x).await,
            Err((e, comp_ctx)) => compile_with_pack_error(comp_ctx, e).await,
        },
    }
}

async fn compile_with_pack_error(context: CompileContext<'_>, error: PackError) -> ExpoContext {
    let comp_doc = CompDoc::from_diagnostic(error, context);
    let exec_ctx = comp_doc.execute().await;
    exec_ctx.prepare_exports()
}

async fn compile_with_compiler(compiler: Compiler<'_>) -> ExpoContext {
    let comp_doc = compiler.compile().await;
    let exec_ctx = comp_doc.execute().await;
    exec_ctx.prepare_exports()
}
