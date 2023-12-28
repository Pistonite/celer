//! Implementation for resolving a `use` from a resource

use crate::util::PathBuf;

use super::{Loader, ResError, ResPath, ResResult, Resource, ValidUse};

impl<'a, L> Resource<'a, L>
where
    L: Loader,
{
    /// Resolve a `use` property from this resource
    pub fn resolve(&self, target: &ValidUse) -> ResResult<Resource<'a, L>> {
        let new_resource = match target {
            ValidUse::Relative(path) => {
                // relative paths are resolved relative to the directory of the file
                let join_part = PathBuf::from("..").join(path);
                self.path().join_resolve(join_part)
            }
            ValidUse::Absolute(path) => {
                // absolute paths are resolved relative to the root of the file system
                let rel_path = &path[1..];
                match self.path() {
                    ResPath::Local(_) => ResPath::new_local(rel_path),
                    ResPath::Remote(url, _) => ResPath::new_remote(url.clone(), rel_path),
                }
            }
            remote_use => target
                .base_url()
                .and_then(|url| ResPath::new_remote(url, remote_use.path())),
        };

        match new_resource {
            Some(new_resource) => Ok(self.with_path(new_resource)),
            None => Err(ResError::CannotResolve(
                self.path.to_string(),
                target.to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::res::test_utils::StubLoader;
    use crate::env::RefCounted;

    static TEST_URL_PREFIX: &str = "https://hello/";

    fn create_local_resource(path: &str) -> Resource<StubLoader> {
        Resource::new(
            ResPath::new_local(path).unwrap(),
            RefCounted::new(StubLoader),
        )
    }

    fn create_remote_resource(path: &str) -> Resource<StubLoader> {
        Resource::new(
            ResPath::new_remote(TEST_URL_PREFIX, path).unwrap(),
            RefCounted::new(StubLoader),
        )
    }

    #[test]
    fn test_local_relative_error() {
        let resource = create_local_resource("foo");
        let target = ValidUse::Relative("../bar".into());
        let result = resource.resolve(&target);
        assert_eq!(
            result.unwrap_err(),
            ResError::CannotResolve("foo".into(), "../bar".into())
        );

        let resource = create_local_resource("foo/bar");
        let target = ValidUse::Relative("../../bar".into());
        let result = resource.resolve(&target);
        assert_eq!(
            result.unwrap_err(),
            ResError::CannotResolve("foo/bar".into(), "../../bar".into())
        );
    }

    #[test]
    fn test_local_relative() {
        let resource = create_local_resource("foo");
        let target = ValidUse::Relative("./bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_local_unchecked("bar"));

        let resource = create_local_resource("foo/bar");
        let target = ValidUse::Relative("./biz".into());
        let result = resource.resolve(&target);
        assert_eq!(
            result.unwrap().path(),
            &ResPath::new_local_unchecked("foo/biz")
        );

        let resource = create_local_resource("foo/bar");
        let target = ValidUse::Relative("../biz".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_local_unchecked("biz"));
    }

    #[test]
    fn test_local_absolute() {
        let resource = create_local_resource("foo");
        let target = ValidUse::Absolute("/bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_local_unchecked("bar"));

        let resource = create_local_resource("foo/bar");
        let target = ValidUse::Absolute("/biz".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), &ResPath::new_local_unchecked("biz"));

        let resource = create_local_resource("foo/bar/woo");
        let target = ValidUse::Absolute("/biz/bar".into());
        let result = resource.resolve(&target);
        assert_eq!(
            result.unwrap().path(),
            &ResPath::new_local_unchecked("biz/bar")
        );
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
        assert_eq!(
            result.unwrap().path(),
            &ResPath::new_remote_unchecked(&target.base_url().unwrap(), "bar")
        );

        let resource = create_local_resource("foo/a");
        let target = ValidUse::Remote {
            owner: "owner".into(),
            repo: "repo".into(),
            path: "bar/b".into(),
            reference: Some("branch".into()),
        };
        let result = resource.resolve(&target);
        assert_eq!(
            result.unwrap().path(),
            &ResPath::new_remote_unchecked(&target.base_url().unwrap(), "bar/b")
        );
    }

    #[test]
    fn test_remote_relative_error() {
        let resource = create_remote_resource("foo");
        let target = ValidUse::Relative("../bar".into());
        let result = resource.resolve(&target);
        assert_eq!(
            result.unwrap_err(),
            ResError::CannotResolve(format!("{TEST_URL_PREFIX}foo"), "../bar".into())
        );

        let resource = create_remote_resource("foo/bar/biz");
        let target = ValidUse::Relative("../../../bar".into());
        let result = resource.resolve(&target);
        assert_eq!(
            result.unwrap_err(),
            ResError::CannotResolve(
                format!("{TEST_URL_PREFIX}foo/bar/biz"),
                "../../../bar".into()
            )
        );
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
        assert_eq!(
            result.unwrap().path(),
            create_remote_resource("foo/bar").path()
        );

        let resource = create_remote_resource("foo/biz");
        let target = ValidUse::Relative("../bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), create_remote_resource("bar").path());

        let resource = create_remote_resource("foo/biz/bar");
        let target = ValidUse::Relative("../a/../b".into());
        let result = resource.resolve(&target);
        assert_eq!(
            result.unwrap().path(),
            create_remote_resource("foo/b").path()
        );
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
        assert_eq!(
            result.unwrap().path(),
            create_remote_resource("bar/a").path()
        );
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
        assert_eq!(
            result.unwrap().path(),
            &ResPath::new_remote_unchecked(&target.base_url().unwrap(), "bar")
        );

        let resource = create_remote_resource("foo/a");
        let target = ValidUse::Remote {
            owner: "owner".into(),
            repo: "repo".into(),
            path: "bar/b".into(),
            reference: Some("branch".into()),
        };
        let result = resource.resolve(&target);
        assert_eq!(
            result.unwrap().path(),
            &ResPath::new_remote_unchecked(&target.base_url().unwrap(), "bar/b")
        );
    }
}
