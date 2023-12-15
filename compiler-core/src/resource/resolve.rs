//! Implementation for resolving a `use` from a resource

use crate::util::{Path, RefCounted};

use super::{ResPath, Loader, Resource, ValidUse, ResResult, ResError};

impl<'a, 'b, L> Resource<'a, 'b, L> where L: Loader {

    /// Resolve a `use` property from this resource
    pub fn resolve(&self, target: &ValidUse) -> ResResult<Resource<'a, 'b, L>> {
        // See tests
        todo!()
    }
    // reference:
// pub struct LocalResourceResolver(pub Path);
//
// #[async_trait(auto)]
// impl ResourceResolver for LocalResourceResolver {
//     async fn resolve<TContext>(
//         &self, 
//         source: &Resource<TContext>, 
//         target: &ValidUse) -> PackerResult<Resource<TContext>> 
//     {
//         match target {
//             ValidUse::Relative(path) => {
//                 let new_path = self
//                     .0
//                     .join(&path)
//                     .ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
//                 if self.0 == new_path {
//                     return Ok(source.clone());
//                 }
//                 let new_parent = new_path
//                     .parent()
//                     .ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
//                 Ok(source.create_file(new_path, RefCounted::new(Self(new_parent))))
//             }
//             ValidUse::Absolute(path) => {
//                 let new_path =
//                     Path::try_from(path).ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
//                 if self.0 == new_path {
//                     return Ok(source.clone());
//                 }
//                 let new_parent = new_path
//                     .parent()
//                     .ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
//                 Ok(source.create_file(new_path, RefCounted::new(Self(new_parent))))
//             }
//             ValidUse::Remote {
//                 owner,
//                 repo,
//                 path,
//                 reference,
//             } => create_github_resource_from(source, owner, repo, path, reference.as_deref()).await,
//         }
//     }
// }

// pub struct GitHubResourceResolver {
//     owner: String,
//     repo: String,
//     path: Path,
//     reference: Option<String>,
// }
//
// impl GitHubResourceResolver {
//     pub fn new(owner: &str, repo: &str, path: Path, reference: Option<&str>) -> Self {
//         Self {
//             owner: owner.to_string(),
//             repo: repo.to_string(),
//             path,
//             reference: reference.map(|s| s.to_string()),
//         }
//     }
// }
//
// #[async_trait(auto)]
// impl ResourceResolver for GitHubResourceResolver {
//     async fn resolve<TContext>(
//         &self, 
//         source: &Resource<TContext>,
//         target: &ValidUse) -> PackerResult<Resource<TContext>> 
//     {
//         match target {
//             ValidUse::Relative(path) => {
//                 let new_path = self
//                     .path
//                     .join(&format!("../{path}"))
//                     .ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
//                 if self.path == new_path {
//                     return Ok(source.clone());
//                 }
//                 let url = get_github_url(
//                     &self.owner,
//                     &self.repo,
//                     &new_path,
//                     self.reference.as_deref(),
//                 )
//                 .await?;
//                 Ok(source.create_url(
//                     url,
//                     RefCounted::new(Self::new(
//                         &self.owner,
//                         &self.repo,
//                         new_path,
//                         self.reference.as_deref(),
//                     )),
//                 ))
//             }
//             ValidUse::Absolute(path) => {
//                 let new_path =
//                     Path::try_from(path).ok_or_else(|| PackerError::InvalidPath(path.clone()))?;
//                 if self.path == new_path {
//                     return Ok(source.clone());
//                 }
//                 let url = get_github_url(
//                     &self.owner,
//                     &self.repo,
//                     &new_path,
//                     self.reference.as_deref(),
//                 )
//                 .await?;
//                 Ok(source.create_url(
//                     url,
//                     RefCounted::new(Self::new(
//                         &self.owner,
//                         &self.repo,
//                         new_path,
//                         self.reference.as_deref(),
//                     )),
//                 ))
//             }
//             ValidUse::Remote {
//                 owner,
//                 repo,
//                 path,
//                 reference,
//             } => create_github_resource_from(source, owner, repo, path, reference.as_deref()).await,
//         }
//     }
// }
//
// pub async fn create_github_resource<TContext>(
//     owner: &str,
//     repo: &str,
//     path: &str,
//     reference: Option<&str>,
//     url_loader: RefCounted<TContext::UrlLoader>,
// ) -> PackerResult<Resource<TContext>>
// where
//     TContext: ResourceContext,
// {
//     let path = Path::try_from(path).ok_or_else(|| PackerError::InvalidPath(path.to_string()))?;
//     let url = get_github_url(owner, repo, &path, reference).await?;
//     Ok(Resource::new_url(
//         url,
//         url_loader,
//         RefCounted::new(GitHubResourceResolver::new(owner, repo, path, reference)),
//     ))
// }
//
// pub async fn create_github_resource_from<TContext>(
//     source: &Resource<TContext>,
//     owner: &str,
//     repo: &str,
//     path: &str,
//     reference: Option<&str>,
// ) -> PackerResult<Resource<TContext>>
//     where
//         TContext: ResourceContext,
// {
//     let path = Path::try_from(path).ok_or_else(|| PackerError::InvalidPath(path.to_string()))?;
//     let url = get_github_url(owner, repo, &path, reference).await?;
//     Ok(source.create_url(
//         url,
//         RefCounted::new(GitHubResourceResolver::new(owner, repo, path, reference)),
//     ))
// }
//
// /// Get a github URL
// async fn get_github_url(
//     owner: &str,
//     repo: &str,
//     path: &Path,
//     reference: Option<&str>,
// ) -> PackerResult<String> {
//     let path = path.as_ref();
//     let branch = reference.unwrap_or("main");
//     let url = format!("https://raw.githubusercontent.com/{owner}/{repo}/{branch}/{path}");
//     Ok(url)
// }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::resource::test_utils::StubLoader;

    static TEST_URL_PREFIX: &str = "https://hello/";

    fn create_local_resource(path: &str) -> Resource<StubLoader> {
        Resource::new(
            &ResPath::new_local(path),
            RefCounted::new(StubLoader),
        )
    }

    fn create_remote_resource(path: &str) -> Resource<StubLoader> {
        Resource::new(
            &ResPath::new_remote(TEST_URL_PREFIX, path),
            RefCounted::new(StubLoader),
        )
    }

    #[test]
    fn test_local_relative_error() {
        let resource = create_local_resource("foo");
        let target = ValidUse::Relative("../bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap_err(), ResError::CannotResolve("foo".into(), "../bar".into()));

        let resource = create_local_resource("foo/bar");
        let target = ValidUse::Relative("../../bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap_err(), ResError::CannotResolve("foo/bar".into(), "../../bar".into()));
    }

    #[test]
    fn test_local_relative() {
        let resource = create_local_resource("foo");
        let target = ValidUse::Relative("./bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_local("bar"));

        let resource = create_local_resource("foo/bar");
        let target = ValidUse::Relative("./biz".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_local("foo/biz"));

        let resource = create_local_resource("foo/bar");
        let target = ValidUse::Relative("../biz".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_local("biz"));
    }

    #[test]
    fn test_local_absolute() {
        let resource = create_local_resource("foo");
        let target = ValidUse::Absolute("/bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_local("bar"));

        let resource = create_local_resource("foo/bar");
        let target = ValidUse::Absolute("/biz".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_local("biz"));

        let resource = create_local_resource("foo/bar/woo");
        let target = ValidUse::Absolute("/biz/bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_local("biz/bar"));
    }

    #[test]
    fn test_local_remote() {
        let resource = create_local_resource("foo");
        let target = ValidUse::Remote {
            owner: "owner".into(),
            repo: "repo".into(),
            path: "bar".into(),
            reference: None,
        };
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_remote(&target.base_url().unwrap(), "bar"));

        let resource = create_local_resource("foo/a");
        let target = ValidUse::Remote {
            owner: "owner".into(),
            repo: "repo".into(),
            path: "bar/b".into(),
            reference: Some("branch".into()),
        };
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_remote(&target.base_url().unwrap(), "bar/b"));
    }

    #[test]
    fn test_remote_relative_error() {
        let resource = create_remote_resource("foo");
        let target = ValidUse::Relative("../bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap_err(), ResError::CannotResolve(format!("{TEST_URL_PREFIX}/foo"), "../bar".into()));

        let resource = create_remote_resource("foo/bar/biz");
        let target = ValidUse::Relative("../../../bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap_err(), ResError::CannotResolve(format!("{TEST_URL_PREFIX}/foo/bar/biz"), "../../../bar".into()));
    }

    #[test]
    fn test_remote_relative() {
        let resource = create_remote_resource("foo");
        let target = ValidUse::Relative("./bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), create_remote_resource("bar").path());

        let resource = create_remote_resource("foo/biz");
        let target = ValidUse::Relative("./bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), create_remote_resource("foo/bar").path());

        let resource = create_remote_resource("foo/biz");
        let target = ValidUse::Relative("../bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), create_remote_resource("bar").path());

        let resource = create_remote_resource("foo/biz/bar");
        let target = ValidUse::Relative("../a/../b".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), create_remote_resource("foo/b").path());
    }

    #[test]
    fn test_remote_absolute() {
        let resource = create_remote_resource("foo");
        let target = ValidUse::Absolute("/bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), create_remote_resource("bar").path());

        let resource = create_remote_resource("foo/zar");
        let target = ValidUse::Absolute("/bar/biz/../a".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), create_remote_resource("bar/a").path());
    }

    #[test]
    fn test_remote_remote() {
        let resource = create_remote_resource("foo");
        let target = ValidUse::Remote {
            owner: "owner".into(),
            repo: "repo".into(),
            path: "bar".into(),
            reference: None,
        };
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_remote(&target.base_url().unwrap(), "bar"));

        let resource = create_remote_resource("foo/a");
        let target = ValidUse::Remote {
            owner: "owner".into(),
            repo: "repo".into(),
            path: "bar/b".into(),
            reference: Some("branch".into()),
        };
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_remote(&target.base_url().unwrap(), "bar/b"));
    }
}

