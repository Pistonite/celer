use std::sync::Arc;

use crate::pack::{ValidUse, PackerResult, PackerError};
use crate::util::Path;

use super::{ResourceResolver, Resource, ResourcePath, create_github_resource_from};

pub struct LocalResourceResolver(pub Path);

#[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
#[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
impl ResourceResolver for LocalResourceResolver {
    async fn resolve(&self, source: &Resource, target: &ValidUse) -> PackerResult<Resource> {
        match target {
            ValidUse::Relative(path) => {
                let new_path = self.0.join(&path).ok_or_else(||PackerError::InvalidPath(path.clone()))?;
                if self.0 == new_path {
                    return Ok(source.clone());
                }
                Ok(source.create(ResourcePath::FsPath(new_path.clone()), Arc::new(Self(new_path))))
            }
            ValidUse::Absolute(path) => {
                let new_path = Path::try_from(&path).ok_or_else(||PackerError::InvalidPath(path.clone()))?;
                if self.0 == new_path {
                    return Ok(source.clone());
                }
                Ok(source.create(ResourcePath::FsPath(new_path.clone()), Arc::new(Self(new_path))))
            }
            ValidUse::Remote { owner, repo, path, reference } => {
                create_github_resource_from(source, owner, repo, path, reference.as_deref()).await
            },
        }
    }
}
