use serde_json::Value;

use crate::macros::async_trait;

use crate::pack::ValidUse;
use crate::pack::{PackerError, PackerResult};

mod loader_empty;
pub use loader_empty::*;
mod resource_github;
pub use resource_github::*;
mod resource_impl;
pub use resource_impl::*;
mod resource_local;
pub use resource_local::*;

/// This is a grouping trait used to group resource loader types in
/// an environment.
pub trait ResourceEnvironment {
    type FsLoader: ResourceLoader;
    type UrlLoader: ResourceLoader;

    fn fs_loader(&self) -> &Self::FsLoader;
    fn url_loader(&self) -> &Self::UrlLoader;
}



#[async_trait(auto)]
pub trait ResourceResolver {
    /// Resolve a resource from the given `Use`.
    ///
    /// The returned resource must have the same loader types as the source.
    /// This is due to the constraint of using static dispatch. However,
    /// it does not make sense that different resource loaders are used in the same resource
    /// context.
    async fn resolve<TContext>(
        &self, 
        source: &Resource<TContext>, 
        target: &ValidUse) 
    -> PackerResult<Resource<TContext>> where
        Self: Sized;
}



/// Loader that loads resources from external place
#[async_trait(auto)]
pub trait ResourceLoader {
    /// Load a resource as raw bytes
    async fn load_raw(&self, path: &str) -> PackerResult<Vec<u8>>;

    async fn load_utf8(&self, path: &str) -> PackerResult<String> {
        let bytes = self.load_raw(path).await?;
        match String::from_utf8(bytes) {
            Ok(v) => Ok(v),
            Err(_) => Err(PackerError::InvalidUtf8(path.to_string())),
        }
    }

    /// Load structured value. The type depends on the file extension read from the ref
    async fn load_structured(&self, path: &str) -> PackerResult<Value> {
        let v = if path.ends_with(".yaml") || path.ends_with(".yml") {
            self.load_yaml(path).await?
        } else if path.ends_with(".json") {
            self.load_json(path).await?
        } else {
            return Err(PackerError::UnknownFormat(path.to_string()));
        };
        Ok(v)
    }

    async fn load_yaml(&self, path: &str) -> PackerResult<Value> {
        let bytes = self.load_raw(path).await?;
        match serde_yaml::from_slice(&bytes) {
            Ok(v) => Ok(v),
            Err(e) => Err(PackerError::InvalidFormat(path.to_string(), e.to_string())),
        }
    }

    async fn load_json(&self, path: &str) -> PackerResult<Value> {
        let bytes = self.load_raw(path).await?;
        match serde_json::from_slice(&bytes) {
            Ok(v) => Ok(v),
            Err(e) => Err(PackerError::InvalidFormat(path.to_string(), e.to_string())),
        }
    }

    /// Load an image resource as URL
    async fn load_image_url(&self, path: &str) -> PackerResult<String>;
}
