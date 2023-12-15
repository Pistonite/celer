//! Resource resolving and loading
use std::borrow::Cow;

use crate::macros::async_trait;

mod path;
pub use path::*;
mod res_use;
pub use res_use::*;
mod res_type;
pub use res_type::*;
mod resolve;
pub use resolve::*;

mod resource_github;
pub use resource_github::*;
mod resource_impl;
pub use resource_impl::*;
mod resource_local;
pub use resource_local::*;

#[cfg(test)]
mod test_utils;

/// Resource-related error types
#[derive(Debug, thiserror::Error)]
pub enum ResError {
    #[error("Resource is not valid UTF-8: {0}")]
    InvalidUtf8(String),

    #[error("Error when parsing JSON resource `{0}`: {1}")]
    InvalidJson(String, serde_json::Error),

    #[error("Error when parsing YAML resource `{0}`: {1}")]
    InvalidYaml(String, serde_yaml::Error),

    #[error("Cannot determine the data format for `{0}`.")]
    UnknownDataFormat(String),

    #[error("Cannot determine the image format for `{0}`.")]
    UnknownImageFormat(String),

    #[error("Cannot resolve `{0}` from `{1}`.")]
    CannotResolve(String, String),
}

pub type ResResult<T> = Result<T, ResError>;

// /// This is a grouping trait used to group resource loader types in
// /// an environment.
// pub trait ResourceEnvironment {
//     type FsLoader: ResourceLoader;
//     type UrlLoader: ResourceLoader;
//
//     fn fs_loader(&self) -> &Self::FsLoader;
//     fn url_loader(&self) -> &Self::UrlLoader;
// }


// #[async_trait(auto)]
// pub trait ResourceResolver {
//     /// Resolve a resource from the given `Use`.
//     ///
//     /// The returned resource must have the same loader types as the source.
//     /// This is due to the constraint of using static dispatch. However,
//     /// it does not make sense that different resource loaders are used in the same resource
//     /// context.
//     async fn resolve<TResEnv>(
//         rc_self: &RefCounted<Self>,
//         source: &RefCounted<TResEnv>,
//         target: &ValidUse) 
//     -> PackerResult<Resource<TResEnv>> where
//         Self: Sized;
// }
//


/// Loader that loads resources from external place
#[async_trait(auto)]
pub trait Loader {
    /// Load a resource as raw bytes
    async fn load_raw<'s, 'a>(&'s self, path: &ResPath<'a>) -> ResResult<Cow<'s, [u8]>>;
}
