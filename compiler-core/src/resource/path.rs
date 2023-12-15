//! Resource path implementation
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

use crate::util::{Path, PathBuf, RefCounted};

/// Path of a resource in the context of a certain project, either local or remote (URL)
///
/// When manipulating or constructing resource paths, operations that don't require copying
/// may produce a borrowed path. This is indicated by the lifetime parameter.
///
/// Calling `to_string()` will result in either the relative path from the root, or the URL of the
/// resource.
#[derive(Debug, Clone, PartialEq)]
pub enum ResPath<'url, 'path> {
    /// Local path, represented as a relative path from the "root"
    /// (i.e. without the leading "/")
    Local(Cow<'path, Path>),
    /// Remote path, represented as a URL prefix (with a trailing "/")
    /// and a relative path (without the leading "/")
    Remote(Cow<'url, str>, Cow<'path, Path>),
}

impl<'url, 'path> ResPath<'url, 'path> {
    /// Create a new local resource path.
    ///
    /// Converts the path to a relative path if it is absolute.
    pub fn new_local<P>(path: P) -> Self where P: AsRef<Path> {
        let path = match path.as_ref().strip_prefix("/") {
            Ok(path) => Cow::from(path),
            _ => Cow::from(path.as_ref()),
        };

        Self::Local(path)
    }

    /// Create a new remote resource path.
    ///
    /// Converts the path to a relative path if it is absolute.
    pub fn new_remote<P>(url: &str, path: P) -> Self where P: AsRef<Path>{
        let path = match path.as_ref().strip_prefix("/") {
            Ok(path) => Cow::from(path),
            _ => Cow::from(path.as_ref()),
        };

        let url = if url.ends_with('/') {
            Cow::from(url)
        } else {
            let mut url: String = url.to_owned();
            url.push('/');
            Cow::from(url)
        };

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

    /// Get the parent path. Returns `None` if the path is at the root
    pub fn parent(&self) -> Option<Self> {
        match self {
            Self::Local(path) => {
                match path.parent() {
                    Some(path) => Some(Self::Local(Cow::from(path))),
                    None => None,
                }
            }
            Self::Remote(url, path) => {
                match path.parent() {
                    Some(path) => Some(Self::Remote(Cow::clone(url), Cow::from(path))),
                    None => None,
                }
            }
        }
    }

    /// Join a path to the current path, "." and ".."s are normalized away
    ///
    /// Returns `None` if the path tries to get the parent of root at any point
    pub fn join_resolve<P>(&self, path: P) -> Option<Self> where P: AsRef<Path> {
        todo!()
    }
}

impl<'a, 'b> Display for ResPath<'a, 'b> {
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
        let path = ResPath::new_local(&path);
        assert_eq!(path.to_string(), "");

        let path = "/";
        let path = ResPath::new_local(&path);
        assert_eq!(path.to_string(), "");

        let path = "/test/path";
        let path = ResPath::new_local(&path);
        assert_eq!(path.to_string(), "test/path");

        let path = "test/path";
        let path = ResPath::new_local(&path);
        assert_eq!(path.to_string(), "test/path");
    }

    #[test]
    fn test_new_remote() {
        let path = "";
        let path = ResPath::new_remote("https://hello/", &path);
        assert_eq!(path.to_string(), "https://hello/");

        let path = "foo";
        let path = ResPath::new_remote("https://hello", &path);
        assert_eq!(path.to_string(), "https://hello/foo");

        let path = "/test/path";
        let path = ResPath::new_remote("https://hello/", &path);
        assert_eq!(path.to_string(), "https://hello/test/path");
    }
}
