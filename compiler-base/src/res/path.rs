//! Resource path implementation
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use crate::util::{Path, PathBuf, Component};

/// Path of a resource in the context of a certain project, either local or remote (URL)
///
/// When manipulating or constructing resource paths, operations that don't require copying
/// may produce a borrowed path. This is indicated by the lifetime parameter.
///
/// Calling `to_string()` will result in either the relative path from the root, or the URL of the
/// resource.
///
/// # Lifetime
/// The lifetime parameter `'u` is the lifetime of the URL prefix for remote paths.
/// It may contain a borrowed URL prefix, since the URL prefix is only modified when switching
/// between remote repositories.
#[derive(Debug, Clone, PartialEq)]
pub enum ResPath<'u> {
    /// Local path, represented as a relative path from the "root"
    /// (i.e. without the leading "/")
    Local(PathBuf),
    /// Remote path, represented as a URL prefix (with a trailing "/")
    /// and a relative path (without the leading "/")
    Remote(Cow<'u, str>, PathBuf),
}

impl<'u> ResPath<'u> {
    /// Create a new local resource path.
    ///
    /// # Requirements
    /// Path must be relative.
    pub fn new_local<P>(path: P) -> Option<Self> where P: AsRef<Path> {
        Self::new_local_unchecked("").join_resolve(path)
    }

    /// Create a new local resource path without normalizing the path.
    pub fn new_local_unchecked<P>(path: P) -> Self where P: Into<PathBuf> {
        let path = path.into();
        debug_assert!(path.is_relative());
        Self::Local(path)
    }

    /// Create a new remote resource path.
    ///
    /// # Requirements
    /// 1. Path must be relative.
    /// 2. Url must end with a trailing "/".
    pub fn new_remote<U, P>(url: U, path: P) -> Option<Self>
where 
        U: Into<Cow<'u, str>>,
        P: AsRef<Path>{
        Self::new_remote_unchecked(url, "").join_resolve(path)
    }

    /// Create a new remote resource path without normalizing the path.
    pub fn new_remote_unchecked<U, P>(url: U, path: P) -> Self 
where 
        U: Into<Cow<'u, str>>,
        P: Into<PathBuf>
    {
        let url = url.into();
        debug_assert!(url.ends_with('/'));
        let path = path.into();
        debug_assert!(path.is_relative());

        Self::Remote(url, path)
    }

    /// Get if the path is local
    #[inline]
    pub fn is_local(&self) -> bool {
        match self {
            Self::Local(_) => true,
            Self::Remote(_, _) => false,
        }
    }

    /// Get the path as a `Path` for manipulation
    pub fn as_path(&self) -> &Path {
        match self {
            Self::Local(path) => path.as_path(),
            Self::Remote(_, path) => path.as_path(),
        }
    }

    /// Join a path to the current path, "." and ".."s are normalized away. The path must be
    /// relative
    ///
    /// Returns `None` if the path tries to get the parent of root at any point,
    /// or if the final path is the root
    pub fn join_resolve<P>(&self, path: P) -> Option<Self> where P: AsRef<Path> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        let mut new_path = self.as_path().to_path_buf();
        for c in path.components() {
            match c {
                Component::ParentDir => {
                    if !new_path.pop() {
                        return None;
                    }
                }
                Component::Normal(c) => new_path.push(c),
                _ => {}
            }
        }

        if new_path == Path::new("") {
            return None;
        }

        let new_path = match self {
            Self::Local(_) => Self::new_local_unchecked(new_path),
            Self::Remote(url, _) => Self::new_remote_unchecked(url.clone(), new_path),
        };

        Some(new_path)
    }
}

impl<'a> Display for ResPath<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(path) => write!(f, "{}", path.to_str()),
            Self::Remote(url, path) => write!(f, "{}{}", url, path.to_str()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_local() {
        let path = "";
        let path = ResPath::new_local_unchecked(&path);
        assert_eq!(path.to_string(), "");

        let path = "test/path";
        let path = ResPath::new_local_unchecked(&path);
        assert_eq!(path.to_string(), "test/path");
    }

    #[test]
    fn test_new_remote() {
        let path = "";
        let path = ResPath::new_remote_unchecked("https://hello/", &path);
        assert_eq!(path.to_string(), "https://hello/");

        let path = "test/path";
        let path = ResPath::new_remote_unchecked("https://hello/", &path);
        assert_eq!(path.to_string(), "https://hello/test/path");
    }

    #[test]
    fn test_join_empty() {
        let path = ResPath::new_local_unchecked("");
        let path = path.join_resolve("");
        assert_eq!(path, None);

        let path = ResPath::new_local_unchecked("");
        let path = path.join_resolve("test/path");
        assert_eq!(path, Some(ResPath::new_local_unchecked("test/path")));

        let path = ResPath::new_local_unchecked("test/path");
        let path = path.join_resolve("");
        assert_eq!(path, Some(ResPath::new_local_unchecked("test/path")));
    }

    #[test]
    fn test_join_local() {
        let path = ResPath::new_local_unchecked("test/path");
        let path = path.join_resolve("test/path");
        assert_eq!(path, Some(ResPath::new_local_unchecked("test/path/test/path")));

        let path = ResPath::new_local_unchecked("test/path");
        let path = path.join_resolve("test/path/../.");
        assert_eq!(path, Some(ResPath::new_local_unchecked("test/path/test")));

        let path = ResPath::new_local_unchecked("test/path");
        let path = path.join_resolve("test/path/../..");
        assert_eq!(path, Some(ResPath::new_local_unchecked("test/path")));

        let path = ResPath::new_local_unchecked("test/path");
        let path = path.join_resolve("test/../../..");
        assert_eq!(path, None);

        let path = ResPath::new_local_unchecked("test/path");
        let path = path.join_resolve("test/path/../../..");
        assert_eq!(path, Some(ResPath::new_local_unchecked("test")));
    }

}
