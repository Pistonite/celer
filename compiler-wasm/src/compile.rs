use std::cell::RefCell;

use celerc::env::RefCounted;
use celerc::res::{Resource, ResPath};
use instant::Instant;
use log::info;
use wasm_bindgen::prelude::*;

use celerc::pack::PackError;
use celerc::{PreparedContext, Compiler, CompDoc, ContextBuilder};

use crate::interop::OpaqueExecDoc;
use crate::loader::{LoaderInWasm, LoadFileOutput, self};

thread_local! {
    static CACHED_COMPILER_CONTEXT: RefCell<Option<PreparedContext<LoaderInWasm>>> = RefCell::new(None);
    static CACHED_COMPILER_ENTRY_PATH: RefCell<Option<String>> = RefCell::new(None);
}

/// Compile a document from web editor
pub async fn compile_document(
    entry_path: Option<String>,
    use_cache: bool,
) -> Result<OpaqueExecDoc, JsValue> {
    if use_cache && is_cached_compiler_valid(entry_path.as_ref()).await {
        let cached_context = CACHED_COMPILER_CONTEXT.with_borrow_mut(|x| x.take());

        if let Some(context) = cached_context {
            info!("using cached compiler context");
            let start_time = Instant::now();
            let result = match context.create_compiler(Some(start_time)).await {
                Ok(x) => compile_with_compiler(x).await,
                Err(e) => compile_with_pack_error(&context, e).await,
            };
            CACHED_COMPILER_CONTEXT.with_borrow_mut(|x| x.replace(context));
            return result;
        }
    }

    let mut context_builder = new_context_builder();
    if entry_path.is_some() {
        context_builder = context_builder.entry_point(entry_path);
    }
    let start_time = Instant::now();
    let prepared_context = match context_builder.build_context().await {
        Ok(x) => x,
        Err(e) => {
            let comp_doc = CompDoc::from_prep_error(e, start_time);
            let exec_doc = comp_doc.execute().await;
            return OpaqueExecDoc::wrap(exec_doc);
        }
    };

    let compiler_result = prepared_context.create_compiler(None).await;
    let output = match compiler_result {
        Ok(x) => compile_with_compiler(x).await,
        Err(e) => compile_with_pack_error(&prepared_context, e).await,
    };

    CACHED_COMPILER_CONTEXT.with_borrow_mut(|x| x.replace(prepared_context));

    output
}


async fn is_cached_compiler_valid(entry_path: Option<&String>) -> bool {
    // TODO #173: better cache invalidation when local config changes

    let root_project_result = loader::load_file_check_changed("project.yaml").await;
    if !matches!(root_project_result, Ok(LoadFileOutput::NotModified)) {
        info!("root project.yaml is modified");
        return false;
    }
    if let Some(entry_path) = entry_path {
        let entry_path = match entry_path.strip_prefix('/') {
            Some(x) => x,
            None => entry_path,
        };
        let entry_result = loader::load_file_check_changed(entry_path).await;
        if !matches!(entry_result, Ok(LoadFileOutput::NotModified)) {
            info!("entry project.yaml is modified");
            return false;
        }
    }
    let is_same = CACHED_COMPILER_ENTRY_PATH.with_borrow(|x| x.as_ref() == entry_path);
    if !is_same {
        info!("entry changed");
        return false;
    }
    
    true
}

async fn compile_with_pack_error(context: &PreparedContext<LoaderInWasm>, error: PackError) -> Result<OpaqueExecDoc, JsValue> {
    let comp_doc = CompDoc::from_diagnostic(error, context.create_compile_context(None));
    let exec_doc = comp_doc.execute().await;
    OpaqueExecDoc::wrap(exec_doc)
}

async fn compile_with_compiler(compiler: Compiler<'_>) -> Result<OpaqueExecDoc, JsValue> {
    let comp_doc = compiler.compile().await;
    let exec_doc = comp_doc.execute().await;
    OpaqueExecDoc::wrap(exec_doc)
}

/// Create a context builder that corresponds to the root project.yaml
pub fn new_context_builder() -> ContextBuilder<LoaderInWasm> {
    let source = "Web Editor".to_string();
    let project_res = Resource::new(
        ResPath::Local("project.yaml".into()),
        RefCounted::new(LoaderInWasm),
    );
    ContextBuilder::new(source, project_res)
}
