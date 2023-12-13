use crate::macros::async_trait;
use crate::pack::{PackerError, PackerResult, ValidUse};
use crate::util::{RefCounted, Path};

use super::{create_github_resource_from, Resource, ResourceResolver, ResourceLoader};

pub struct LocalResourceResolver(pub Path);

#[async_trait(auto)]
impl ResourceResolver for LocalResourceResolver {
    async fn resolve<TContext>(
        &self, 
        source: &Resource<TContext>, 
        target: &ValidUse) -> PackerResult<Resource<TContext>> 
    {
        match target {
            ValidUse::Relative(path) => {
                let new_path = self
                    .0
                    .join(&path)
                    .ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
                if self.0 == new_path {
                    return Ok(source.clone());
                }
                let new_parent = new_path
                    .parent()
                    .ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
                Ok(source.create_file(new_path, RefCounted::new(Self(new_parent))))
            }
            ValidUse::Absolute(path) => {
                let new_path =
                    Path::try_from(path).ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
                if self.0 == new_path {
                    return Ok(source.clone());
                }
                let new_parent = new_path
                    .parent()
                    .ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
                Ok(source.create_file(new_path, RefCounted::new(Self(new_parent))))
            }
            ValidUse::Remote {
                owner,
                repo,
                path,
                reference,
            } => create_github_resource_from(source, owner, repo, path, reference.as_deref()).await,
        }
    }
}
