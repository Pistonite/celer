//! GitHub resource resolver and loader impl
use std::sync::Arc;

use crate::util::Path;
use cached::proc_macro::cached;

use super::{ArcLoader, EmptyLoader, Resource, ResourceLoader, ResourcePath, ResourceResolver};
use crate::pack::{PackerError, PackerResult, ValidUse};

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

#[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
#[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
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
    let url = match reference {
        Some(reference) => {
            format!("https://raw.githubusercontent.com/{owner}/{repo}/{reference}/{path}")
        }
        None => {
            let default_branch = get_default_branch(owner, repo).await?;
            format!("https://raw.githubusercontent.com/{owner}/{repo}/{default_branch}/{path}")
        }
    };
    Ok(url)
}

/// Get the default branch of a repository.
#[cached(
    key="String", 
    convert = r#"{ format!("{}/{}", owner, repo) }"#,
    // 1 hour TTL
    time=3600,
)]
async fn get_default_branch(owner: &str, repo: &str) -> PackerResult<String> {
    Err(PackerError::NotImpl(
        "getting default branch not implemented".to_string(),
    ))
}
