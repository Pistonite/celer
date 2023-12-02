//! Pack the `entry-point` property of the project.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::json::Cast;
use crate::json::Coerce;
use crate::macros::test_suite;
use crate::prop;
use crate::types::EntryPoints;
use crate::util::{yield_budget, Path};

use super::{PackerError, PackerResult};

/// Converts the entry point map property value to [`EntryPoints`] object
pub async fn pack_entry_points(value: Value) -> PackerResult<EntryPoints> {
    let obj = value
        .try_into_object()
        .map_err(|_| PackerError::InvalidMetadataPropertyType(prop::ENTRY_POINTS.to_string()))?;

    let mut map = BTreeMap::new();
    for (key, value) in obj {
        yield_budget(64).await;
        let value = value.coerce_to_string();
        let path = if value.starts_with('/') {
            match Path::try_from(&value) {
                Some(path) => format!("/{path}"),
                None => value,
            }
        } else {
            value
        };

        map.insert(key, path);
    }

    for (key, value) in &map {
        yield_budget(64).await;
        let valid = if value.is_empty() {
            false
        } else {
            value.starts_with('/') || resolve_alias(&map, value, 0)?.is_some()
        };
        if !valid {
            return Err(PackerError::InvalidEntryPoint(
                key.to_string(),
                value.to_string(),
            ));
        }
    }

    Ok(EntryPoints(map))
}

const MAX_DEPTH: usize = 16;
fn resolve_alias(
    map: &BTreeMap<String, String>,
    key: &str,
    depth: usize,
) -> PackerResult<Option<String>> {
    if depth > MAX_DEPTH {
        return Err(PackerError::MaxEntryPointDepthExceeded(key.to_string()));
    }
    match map.get(key) {
        Some(value) => {
            if value.starts_with('/') {
                Ok(Some(value.to_string()))
            } else {
                resolve_alias(map, value, depth + 1)
            }
        }
        None => Ok(None),
    }
}

#[test_suite]
mod test {
    use serde_json::json;

    use super::*;

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
            let result = pack_entry_points(test).await;
            assert_eq!(
                result,
                Err(PackerError::InvalidMetadataPropertyType(
                    prop::ENTRY_POINTS.to_string()
                ))
            );
        }
    }

    #[tokio::test]
    async fn test_empty_object() {
        let result = pack_entry_points(json!({})).await;
        assert_eq!(result, Ok(Default::default()));
    }

    #[tokio::test]
    async fn test_empty_path() {
        let result = pack_entry_points(json!({
            "test": ""
        }))
        .await;
        assert_eq!(
            result,
            Err(PackerError::InvalidEntryPoint(
                "test".to_string(),
                "".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn test_invalid_path() {
        let result = pack_entry_points(json!({
            "test": "something"
        }))
        .await;
        assert_eq!(
            result,
            Err(PackerError::InvalidEntryPoint(
                "test".to_string(),
                "something".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn test_invalid_parent() {
        let result = pack_entry_points(json!({
            "test": "../something"
        }))
        .await;
        assert_eq!(
            result,
            Err(PackerError::InvalidEntryPoint(
                "test".to_string(),
                "../something".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn test_recursive() {
        let result = pack_entry_points(json!({
            "test": "test"
        }))
        .await;
        assert_eq!(
            result,
            Err(PackerError::MaxEntryPointDepthExceeded("test".to_string()))
        );
        let result = pack_entry_points(json!({
            "test1": "test2",
            "test2": "test1",
        }))
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_valid() {
        let result = pack_entry_points(json!({
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

        assert_eq!(result, Ok(EntryPoints(expected)));
    }

    #[tokio::test]
    async fn test_normalize() {
        let result = pack_entry_points(json!({
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

        assert_eq!(result, Ok(EntryPoints(expected)));
    }
}
