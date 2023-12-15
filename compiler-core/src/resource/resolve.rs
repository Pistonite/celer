//! Implementation for resolving a `use` from a resource

use crate::util::{Path, RefCounted};

use super::{ResPath, Loader, Resource, ValidUse, ResResult, ResError};

impl<L> Resource<L> where L: Loader {

    /// Resolve a `use` property from this resource
    pub fn resolve(&self, target: &ValidUse) -> ResResult<Resource<L>> {
        // self.resolver.resolve(self, target).await
        todo!()
    }

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
        assert_eq!(result.unwrap().path(), ResPath::new_local("bar"));

        let resource = create_local_resource("foo/bar");
        let target = ValidUse::Relative("./biz".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), ResPath::new_local("foo/biz"));

        let resource = create_local_resource("foo/bar");
        let target = ValidUse::Relative("../biz".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), ResPath::new_local("biz"));
    }

    #[test]
    fn test_local_absolute() {
        let resource = create_local_resource("foo");
        let target = ValidUse::Absolute("/bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), ResPath::new_local("bar"));

        let resource = create_local_resource("foo/bar");
        let target = ValidUse::Absolute("/biz".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), ResPath::new_local("biz"));

        let resource = create_local_resource("foo/bar/woo");
        let target = ValidUse::Absolute("/biz/bar".into());
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), ResPath::new_local("biz/bar"));
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
        assert_eq!(result.unwrap().path(), ResPath::new_remote(&target.base_url().unwrap(), "bar"));

        let resource = create_local_resource("foo/a");
        let target = ValidUse::Remote {
            owner: "owner".into(),
            repo: "repo".into(),
            path: "bar/b".into(),
            reference: Some("branch".into()),
        };
        let result = resource.resolve(&target);
        assert_eq!(result.unwrap().path(), ResPath::new_remote(&target.base_url().unwrap(), "bar/b"));
    }

    #[test]
    fn test_remote_relative_error() {
        let resource = create_remote_resource("foo");
    }
    #[test]
    fn test_remote_relative() {
    }
    #[test]
    fn test_remote_absolute() {
    }
    #[test]
    fn test_remote_remote() {
    }
}

