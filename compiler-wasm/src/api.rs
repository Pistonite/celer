use std::cell::RefCell;
use std::sync::Arc;

use celerc::api::{CompilerMetadata, CompilerOutput, Setting};
use celerc::pack::{LocalResourceResolver, Resource, ResourcePath};
use celerc::util::{Path, self};
use celerctypes::ExecDoc;
use js_sys::Function;
use log::{info, warn, LevelFilter};
use wasm_bindgen::JsValue;
use web_sys::{console, window};

use crate::loader_file::FileLoader;
use crate::loader_url::UrlLoader;
use crate::logger::{self, Logger};

const LOGGER: Logger = Logger;

thread_local! {
    #[allow(clippy::arc_with_non_send_sync)]
    static FILE_LOADER: Arc<FileLoader> = Arc::new(FileLoader::new());
}

thread_local! {
    #[allow(clippy::arc_with_non_send_sync)]
    static URL_LOADER: Arc<UrlLoader> = Arc::new(UrlLoader::new());
}

thread_local! {
    static COMPILER_META: RefCell<Option<CompilerMetadata>> = RefCell::new(None);
}

/// Initialize
pub fn init(logger: JsValue, load_file: Function, fetch: Function) {
    if let Err(e) = logger::bind_logger(logger) {
        console::error_1(&e);
    }
    match log::set_logger(&LOGGER) {
        Ok(_) => {
            log::set_max_level(LevelFilter::Info);
        }
        Err(_) => {
            console::warn_1(
                &"failed to initialize compiler logger. It might have already been initialized"
                    .into(),
            );
        }
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
pub async fn compile_document() -> Option<ExecDoc> {
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

    match celerc::api::compile(&resource, &setting).await {
        CompilerOutput::Cancelled => None,
        CompilerOutput::Ok(output) => {
            let metadata = output.metadata;
            COMPILER_META.with(|x| {
                x.borrow_mut().replace(metadata);
            });
            Some(output.exec_doc)
        }
    }
}
