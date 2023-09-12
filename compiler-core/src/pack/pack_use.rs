//! Implementation of the `use` property

use serde_json::Value;

use crate::comp::prop;
use crate::json::Coerce;

/// Result of parsing an object which could be loading a resource with
/// the `use` property
#[derive(Debug, PartialEq, Clone)]
pub enum Use {
    /// Correctly formed `use` property
    Valid(ValidUse),
    /// Not loading a resource
    NotUse(Value),
    /// Invalid path specified in the use property
    Invalid(String),
}

/// Correctly formed `use` property
#[derive(Debug, PartialEq, Clone)]
pub enum ValidUse {
    /// Loading a resource using relative path
    Relative(String),
    /// Loading a resource using absolute path
    Absolute(String),
    /// Loading a resource using remote path
    Remote {
        owner: String,
        repo: String,
        path: String,
        reference: Option<String>,
    },
}

impl From<Value> for Use {
    fn from(value: Value) -> Self {
        let obj = match value.as_object() {
            Some(obj) => obj,
            None => return Self::NotUse(value),
        };
        let mut iter = obj.iter();
        let (key, v) = match iter.next() {
            Some((key, value)) => (key, value),
            None => return Self::NotUse(value),
        };
        if iter.next().is_some() {
            return Self::NotUse(value);
        }
        if key != prop::USE {
            return Self::NotUse(value);
        }
        let v = v.coerce_to_string();
        if v.starts_with("/") {
            if v.ends_with("/") {
                Self::Invalid(v)
            } else {
                Self::Valid(ValidUse::Absolute(v))
            }
        } else if v.starts_with("./") || v.starts_with("../") {
            if v.ends_with("/") {
                Self::Invalid(v)
            } else {
                Self::Valid(ValidUse::Relative(v))
            }
        } else {
            let mut reference_split = v.splitn(2, ':');
            // unwrap is safe because we know there is at least one element
            let path = reference_split.next().unwrap();
            if path.ends_with("/") {
                return Self::Invalid(v);
            }

            let reference = reference_split.next().filter(|s| !s.is_empty());
            let mut path_split = path.splitn(3, '/');
            let owner = match path_split.next() {
                Some(owner) => owner,
                None => return Self::Invalid(v),
            };
            let repo = match path_split.next() {
                Some(repo) => repo,
                None => return Self::Invalid(v),
            };
            let path = match path_split.next() {
                Some(path) => path,
                None => return Self::Invalid(v),
            };
            Self::Valid(ValidUse::Remote {
                owner: owner.to_string(),
                repo: repo.to_string(),
                path: path.to_string(),
                reference: reference.map(|s| s.to_string()),
            })
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
            assert_eq!(Use::from(test.clone()), Use::NotUse(test));
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
            assert_eq!(Use::from(test.clone()), Use::NotUse(test));
        }
    }

    #[test]
    fn test_use_relative() {
        assert_eq!(
            Use::from(json!({
                "use": "./hello"
            })),
            Use::Valid(ValidUse::Relative("./hello".to_string()))
        );
        assert_eq!(
            Use::from(json!({
                "use": "../foo/hello"
            })),
            Use::Valid(ValidUse::Relative("../foo/hello".to_string()))
        );
    }

    #[test]
    fn test_use_absolute() {
        assert_eq!(
            Use::from(json!({
                "use": "/hello"
            })),
            Use::Valid(ValidUse::Absolute("/hello".to_string()))
        );
        assert_eq!(
            Use::from(json!({
                "use": "/foo/hello"
            })),
            Use::Valid(ValidUse::Absolute("/foo/hello".to_string()))
        );
        assert_eq!(
            Use::from(json!({
                "use": "//foo/hello"
            })),
            Use::Valid(ValidUse::Absolute("//foo/hello".to_string()))
        );
    }

    #[test]
    fn test_use_remote() {
        assert_eq!(
            Use::from(json!({
                "use": "foo/hello/bar"
            })),
            Use::Valid(ValidUse::Remote {
                owner: "foo".to_string(),
                repo: "hello".to_string(),
                path: "bar".to_string(),
                reference: None,
            })
        );
        assert_eq!(
            Use::from(json!({
                "use": "foo/hello/bar:test"
            })),
            Use::Valid(ValidUse::Remote {
                owner: "foo".to_string(),
                repo: "hello".to_string(),
                path: "bar".to_string(),
                reference: Some("test".to_string()),
            })
        );
        assert_eq!(
            Use::from(json!({
                "use": ".foo/hello/bar/giz"
            })),
            Use::Valid(ValidUse::Remote {
                owner: ".foo".to_string(),
                repo: "hello".to_string(),
                path: "bar/giz".to_string(),
                reference: None,
            })
        );
        assert_eq!(
            Use::from(json!({
                "use": "foo/hello/bar/giz/biz:test"
            })),
            Use::Valid(ValidUse::Remote {
                owner: "foo".to_string(),
                repo: "hello".to_string(),
                path: "bar/giz/biz".to_string(),
                reference: Some("test".to_string()),
            })
        );
        assert_eq!(
            Use::from(json!({
                "use": "foo/hello/bar/giz/biz:"
            })),
            Use::Valid(ValidUse::Remote {
                owner: "foo".to_string(),
                repo: "hello".to_string(),
                path: "bar/giz/biz".to_string(),
                reference: None,
            })
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
            assert_eq!(Use::from(make_use(&test)), Use::Invalid(test.to_string()));
        }
    }
}
