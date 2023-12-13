//! GitHub resource resolver and loader impl

use crate::macros::async_trait;
use crate::pack::{PackerError, PackerResult, ValidUse};
use crate::util::{RefCounted, Path};

use super::{EmptyLoader, Resource, ResourceResolver, ResourceLoader, ResourceContext};

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

#[async_trait(auto)]
impl ResourceResolver for GitHubResourceResolver {
    async fn resolve<TContext>(
        &self, 
        source: &Resource<TContext>,
        target: &ValidUse) -> PackerResult<Resource<TContext>> 
    {
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
                Ok(source.create_url(
                    url,
                    RefCounted::new(Self::new(
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
                Ok(source.create_url(
                    url,
                    RefCounted::new(Self::new(
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

pub async fn create_github_resource<TContext>(
    owner: &str,
    repo: &str,
    path: &str,
    reference: Option<&str>,
    url_loader: RefCounted<TContext::UrlLoader>,
) -> PackerResult<Resource<TContext>>
where
    TContext: ResourceContext,
{
    let path = Path::try_from(path).ok_or_else(|| PackerError::InvalidPath(path.to_string()))?;
    let url = get_github_url(owner, repo, &path, reference).await?;
    Ok(Resource::new_url(
        url,
        url_loader,
        RefCounted::new(GitHubResourceResolver::new(owner, repo, path, reference)),
    ))
}

pub async fn create_github_resource_from<TContext>(
    source: &Resource<TContext>,
    owner: &str,
    repo: &str,
    path: &str,
    reference: Option<&str>,
) -> PackerResult<Resource<TContext>>
    where
        TContext: ResourceContext,
{
    let path = Path::try_from(path).ok_or_else(|| PackerError::InvalidPath(path.to_string()))?;
    let url = get_github_url(owner, repo, &path, reference).await?;
    Ok(source.create_url(
        url,
        RefCounted::new(GitHubResourceResolver::new(owner, repo, path, reference)),
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
