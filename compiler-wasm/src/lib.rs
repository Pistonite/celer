use celerc::api::Setting;
use celerc::pack::LocalResourceResolver;
use celerc::pack::Resource;
use celerc::pack::ResourcePath;
use celerc::types::GameCoord;
use celerc::types::ExecDoc;
use celerc::util::Marc;
use celerc::util::Path;
use interop::OpaqueExecDoc;
use js_sys::Function;
use log::info;
use tsify::Tsify;
use tsify::declare;
use wasm_bindgen::__rt::IntoJsResult;
use wasm_bindgen::prelude::*;

mod api;
mod interop;


mod loader_file;
mod loader_url;
mod logger;

mod runtime;

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

/// Create a resource that corresponds to the project root
fn create_root_resource() -> Resource {
    Resource::new(
        ResourcePath::FsPath(Path::new()),
        loader_file::new_loader(),
        loader_url::new_loader(),
        Marc::new(LocalResourceResolver(Path::new()))
    )
}

const SOURCE_NAME: &str = "(local)";

#[wasm_bindgen]
pub async fn test_placeholder() -> Result<OpaqueExecDoc, JsValue> {
    let resource = create_root_resource();
    let setting = Setting::default();
    let project_resource = match celerc::api::resolve_project(&resource).await {
        Ok(x) => x,
        Err(e) => {
            let x = 
            celerc::api::make_doc_for_packer_error(SOURCE_NAME, e).await;
            return OpaqueExecDoc::wrap(x);
        }
    };
    // TODO #86 cache this
    let context = match celerc::api::prepare(SOURCE_NAME, project_resource, setting).await {
        Ok(x) => x,
        Err(e) => {
            let x = 
            celerc::api::make_doc_for_packer_error(SOURCE_NAME, e).await;
            return OpaqueExecDoc::wrap(x);
        }
    };

    let x = context.compile().await;
            return OpaqueExecDoc::wrap(x);
}

// ffi!(
//     /// Compile a document from web editor
//     ///
//     /// If use_cache is true, the compiler will use cached results loaded from URLs
//     pub async fn compileDocument() -> Option<ExecDoc> {
//         api::compile_document().await
//     }
//
// );


/// Request current compilation be cancelled
#[wasm_bindgen]
pub fn request_cancel() {
    celerc::api::cancel_current_compilation();
}
