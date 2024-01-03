use std::borrow::Cow;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::json::Cast;
use crate::macros::derive_wasm;
use crate::prop;
use crate::util::StringMap;

use super::{PrepError, PrepResult, Setting};

/// Compiler entry points (name, path) pairs
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct EntryPoints(pub StringMap<String>);

impl EntryPoints {
    /// Remove the aliases. Only keep the entry points that map directly to a path
    pub fn path_only(mut self) -> Self {
        self.0.retain(|_, v| v.starts_with('/'));
        self
    }

    /// Resolve the alias to get the path
    pub fn resolve_alias<'a>(&self, key: &'a str, setting: &Setting) -> PrepResult<Cow<'a, str>> {
        if key.starts_with('/') {
            return Ok(key.into());
        }
        resolve_alias(&self.0, key, 0, setting).map(|v| v.into())
    }

    pub fn get_default(&self) -> Option<&str> {
        self.0.get(prop::DEFAULT).map(|v| v.as_str())
    }
}

#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct EntryPointsSorted(pub Vec<(String, String)>);

impl From<EntryPoints> for EntryPointsSorted {
    fn from(entry_points: EntryPoints) -> Self {
        let map: BTreeMap<String, String> = entry_points.0.into();
        let mut vec = map.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        Self(vec)
    }
}

/// Converts the entry point map property value to [`EntryPoints`] object
pub async fn load_entry_points(value: Value, setting: &Setting) -> PrepResult<EntryPoints> {
    let obj = value.try_into_object().map_err(|_| {
        PrepError::InvalidMetadataPropertyType(prop::ENTRY_POINTS, "mapping object")
    })?;

    // let mut map = BTreeMap::new();
    // for (key, value) in obj {
    //     yield_budget(64).await;
    //     let value = value.coerce_to_string();
    //     // let path = if value.starts_with('/') {
    //     //     match Path::try_from(&value) {
    //     //         Some(path) => format!("/{path}"),
    //     //         None => value,
    //     //     }
    //     // } else {
    //     //     value
    //     // };
    //
    //     map.insert(key, value);
    // }
    //
    // for (key, value) in &map {
    //     yield_budget(64).await;
    //     let valid = if value.is_empty() {
    //         false
    //     } else {
    //         value.starts_with('/') || resolve_alias(&map, value, 0, setting)?.is_some()
    //     };
    //     if !valid {
    //         return Err(PrepError::InvalidEntryPoint(
    //             key.to_string(),
    //             value.to_string(),
    //         ));
    //     }
    // }
    //
    // Ok(EntryPoints(map))
    todo!()
}

fn resolve_alias(
    map: &BTreeMap<String, String>,
    key: &str,
    depth: usize,
    setting: &Setting,
) -> PrepResult<String> {
    // if depth > setting.max_entry_point_depth {
    //     return Err(PrepError::MaxEntryPointDepthExceeded(key.to_string()));
    // }
    // match map.get(key) {
    //     Some(value) => {
    //         if value.starts_with('/') {
    //             Ok(Some(value.to_string()))
    //         } else {
    //             resolve_alias(map, value, depth + 1, setting)
    //         }
    //     }
    //     None => Ok(None),
    // }
    todo!()
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    async fn load_entry_points_test(value: Value) -> PrepResult<EntryPoints> {
        load_entry_points(value, &Setting::default()).await
    }

    #[tokio::test]
    async fn test_non_object() {
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
            let result = load_entry_points_test(test).await;
            assert_eq!(
                result,
                Err(PrepError::InvalidMetadataPropertyType(
                    prop::ENTRY_POINTS,
                    "mapping object"
                ))
            );
        }
    }

    #[tokio::test]
    async fn test_empty_object() {
        let result = load_entry_points_test(json!({})).await;
        assert_eq!(result, Ok(Default::default()));
    }

    #[tokio::test]
    async fn test_empty_path() {
        let result = load_entry_points_test(json!({
            "test": ""
        }))
        .await;
        assert_eq!(
            result,
            Err(PrepError::InvalidEntryPoint(
                "test".to_string(),
                "".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn test_invalid_path() {
        let result = load_entry_points_test(json!({
            "test": "something"
        }))
        .await;
        assert_eq!(
            result,
            Err(PrepError::InvalidEntryPoint(
                "test".to_string(),
                "something".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn test_invalid_parent() {
        let result = load_entry_points_test(json!({
            "test": "../something"
        }))
        .await;
        assert_eq!(
            result,
            Err(PrepError::InvalidEntryPoint(
                "test".to_string(),
                "../something".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn test_recursive() {
        let result = load_entry_points_test(json!({
            "test": "test"
        }))
        .await;
        assert_eq!(
            result,
            Err(PrepError::MaxEntryPointDepthExceeded("test".to_string()))
        );
        let result = load_entry_points_test(json!({
            "test1": "test2",
            "test2": "test1",
        }))
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_valid() {
        let result = load_entry_points_test(json!({
            "test": "/something",
            "test2": "test"
        }))
        .await;
        let expected = vec![
            ("test".to_string(), "/something".to_string()),
            ("test2".to_string(), "test".to_string()),
        ]
        .into_iter()
        .collect::<BTreeMap<_, _>>();

        assert_eq!(result, Ok(EntryPoints(expected.into())));
    }

    #[tokio::test]
    async fn test_normalize() {
        let result = load_entry_points_test(json!({
            "test": "//something",
            "test2": "/test/../root"
        }))
        .await;
        let expected = vec![
            ("test".to_string(), "/something".to_string()),
            ("test2".to_string(), "/root".to_string()),
        ]
        .into_iter()
        .collect::<BTreeMap<_, _>>();

        assert_eq!(result, Ok(EntryPoints(expected.into())));
    }
}
