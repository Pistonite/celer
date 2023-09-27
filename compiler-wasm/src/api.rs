use std::cell::RefCell;
use std::sync::Arc;

use celerc::api::{CompilerMetadata, CompilerOutput, Setting};
use celerc::pack::{
    ArcLoader, GlobalCacheLoader, LocalResourceResolver, Resource, ResourcePath, UrlLoader,
};
use celerc::util::Path;
use celerctypes::ExecDoc;
use js_sys::Function;
use log::{info, LevelFilter};
use wasm_bindgen::JsValue;
use web_sys::console;

use crate::logger::{self, Logger};
use crate::resource::FileLoader;

const LOGGER: Logger = Logger;

thread_local! {
    static FILE_LOADER: Arc<FileLoader> = Arc::new(FileLoader::new());
}

thread_local! {
    static URL_LOADER: ArcLoader = Arc::new(GlobalCacheLoader::new(Arc::new(UrlLoader)));
}

thread_local! {
    static COMPILER_META: RefCell<Option<CompilerMetadata>> = RefCell::new(None);
}

/// Initialize
pub fn init(logger: JsValue, load_file: Function) {
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
        CompilerOutput::Ok {
            exec_doc, metadata, ..
        } => {
            COMPILER_META.with(|x| {
                x.borrow_mut().replace(metadata);
            });
            Some(exec_doc)
        }
    }
}
