use std::borrow::Cow;

use base64::Engine;
use serde_json::Value;

use crate::macros::async_trait;
use crate::util::{RefCounted, Path};

// use super::{ResourceLoader, EmptyLoader, ResourceResolver, ResourceContext, ResourceEnvironment, ResourcePath};
use super::{ResPath, Loader, ResResult, ResError, ResType, ValidUse};

// macro_rules! loader_delegate {
//     ($func:ident, $type:ty) => {
//         #[doc = "Macro-generated loader delegate. See [`ResourceLoader`]"]
//         pub async fn $func(&self) -> PackerResult<$type> {
//         self.loader.$func(&self.path).await
//         }
//     };
// }

/// A Resource is an absolute reference to a resource that can be loaded.
/// It can be a local file or a remote URL. It also has an associated ref-counted
/// [`Loader`] that can be used to load the resource.
#[derive(Debug, Clone)]
pub struct Resource<'a, 'b, L> where L: Loader {
    path: ResPath<'a, 'b>,
    loader: RefCounted<L>,
        //resolver: RefCounted<dyn ResourceResolver>,
}



// #[derive(Debug, Clone)]
// pub struct FileResource<TContext> 
//     where TContext: ResourceContext {
//     path: Path,
//     fs_loader: RefCounted<TContext::FsLoader>,
//     url_loader: RefCounted<TContext::UrlLoader>,
//     resolver: RefCounted<dyn ResourceResolver>,
// }
//
// #[derive(Debug, Clone)]
// pub struct UrlResource<TUrlLoader> 
// where        TUrlLoader: ResourceLoader {
//     url: String,
//     url_loader: RefCounted<TUrlLoader>,
//     resolver: RefCounted<dyn ResourceResolver>,
// }


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

impl<'a, 'b, L> Resource<'a, 'b, L> where L: Loader {
    pub fn new(
        path: &ResPath<'a, 'b>,
        loader: RefCounted<L>,
        // fs_loader: RefCounted<TContext::FsLoader>,
        // url_loader: RefCounted<TContext::UrlLoader>,
        // resolver: RefCounted<dyn ResourceResolver>,
    ) -> Self {
        Self {
            path: path.clone(), //clone the pointer, not data
            loader,
            // fs_loader,
            // url_loader,
            // resolver,
        }
        // Self::File(FileResource {
        //     path,
        //     fs_loader,
        //     url_loader,
        //     resolver,
        // })
    }

    // pub fn new_url(
    //     url: String,
    //     url_loader: RefCounted<TContext::UrlLoader>,
    //     resolver: RefCounted<dyn ResourceResolver>,
    // ) -> Self {
    //     Self::Url(UrlResource {
    //         url,
    //         url_loader,
    //         resolver,
    //     })
    // }

    // /// Create a new file resource with the same loaders
    // pub fn create_file(&self, path: Path, resolver: RefCounted<dyn ResourceResolver>) -> Self {
    //     Self::new_file(
    //         path,
    //         RefCounted::clone(&self.fs_loader),
    //         RefCounted::clone(&self.url_loader),
    //         resolver,
    //     )
    // }
    //
    // /// Create a new url resource with the same url loader
    // pub fn create_url(&self, url: String, resolver: RefCounted<dyn ResourceResolver>) -> Self {
    //     Self::new_url(
    //         url,
    //         RefCounted::clone(&self.url_loader),
    //         resolver,
    //     )
    // }

    pub fn path(&self) -> &ResPath<'a, 'b> {
        &self.path
        // match &self{
        //     Self::Url(res) => &res.url,
        //     Self::File(res) => &res.path.as_ref(),
        // }
    }

    /// Load the resource as raw bytes
    pub async fn load_raw(&self) -> ResResult<Cow<'_, [u8]>> {
        self.loader.load_raw(&self.path).await
    }

    /// Load the resource as UTF-8 string
    pub async fn load_utf8(&self) -> ResResult<Cow<'_, str>> {
        let bytes = self.loader.load_raw(&self.path).await?;
        match bytes {
            Cow::Borrowed(bytes) => {
                match std::str::from_utf8(bytes) {
                    Ok(v) => Ok(Cow::from(v)),
                    Err(_) => Err(ResError::InvalidUtf8(self.path.to_string())),
                }
            }
            Cow::Owned(bytes) => {
                match String::from_utf8(bytes) {
                    Ok(v) => Ok(Cow::from(v)),
                    Err(_) => Err(ResError::InvalidUtf8(self.path.to_string())),
                }
            }
        }
    }

    /// Load the resource as structured value for supported formats (JSON, YAML)
    pub async fn load_structured(&self) -> ResResult<Value> {
        match self.path.get_type() {
            Some(ResType::Yaml) => {
                let bytes = self.loader.load_raw(&self.path).await?;
                match serde_yaml::from_slice(&bytes) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ResError::InvalidYaml(self.path.to_string(), e)),
                }
            }
            Some(ResType::Json) => {
                let bytes = self.loader.load_raw(&self.path).await?;
                match serde_json::from_slice(&bytes) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ResError::InvalidJson(self.path.to_string(), e)),
                }
            }
            _ => Err(ResError::UnknownDataFormat(self.path.to_string())),
        }
    }

    /// Load the image as either a remote URL or a data URL
    pub async fn load_image_url(&self) -> ResResult<String> {
        if !self.path.is_local() {
            // if path is a URL, just return it
            return Ok(self.path.to_string());
        }
        let image_type = self.path.get_type();
        let media_type = match image_type {
            Some(x) if x.is_image() => x.media_type(),
            _ => return Err(ResError::UnknownImageFormat(self.path.to_string())),
        };
        // prepare the beginning of the data url
        let mut data_url = format!("data:{media_type};base64,");
        // load the bytes
        let bytes = self.loader.load_raw(&self.path).await?;
        // encode the bytes and append to the data url
        base64::engine::general_purpose::STANDARD.encode_string(bytes, &mut data_url);

        Ok(data_url)
    }
}
