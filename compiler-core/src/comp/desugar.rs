use std::collections::BTreeMap;

use serde_json::{json, Map, Value};

use crate::json::Coerce;
use crate::macros::test_suite;
use crate::prop;

use super::CompError;

type DesugarLine = (String, Map<String, Value>);
type DesugarLineError = (String, CompError);

/// Desugar a line as a json blob
///
/// Valid line formats:
/// - object with one key, and value is an object
/// - string (desugared to `{[value]: {}}`)
/// - null (desugared to `{"": {}}`)
/// - boolean, number (desugared to string representation)
pub fn desugar_line(value: Value) -> Result<DesugarLine, DesugarLineError> {
    let text = value.coerce_to_string();
    match value {
        Value::Array(_) => Err((text, CompError::ArrayCannotBeLine)),
        Value::Object(obj) => {
            let mut iter = obj.into_iter();
            let (key, obj) = match iter.next() {
                None => {
                    return Err((text, CompError::EmptyObjectCannotBeLine));
                }
                Some(first) => first,
            };
            if iter.next().is_some() {
                return Err((text, CompError::TooManyKeysInObjectLine));
            }
            let properties = match obj {
                Value::Object(map) => map,
                _ => {
                    return Err((text, CompError::LinePropertiesMustBeObject));
                }
            };
            Ok((key, properties))
        }
        _ => Ok((text, Map::new())),
    }
}

/// Desugar properties on a line
///
/// Some properties like `coord` are simply short-hand for other properties.
/// They are converted to their long-hand form here.
pub fn desugar_properties(properties: &mut BTreeMap<String, Value>) {
    if let Some(value) = properties.remove(prop::COORD) {
        properties.insert(prop::MOVEMENTS.to_string(), json!([value]));
    }
    if let Some(value) = properties.remove(prop::ICON) {
        properties.insert(prop::ICON_DOC.to_string(), value.clone());
        properties.insert(prop::ICON_MAP.to_string(), value);
    }
}

#[test_suite]
mod test {
    use super::*;

    #[test]
    fn test_line_primitive() {
        assert_eq!(desugar_line(json!(null)), Ok(("".to_string(), Map::new())));
        assert_eq!(
            desugar_line(json!(true)),
            Ok(("true".to_string(), Map::new()))
        );
        assert_eq!(
            desugar_line(json!(false)),
            Ok(("false".to_string(), Map::new()))
        );
        assert_eq!(desugar_line(json!("")), Ok(("".to_string(), Map::new())));
        assert_eq!(
            desugar_line(json!("hello world")),
            Ok(("hello world".to_string(), Map::new()))
        );
    }

    #[test]
    fn test_line_array() {
        assert_eq!(
            desugar_line(json!([])),
            Err(("[object array]".to_string(), CompError::ArrayCannotBeLine))
        );
    }

    #[test]
    fn test_line_object_invalid() {
        let str = "[object object]";
        assert_eq!(
            desugar_line(json!({})),
            Err((str.to_string(), CompError::EmptyObjectCannotBeLine))
        );
        assert_eq!(
            desugar_line(json!({"one":"two", "three":"four" })),
            Err((str.to_string(), CompError::TooManyKeysInObjectLine))
        );
        assert_eq!(
            desugar_line(json!({"one": []})),
            Err((str.to_string(), CompError::LinePropertiesMustBeObject))
        );
    }

    #[test]
    fn test_line_object_valid() {
        assert_eq!(
            desugar_line(json!({"one": {
                "two": "three"
            }})),
            Ok((
                "one".to_string(),
                [{ ("two".to_string(), json!("three")) }]
                    .into_iter()
                    .collect()
            ))
        );
    }

    #[test]
    fn test_properties_empty() {
        let mut properties = BTreeMap::new();
        desugar_properties(&mut properties);
        assert_eq!(properties, BTreeMap::new());
    }

    #[test]
    fn test_properties_coord() {
        let mut properties = BTreeMap::new();
        properties.insert("coord".to_string(), json!([1, 2]));
        desugar_properties(&mut properties);
        assert!(properties.get("coord").is_none());
        assert_eq!(properties.get("movements").unwrap(), &json!([[1, 2]]));
    }

    #[test]
    fn test_properties_icon() {
        let mut properties = BTreeMap::new();
        properties.insert("icon".to_string(), json!([1, 2]));
        desugar_properties(&mut properties);
        assert!(properties.get("icon").is_none());
        assert_eq!(properties.get("icon-doc").unwrap(), &json!([1, 2]));
        assert_eq!(properties.get("icon-map").unwrap(), &json!([1, 2]));
    }
}
