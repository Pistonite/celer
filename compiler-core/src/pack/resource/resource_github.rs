//! GitHub resource resolver and loader impl
use std::sync::Arc;

use crate::macros::{async_trait, maybe_send};
use crate::pack::{PackerError, PackerResult, ValidUse};
use crate::util::Path;

use super::{ArcLoader, EmptyLoader, Resource, ResourcePath, ResourceResolver};

pub struct GitHubResourceResolver {
    owner: String,
    repo: String,
    path: Path,
    reference: Option<String>,
}

impl GitHubResourceResolver {
    pub fn new(owner: &str, repo: &str, path: Path, reference: Option<&str>) -> Self {
        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            path,
            reference: reference.map(|s| s.to_string()),
        }
    }
}

#[maybe_send(async_trait)]
impl ResourceResolver for GitHubResourceResolver {
    async fn resolve(&self, source: &Resource, target: &ValidUse) -> PackerResult<Resource> {
        match target {
            ValidUse::Relative(path) => {
                let new_path = self
                    .path
                    .join(&format!("../{path}"))
                    .ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
                if self.path == new_path {
                    return Ok(source.clone());
                }
                let url = get_github_url(
                    &self.owner,
                    &self.repo,
                    &new_path,
                    self.reference.as_deref(),
                )
                .await?;
                Ok(source.create(
                    ResourcePath::Url(url),
                    Arc::new(Self::new(
                        &self.owner,
                        &self.repo,
                        new_path,
                        self.reference.as_deref(),
                    )),
                ))
            }
            ValidUse::Absolute(path) => {
                let new_path =
                    Path::try_from(path).ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
                if self.path == new_path {
                    return Ok(source.clone());
                }
                let url = get_github_url(
                    &self.owner,
                    &self.repo,
                    &new_path,
                    self.reference.as_deref(),
                )
                .await?;
                Ok(source.create(
                    ResourcePath::Url(url),
                    Arc::new(Self::new(
                        &self.owner,
                        &self.repo,
                        new_path,
                        self.reference.as_deref(),
                    )),
                ))
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

pub async fn create_github_resource(
    owner: &str,
    repo: &str,
    path: &str,
    reference: Option<&str>,
    url_loader: ArcLoader,
) -> PackerResult<Resource> {
    let path = Path::try_from(path).ok_or_else(|| PackerError::InvalidPath(path.to_string()))?;
    let url = get_github_url(owner, repo, &path, reference).await?;
    Ok(Resource::new(
        ResourcePath::Url(url),
        Arc::new(EmptyLoader),
        url_loader,
        Arc::new(GitHubResourceResolver::new(owner, repo, path, reference)),
    ))
}

pub async fn create_github_resource_from(
    source: &Resource,
    owner: &str,
    repo: &str,
    path: &str,
    reference: Option<&str>,
) -> PackerResult<Resource> {
    let path = Path::try_from(path).ok_or_else(|| PackerError::InvalidPath(path.to_string()))?;
    let url = get_github_url(owner, repo, &path, reference).await?;
    Ok(source.create(
        ResourcePath::Url(url),
        Arc::new(GitHubResourceResolver::new(owner, repo, path, reference)),
    ))
}

/// Get a github URL
async fn get_github_url(
    owner: &str,
    repo: &str,
    path: &Path,
    reference: Option<&str>,
) -> PackerResult<String> {
    let path = path.as_ref();
    let branch = reference.unwrap_or("main");
    let url = format!("https://raw.githubusercontent.com/{owner}/{repo}/{branch}/{path}");
    Ok(url)
}
