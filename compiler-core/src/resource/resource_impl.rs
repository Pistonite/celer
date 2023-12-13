use serde_json::Value;

use crate::macros::async_trait;
use crate::pack::{PackerResult, ValidUse};
use crate::util::{RefCounted, Path};

use super::{ResourceLoader, EmptyLoader, ResourceResolver, ResourceContext};

macro_rules! loader_delegate {
    ($func:ident, $type:ty) => {
        #[doc = "Macro-generated loader delegate. See [`ResourceLoader`]"]
        pub async fn $func(&self) -> PackerResult<$type> {
            match &self {
                Self::Url(res) => res.url_loader.$func(&res.url).await,
                Self::File(res) => res.fs_loader.$func(res.path.as_ref()).await,
            }
        }
    };
}

/// A resource contains:
/// - an API to load the resource
/// - an API to resolve another resource from its location
///
/// # Static vs Dynamic Dispatch
/// The reason why loaders are using static dispatch while the resolver
/// is using dynamic dispatch, is because loader types should stay the same
/// in a given environment (e.g WASM or server), but the resolver type can change depending
/// on the input to the resolver (e.g. resolving local vs remote resource).
///
/// While it may make sense to hard code the resolver logic into Resource itself,
/// it will be hard in the future if we were to support more remote locations (other than GitHub).
/// By using dynamic dispatch, the resource only cares whether it is a file or a URL, and what the
/// URL is. It delegates to the resolver to figure out how to resolve the next URL.
#[derive(Debug, Clone)]
pub enum Resource<TContext>
    where TContext: ResourceContext {
    File(FileResource<TContext>),
    Url(UrlResource<TContext::UrlLoader>),
}

#[derive(Debug, Clone)]
pub struct FileResource<TContext> 
    where TContext: ResourceContext {
    path: Path,
    fs_loader: RefCounted<TContext::FsLoader>,
    url_loader: RefCounted<TContext::UrlLoader>,
    resolver: RefCounted<dyn ResourceResolver>,
}

#[derive(Debug, Clone)]
pub struct UrlResource<TUrlLoader> 
where        TUrlLoader: ResourceLoader {
    url: String,
    url_loader: RefCounted<TUrlLoader>,
    resolver: RefCounted<dyn ResourceResolver>,
}


// /// A resource contains:
// /// - an API to load the resource
// /// - an API to resolve another resource from its location
// #[derive(Clone)]
// pub struct Resource<TFsLoader, TUrlLoader, TResolver> 
//     where TFsLoader: ResourceLoader,
//         TUrlLoader: ResourceLoader,
//         TResolver: ResourceResolver,
//     {
//     path: ResourcePath,
//     fs_loader: RefCounted<TFsLoader>,
//     url_loader: RefCounted<TUrlLoader>,
//     resolver: RefCounted<TResolver>,
// }

// impl <TUrlLoader> Resource<EmptyLoader, TUrlLoader>
//     where TUrlLoader: ResourceLoader {
//
//
// }

impl <TContext> Resource<TContext> 
    where TContext: ResourceContext {
    pub fn new_file(
        path: Path,
        fs_loader: RefCounted<TContext::FsLoader>,
        url_loader: RefCounted<TContext::UrlLoader>,
        resolver: RefCounted<dyn ResourceResolver>,
    ) -> Self {
        Self::File(FileResource {
            path,
            fs_loader,
            url_loader,
            resolver,
        })
    }

    pub fn new_url(
        url: String,
        url_loader: RefCounted<TContext::UrlLoader>,
        resolver: RefCounted<dyn ResourceResolver>,
    ) -> Self {
        Self::Url(UrlResource {
            url,
            url_loader,
            resolver,
        })
    }

    /// Create a new file resource with the same loaders
    pub fn create_file(&self, path: Path, resolver: RefCounted<dyn ResourceResolver>) -> Self {
        Self::new_file(
            path,
            RefCounted::clone(&self.fs_loader),
            RefCounted::clone(&self.url_loader),
            resolver,
        )
    }

    /// Create a new url resource with the same url loader
    pub fn create_url(&self, url: String, resolver: RefCounted<dyn ResourceResolver>) -> Self {
        Self::new_url(
            url,
            RefCounted::clone(&self.url_loader),
            resolver,
        )
    }

    /// File path or URL
    pub fn name(&self) -> &str {
        match &self{
            Self::Url(res) => &res.url,
            Self::File(res) => &res.path.as_ref(),
        }
    }

    loader_delegate!(load_structured, Value);
    loader_delegate!(load_utf8, String);
    loader_delegate!(load_image_url, String);

    pub async fn resolve<TContextNew>(&self, target: &ValidUse) -> PackerResult<Resource<TContextNew>> {
        self.resolver.resolve(self, target).await
    }
}
