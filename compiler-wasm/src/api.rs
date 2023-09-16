use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::sync::Arc;

use celerc::api::{Setting, CompilerMetadata, CompilerOutput};
use celerc::pack::{ArcLoader, GlobalCacheLoader, UrlLoader, LocalResourceResolver, Resource, ResourcePath};
use celerc::util::Path;
use celerctypes::ExecDoc;
use js_sys::Function;
use log::info;

use crate::resource::FileCache;
use crate::logger::Logger;

thread_local! {
    static FILE_CACHE: Arc<FileCache> = Arc::new(FileCache::new());
}

thread_local! {
    static URL_LOADER: ArcLoader = Arc::new(GlobalCacheLoader::new(UrlLoader));
}

thread_local! {
    static COMPILER_META: RefCell<Option<CompilerMetadata>> = RefCell::new(None);
}

/// Initialize
pub fn init(load_file: Function, check_changed: Function) {
    info!("initializing compiler...");
    let _ = log::set_boxed_logger(Box::new(Logger));
    FILE_CACHE.with(|cache| {
        cache.init(load_file, check_changed);
    });

    info!("compiler initialized");
}

/// Compile a document from web editor
///
/// Return None if the compilation was interrupted
pub async fn compile_document() -> Option<ExecDoc> {
    // create root resource
    let fs_loader = FILE_CACHE.with(|x| {
        x.clone()
    });
    let url_loader = URL_LOADER.with(|x| {
        x.clone()
    });
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
        CompilerOutput::Ok { exec_doc, metadata, .. } => {
            COMPILER_META.with(|x| {
                x.borrow_mut().replace(metadata);
            });
            Some(exec_doc)
        }
    }
}
