use interop::OpaqueExecDoc;
use js_sys::Function;
use log::info;
use wasm_bindgen::prelude::*;

use celerc::Setting;
use celerc::pack::{LocalResourceResolver, Resource, ResourcePath};
use celerc::util::{Path, Marc};

mod interop;
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
    load_url: Function
) {
    let _ = logger::bind(info_fn, warn_fn, error_fn);
    info!("initializing compiler...");
    loader_file::bind(load_file);
    loader_url::bind(load_url);
    let _ = celerc::util::init_site_origin(site_origin);

    info!("compiler initialized");
}

const SOURCE_NAME: &str = "(local)";

/// Compile a document from web editor
///
/// Return undefined if the compilation was interrupted
/// TODO #78: undefined no longer needed
#[wasm_bindgen]
pub async fn compile_document() -> Result<OpaqueExecDoc, JsValue> {
    let resource = create_root_resource();
    let setting = Setting::default();
    let project_resource = match celerc::resolve_project(&resource).await {
        Ok(x) => x,
        Err(e) => {
            let x = 
            celerc::make_doc_for_packer_error(SOURCE_NAME, e).await;
            return OpaqueExecDoc::wrap(x);
        }
    };
    // TODO #86 cache this
    let context = match celerc::prepare(SOURCE_NAME, project_resource, setting).await {
        Ok(x) => x,
        Err(e) => {
            let x = 
            celerc::make_doc_for_packer_error(SOURCE_NAME, e).await;
            return OpaqueExecDoc::wrap(x);
        }
    };

    let x = context.compile().await;
            return OpaqueExecDoc::wrap(x);
}

/// Request current compilation be cancelled
#[wasm_bindgen]
pub fn request_cancel() {
    celerc::cancel_current_compilation();
}

/// Create a resource that corresponds to the project root
fn create_root_resource() -> Resource {
    Resource::new(
        ResourcePath::FsPath(Path::new()),
        loader_file::new_loader(),
        loader_url::new_loader(),
        Marc::new(LocalResourceResolver(Path::new()))
    )
}

