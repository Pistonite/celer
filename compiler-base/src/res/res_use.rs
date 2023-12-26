//! Implementation of the `use` property

use std::borrow::Cow;
use std::fmt::Display;

use serde_json::Value;

use crate::json::Coerce;
use crate::prop;

/// Result of parsing an object which could be loading a resource with
/// the `use` property
#[derive(Debug, PartialEq, Clone)]
pub enum Use {
    /// Correctly formed `use` property
    Valid(ValidUse),
    /// Invalid path specified in the use property
    Invalid(String),
}

/// Correctly formed `use` property
#[derive(Debug, PartialEq, Clone)]
pub enum ValidUse {
    /// Loading a resource using relative path. The string must start with `./` or `../`
    Relative(String),
    /// Loading a resource using absolute path. The string must start with `/`
    Absolute(String),
    /// Loading a resource using remote path
    Remote {
        owner: String,
        repo: String,
        path: String,
        reference: Option<String>,
    },
}

impl Display for Use {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Use::Valid(v) => write!(f, "{}", v),
            Use::Invalid(v) => write!(f, "{}", v),
        }
    }
}

impl Display for ValidUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidUse::Relative(v) => write!(f, "{}", v),
            ValidUse::Absolute(v) => write!(f, "{}", v),
            ValidUse::Remote {
                owner,
                repo,
                path,
                reference,
            } => {
                write!(f, "{}/{}/{}", owner, repo, path)?;
                if let Some(reference) = reference {
                    write!(f, ":{}", reference)?;
                }
                Ok(())
            }
        }
    }
}

impl Use {
    /// Convert a path in the `use` property to a Use object
    ///
    /// If the path is malformed, this returns a [`Use::Invalid`]
    pub fn new<'a, S>(v: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let v = v.into();
        if v.starts_with('/') {
            if v.ends_with('/') {
                Self::Invalid(v.into_owned())
            } else {
                Self::Valid(ValidUse::Absolute(v.into_owned()))
            }
        } else if v.starts_with("./") || v.starts_with("../") {
            if v.ends_with('/') {
                Self::Invalid(v.into_owned())
            } else {
                Self::Valid(ValidUse::Relative(v.into_owned()))
            }
        } else {
            let mut reference_split = v.splitn(2, ':');
            // unwrap is safe because we know there is at least one element
            let path = reference_split.next().unwrap();
            if path.ends_with('/') {
                return Self::Invalid(v.into_owned());
            }

            let reference = reference_split.next().filter(|s| !s.is_empty());
            let mut path_split = path.splitn(3, '/');
            let owner = match path_split.next() {
                Some(owner) => owner,
                None => return Self::Invalid(v.into_owned()),
            };
            let repo = match path_split.next() {
                Some(repo) => repo,
                None => return Self::Invalid(v.into_owned()),
            };
            let path = match path_split.next() {
                Some(path) => path,
                None => return Self::Invalid(v.into_owned()),
            };
            Self::Valid(ValidUse::Remote {
                owner: owner.to_string(),
                repo: repo.to_string(),
                path: path.to_string(),
                reference: reference.map(|s| s.to_string()),
            })
        }
    }

    /// Try converting a json object in the form of `{ "use": "..." }` to a `Use`
    ///
    /// If the object is not in the correct form, the original value is returned.
    /// This includes if the object has keys other than `use`.
    ///
    /// If the object is in the correct form but the path is not, this returns an Ok variant with
    /// [`Use::Invalid`]
    pub fn from_value(value: &Value) -> Option<Self> {
        let obj = value.as_object()?;
        let mut iter = obj.iter();
        let (key, v) = iter.next()?;
        if iter.next().is_some() {
            return None;
        }
        if key != prop::USE {
            return None;
        }
        Some(Self::new(v.coerce_to_string()))
    }
}

impl ValidUse {
    pub fn path(&self) -> &str {
        match self {
            ValidUse::Relative(v) => v,
            ValidUse::Absolute(v) => v,
            ValidUse::Remote { path, .. } => path,
        }
    }

    /// Return the base URL if the variant is a Remote
    pub fn base_url(&self) -> Option<String> {
        match self {
            ValidUse::Relative(_) => None,
            ValidUse::Absolute(_) => None,
            ValidUse::Remote {
                owner,
                repo,
                reference,
                ..
            } => {
                let branch = reference.as_deref().unwrap_or("main");
                let url = format!("https://raw.githubusercontent.com/{owner}/{repo}/{branch}/");
                Some(url)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_non_object() {
        let tests = vec![
            json!(null),
            json!(1),
            json!("hello"),
            json!(true),
            json!(false),
            json!([]),
            json!([1, 2, 3]),
        ];

        for test in tests {
            assert_eq!(Use::from_value(&test), None);
        }
    }

    #[test]
    fn test_object_not_use() {
        let tests = vec![
            json!({}),
            json!({
                "hello": "world",
            }),
            json!({
                "use": null,
                "2": 3,
            }),
        ];

        for test in tests {
            assert_eq!(Use::from_value(&test), None);
        }
    }

    #[test]
    fn test_use_relative() {
        assert_eq!(
            Use::from_value(&json!({
                "use": "./hello"
            })),
            Some(Use::Valid(ValidUse::Relative("./hello".to_string())))
        );
        assert_eq!(
            Use::from_value(&json!({
                "use": "../foo/hello"
            })),
            Some(Use::Valid(ValidUse::Relative("../foo/hello".to_string())))
        );
    }

    #[test]
    fn test_use_absolute() {
        assert_eq!(
            Use::from_value(&json!({
                "use": "/hello"
            })),
            Some(Use::Valid(ValidUse::Absolute("/hello".to_string())))
        );
        assert_eq!(
            Use::from_value(&json!({
                "use": "/foo/hello"
            })),
            Some(Use::Valid(ValidUse::Absolute("/foo/hello".to_string())))
        );
        assert_eq!(
            Use::from_value(&json!({
                "use": "//foo/hello"
            })),
            Some(Use::Valid(ValidUse::Absolute("//foo/hello".to_string())))
        );
    }

    #[test]
    fn test_use_remote() {
        assert_eq!(
            Use::from_value(&json!({
                "use": "foo/hello/bar"
            })),
            Some(Use::Valid(ValidUse::Remote {
                owner: "foo".to_string(),
                repo: "hello".to_string(),
                path: "bar".to_string(),
                reference: None,
            }))
        );
        assert_eq!(
            Use::from_value(&json!({
                "use": "foo/hello/bar:test"
            })),
            Some(Use::Valid(ValidUse::Remote {
                owner: "foo".to_string(),
                repo: "hello".to_string(),
                path: "bar".to_string(),
                reference: Some("test".to_string()),
            }))
        );
        assert_eq!(
            Use::from_value(&json!({
                "use": ".foo/hello/bar/giz"
            })),
            Some(Use::Valid(ValidUse::Remote {
                owner: ".foo".to_string(),
                repo: "hello".to_string(),
                path: "bar/giz".to_string(),
                reference: None,
            }))
        );
        assert_eq!(
            Use::from_value(&json!({
                "use": "foo/hello/bar/giz/biz:test"
            })),
            Some(Use::Valid(ValidUse::Remote {
                owner: "foo".to_string(),
                repo: "hello".to_string(),
                path: "bar/giz/biz".to_string(),
                reference: Some("test".to_string()),
            }))
        );
        assert_eq!(
            Use::from_value(&json!({
                "use": "foo/hello/bar/giz/biz:"
            })),
            Some(Use::Valid(ValidUse::Remote {
                owner: "foo".to_string(),
                repo: "hello".to_string(),
                path: "bar/giz/biz".to_string(),
                reference: None,
            }))
        );
    }

    fn make_use(s: &str) -> Value {
        json!({
        "use": s
        })
    }

    #[test]
    fn test_invalid() {
        let tests = vec![
            "hello",
            "/",
            "./",
            "../",
            ".../hello",
            "foo/hello",
            "foo/hello/",
            "./foo/hello/",
            "../foo/hello/",
            "foo/hello/path/",
        ];

        for test in tests {
            assert_eq!(
                Use::from_value(&make_use(test)),
                Some(Use::Invalid(test.to_string()))
            );
        }
    }
}
