//! Pack the `entry-point` property of the project.

use std::collections::HashMap;

use serde_json::Value;

use crate::util::async_for;
use crate::{prop, json::Coerce};
use crate::json::Cast;
use crate::types::EntryPoints;

use super::{PackerError, PackerResult};

/// Converts the entry point map property value to [`EntryPoints`] object
pub async fn pack_entry_points(value: Value) -> PackerResult<EntryPoints> {
    let obj = value.try_into_object().map_err(|_|
        PackerError::InvalidMetadataPropertyType(prop::ENTRY_POINTS.to_string())
    )?;

    let mut map = HashMap::with_capacity(obj.len());
    let _ = async_for!((key, value) in obj, {
        let value = value.coerce_to_string();
        map.insert(key, value);
    });

    let _ = async_for!((key, value) in &map, {
        let valid = if value.is_empty() {
            false
        } else {
            value.starts_with('/') || resolve_alias(&map, value, 0)?.is_some()
        };
        if !valid {
            return Err(PackerError::InvalidEntryPoint(key.to_string(), value.to_string()));
        }
    });

    Ok(EntryPoints(map))
}

const MAX_DEPTH: usize = 16;
fn resolve_alias(map: &HashMap<String, String>, key: &str, depth: usize) -> PackerResult<Option<String>> {
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

#[cfg(test)]
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
            assert_eq!(result, Err(PackerError::InvalidMetadataPropertyType(prop::ENTRY_POINTS.to_string())));
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
        })).await;
        assert_eq!(result, Err(PackerError::InvalidEntryPoint("test".to_string(), "".to_string())));
    }

    #[tokio::test]
    async fn test_invalid_path() {
        let result = pack_entry_points(json!({
            "test": "something"
        })).await;
        assert_eq!(result, Err(PackerError::InvalidEntryPoint("test".to_string(), "something".to_string())));
    }

    #[tokio::test]
    async fn test_recursive() {
        let result = pack_entry_points(json!({
            "test": "test"
        })).await;
        assert_eq!(result, Err(PackerError::MaxEntryPointDepthExceeded("test".to_string())));
        let result = pack_entry_points(json!({
            "test1": "test2",
            "test2": "test1",
        })).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_valid() {
        let result = pack_entry_points(json!({
            "test": "/something",
            "test2": "test"
        })).await;
        let expected = vec![
("test".to_string(), "/something".to_string()), 
("test2".to_string(), "test".to_string())].into_iter().collect::<HashMap<_,_>>();

        assert_eq!(result, Ok(EntryPoints(expected)));
    }
}
