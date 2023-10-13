use std::sync::Arc;

use celerc::api::Setting;
use celerc::pack::{LocalResourceResolver, Resource, ResourcePath};
use celerc::util::{self, Path};
use js_sys::Function;
use log::{info, LevelFilter};
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::loader_file::FileLoader;
use crate::loader_url::UrlLoader;
use crate::logger::{self, Logger};
use crate::wasm::WasmInto;

const SOURCE_NAME: &str = "(local)";

thread_local! {
    #[allow(clippy::arc_with_non_send_sync)]
    static FILE_LOADER: Arc<FileLoader> = Arc::new(FileLoader::new());
}

thread_local! {
    #[allow(clippy::arc_with_non_send_sync)]
    static URL_LOADER: Arc<UrlLoader> = Arc::new(UrlLoader::new());
}

/// Initialize
pub fn init(logger: JsValue, load_file: Function, fetch: Function) {
    if let Err(e) = logger::bind_logger(logger) {
        console::error_1(&e);
    }
    info!("initializing compiler...");
    FILE_LOADER.with(|loader| {
        loader.init(load_file);
    });
    URL_LOADER.with(|loader| {
        loader.init(fetch);
    });
    if let Some(window) = web_sys::window() {
        if let Ok(origin) = window.location().origin() {
            let _ = util::init_site_origin(origin);
        }
    }

    info!("compiler initialized");
}

/// Compile a document from web editor
///
/// Return None if the compilation was interrupted
/// TODO #78: Option no longer needed
///
/// This returns an Option<ExecDoc>, but must be converted to WASM immediately because of lifetime
/// As part of TODO #86 this should be improved as well
pub async fn compile_document() -> Result<JsValue, JsValue> {
    // create root resource
    let fs_loader = FILE_LOADER.with(|x| x.clone());
    let url_loader = URL_LOADER.with(|x| x.clone());
    let root_path = Path::new();
    let resolver = Arc::new(LocalResourceResolver(root_path.clone()));
    let resource = Resource::new(
        ResourcePath::FsPath(root_path),
        fs_loader,
        url_loader,
        resolver,
    );

    let setting = Setting::default();
    let project_resource = match celerc::api::resolve_project(&resource).await {
        Ok(x) => x,
        Err(e) => {
            return celerc::api::make_doc_for_packer_error(SOURCE_NAME, e)
                .await
                .into_wasm();
        }
    };
    // TODO #86 cache this
    let context = match celerc::api::prepare(SOURCE_NAME, project_resource, setting).await {
        Ok(x) => x,
        Err(e) => {
            return celerc::api::make_doc_for_packer_error(SOURCE_NAME, e)
                .await
                .into_wasm();
        }
    };

    context.compile().await.into_wasm()
}
