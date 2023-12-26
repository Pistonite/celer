use std::borrow::Cow;
use std::collections::BTreeMap;

use serde_json::{json, Map, Value};

use crate::comp::CompResult;
use crate::json::{SafeRouteObject, SafeRouteBlob, Coerce, Cast};
use crate::prop;

use super::CompError;

/// Desugar a line as a json blob
///
/// Valid line formats:
/// - object with one key, and value is an object
/// - string (desugared to `{[value]: {}}`)
/// - null (desugared to `{"": {}}`)
/// - boolean, number (desugared to string representation)
pub fn desugar_line<'a>(value: SafeRouteBlob<'a>) -> (Cow<'a, str>, CompResult<SafeRouteObject<'a>>) {
    if value.is_array() {
        return (value.coerce_into_string().into(), Err(CompError::ArrayCannotBeLine));
    }
    match value.try_into_object() {
        Err(value) => (value.coerce_into_string().into(), Ok(SafeRouteObject::new())),
        Ok(obj) => {
            let mut iter = obj.into_iter();
            let (key, obj) = match iter.next() {
                None => {
                    return ("[object object]".into(), Err(CompError::EmptyObjectCannotBeLine));
                }
                Some(first) => first,
            };
            if iter.next().is_some() {
                return ("[object object]".into(), Err(CompError::TooManyKeysInObjectLine));
            }
            match obj.try_into_object() {
                Ok(obj) => {
                    (key, Ok(obj))
                }
                Err(value) => {
                    (value.coerce_into_string().into(), Err(CompError::LinePropertiesMustBeObject))
                }
            }
        }
    }
}

// /// Desugar properties on a line
// ///
// /// Some properties like `coord` are simply short-hand for other properties.
// /// They are converted to their long-hand form here.
// pub fn desugar_properties(properties: &mut BTreeMap<String, Value>) {
//     if let Some(value) = properties.remove(prop::COORD) {
//         properties.insert(prop::MOVEMENTS.to_string(), json!([value]));
//     }
//     if let Some(value) = properties.remove(prop::ICON) {
//         properties.insert(prop::ICON_DOC.to_string(), value.clone());
//         properties.insert(prop::ICON_MAP.to_string(), value);
//     }
// }

#[cfg(test)]
mod test {
    use crate::json::IntoSafeRouteBlob;

    use super::*;

    fn test_desugar_line(value: Value) -> (String, CompResult<Value>) {
        let (text, result) = desugar_line(value.into_unchecked());
        let result = result.map(|x| {
            let mut map = Map::new();
            for (k, v) in x {
                map.insert(k.into_owned(), v.into());
            }
            Value::Object(map)
        });
        (text.to_string(), result)
    }

    #[test]
    fn test_line_primitive() {
        assert_eq!(test_desugar_line(json!(null)), ("".to_string(), Ok(json!({}))));
        assert_eq!(
            test_desugar_line(json!(true)),
            ("true".to_string(), Ok(json!({})))
        );
        assert_eq!(
            test_desugar_line(json!(false)),
            ("false".to_string(), Ok(json!({})))
        );
        assert_eq!(test_desugar_line(json!("")), ("".to_string(), Ok(json!({}))));
        assert_eq!(
            test_desugar_line(json!("hello world")),
            ("hello world".to_string(), Ok(json!({})))
        );
    }

    #[test]
    fn test_line_array() {
        assert_eq!(
            test_desugar_line(json!([])),
            ("[object array]".to_string(), Err(CompError::ArrayCannotBeLine))
        );
    }

    #[test]
    fn test_line_object_invalid() {
        let str = "[object object]";
        assert_eq!(
            test_desugar_line(json!({})),
            (str.to_string(), Err(CompError::EmptyObjectCannotBeLine))
        );
        assert_eq!(
            test_desugar_line(json!({"one":"two", "three":"four" })),
            (str.to_string(), Err(CompError::TooManyKeysInObjectLine))
        );
        assert_eq!(
            test_desugar_line(json!({"one": []})),
            ("[object array]".to_string(), Err(CompError::LinePropertiesMustBeObject))
        );
    }

    #[test]
    fn test_line_object_valid() {
        assert_eq!(
            test_desugar_line(json!({"one": {
                "two": "three"
            }})),
            (
                "one".to_string(),
                Ok(
                    [{ ("two".to_string(), json!("three")) }]
                        .into_iter()
                        .collect()
                )
            )
        );
    }

}
