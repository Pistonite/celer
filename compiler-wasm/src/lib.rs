use std::borrow::Cow;

use celerc::types::EntryPointsSorted;
use js_sys::Function;
use log::info;
use wasm_bindgen::prelude::*;

use celerc::pack::{LocalResourceResolver, Resource, ResourcePath};
use celerc::util::{Marc, Path};
use celerc::Setting;

mod interop;
use interop::OpaqueExecDoc;
mod loader_file;
mod loader_url;
mod logger;

/// Initialize
#[wasm_bindgen]
pub fn init(
    site_origin: String,
    info_fn: Function,
    warn_fn: Function,
    error_fn: Function,
    load_file: Function,
    load_url: Function,
) {
    let _ = logger::bind(info_fn, warn_fn, error_fn);
    info!("initializing compiler...");
    loader_file::bind(load_file);
    loader_url::bind(load_url);
    let _ = celerc::util::init_site_origin(site_origin);

    info!("compiler initialized");
}

/// Return the entry points (only paths, not aliases) defined in the root project
///
/// If there is any error, this returns 0 entry points
#[wasm_bindgen]
pub async fn get_entry_points() -> Result<EntryPointsSorted, JsValue> {
    let resource = create_root_resource();
    let project_resource = match celerc::resolve_project(&resource).await {
        Ok(x) => x,
        Err(_) => {
            return Ok(Default::default());
        }
    };

    let entry_points = match celerc::prepare_entry_points(&project_resource).await {
        Ok(x) => x.path_only().into(),
        Err(_) => Default::default(),
    };

    Ok(entry_points)
}

/// Compile a document from web editor
#[wasm_bindgen]
pub async fn compile_document(entry_path: Option<String>) -> Result<OpaqueExecDoc, JsValue> {
    let root_resource = create_root_resource();
    let (allow_redirect, project_resource_result) = match entry_path.as_ref() {
        None => (
            // allow redirect to default entry point in root project.yaml
            true,
            celerc::resolve_project(&root_resource).await,
        ),
        Some(path) => (
            false,
            celerc::resolve_absolute(&root_resource, path.to_string()).await,
        ),
    };
    let source_name = match entry_path {
        Some(path) => Cow::Owned(path),
        None => Cow::Borrowed("(default)"),
    };
    let project_resource = match project_resource_result {
        Ok(x) => x,
        Err(e) => {
            let x = celerc::make_doc_for_packer_error(&source_name, e).await;
            return OpaqueExecDoc::wrap(x);
        }
    };
    let setting = Setting::default();
    // TODO #86 cache this
    let context =
        match celerc::prepare_compiler(&source_name, project_resource, setting, allow_redirect)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                let x = celerc::make_doc_for_packer_error(&source_name, e).await;
                return OpaqueExecDoc::wrap(x);
            }
        };

    let x = context.compile().await;
    OpaqueExecDoc::wrap(x)
}

/// Create a resource that corresponds to the project root
fn create_root_resource() -> Resource {
    Resource::new(
        ResourcePath::FsPath(Path::new()),
        loader_file::new_loader(),
        loader_url::new_loader(),
        Marc::new(LocalResourceResolver(Path::new())),
    )
}
