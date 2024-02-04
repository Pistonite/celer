use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use celerc::lang::DocDiagnostic;
use celerc::prep::PrepResult;
use instant::Instant;
use log::{error, info};
use wasm_bindgen::prelude::*;

use celerc::pack::PackError;
use celerc::{
    CompDoc, CompileContext, Compiler, ContextBuilder, ExecContext, PluginOptions, PreparedContext,
};

use crate::interop::OpaqueExpoContext;
use crate::loader::{self, LoadFileOutput, LoaderInWasm};
use crate::plugin;

mod cache;
use cache::CachedContextGuard;
mod export;
pub use export::export_document;

/// Compile a document from web editor
pub async fn compile_document(
    entry_path: Option<String>,
    use_cache: bool,
) -> Result<OpaqueExpoContext, JsValue> {
    let plugin_options = match plugin::get_plugin_options() {
        Ok(x) => x,
        Err(message) => {
            let message = format!("Failed to load user plugin options: {message}");
            error!("{message}");
            let diagnostic = DocDiagnostic::error(&message, "web-editor");
            let exec_ctx = ExecContext::from_diagnostic(diagnostic);
            return OpaqueExpoContext::try_from(exec_ctx.prepare_exports());
        }
    };

    if use_cache {
        if let Some(guard) = CachedContextGuard::acquire(entry_path.as_ref()).await {
            info!("using cached compiler context");
            let start_time = Instant::now();
            return compile_in_context(guard.as_ref(), Some(start_time), plugin_options).await;
        }
    }



    // if use_cache && is_cached_compiler_valid(entry_path.as_ref()).await {
    //     let cached_context = CACHED_COMPILER_CONTEXT.with_borrow_mut(|x| x.take());
    //
    //     if let Some(prep_ctx) = cached_context {
    //         info!("using cached compiler context");
    //         let start_time = Instant::now();
    //         let result = compile_in_context(&prep_ctx, Some(start_time), plugin_options).await;
    //         CACHED_COMPILER_CONTEXT.with_borrow_mut(|x| x.replace(prep_ctx));
    //         return result;
    //     }
    // }

    // create a new context
    info!("creating new compiler context");
    let start_time = Instant::now();

    let prep_ctx = match new_context(entry_path).await {
        Ok(x) => x,
        Err(e) => {
            let comp_doc = CompDoc::from_prep_error(e, start_time);
            let exec_context = comp_doc.execute().await;
            return OpaqueExpoContext::try_from(exec_context.prepare_exports());
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

// async fn is_cached_compiler_valid(entry_path: Option<&String>) -> bool {
//     // TODO #173: better cache invalidation when local config changes
//
//     let root_project_result = loader::load_file_check_changed("project.yaml").await;
//     if !matches!(root_project_result, Ok(LoadFileOutput::NotModified)) {
//         info!("root project.yaml is modified");
//         return false;
//     }
//     if let Some(entry_path) = entry_path {
//         let entry_path = match entry_path.strip_prefix('/') {
//             Some(x) => x,
//             None => entry_path,
//         };
//         let entry_result = loader::load_file_check_changed(entry_path).await;
//         if !matches!(entry_result, Ok(LoadFileOutput::NotModified)) {
//             info!("entry project.yaml is modified");
//             return false;
//         }
//     }
//     let is_same = CACHED_COMPILER_ENTRY_PATH.with_borrow(|x| x.as_ref() == entry_path);
//     if !is_same {
//         info!("entry changed");
//         return false;
//     }
//
//     true
// }

async fn compile_in_context(
    prep_ctx: &PreparedContext<LoaderInWasm>,
    start_time: Option<Instant>,
    plugin_options: Option<PluginOptions>,
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
    OpaqueExpoContext::try_from(exec_ctx.prepare_exports())
}

async fn compile_with_compiler(compiler: Compiler<'_>) -> Result<OpaqueExpoContext, JsValue> {
    let comp_doc = compiler.compile().await;
    let exec_ctx = comp_doc.execute().await;
    OpaqueExpoContext::try_from(exec_ctx.prepare_exports())
}

/// Create a context builder that corresponds to the root project.yaml
pub fn new_context_builder() -> ContextBuilder<LoaderInWasm> {
    let source = "Web Editor".to_string();
    let project_res = super::get_root_project_resource();
    ContextBuilder::new(source, project_res)
}
