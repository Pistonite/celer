// /// Desugar a line as a json blob
// ///
// pub async fn desugar_line(value: Value) -> Value {
// }--features build-binary
// fn convert_line_to_object(
//     value: value,
// ) -> result<(string, serde_json::map<string, value>), (compline, vec<compilererror>)> {
//     let text = value.coerce_to_string();
//     match value {
//         value::array(_) => make_error!(&text, compilererror::arraycannotbeline),
//         value::object(obj) => {
//             let mut iter = obj.into_iter();
//             let (key, obj) = match iter.next() {
//                 none => {
//                     return make_error!(&text, compilererror::emptyobjectcannotbeline);
//                 }
//                 some(first) => first,
//             };
//             if iter.next().is_some() {
//                 return make_error!(&text, compilererror::toomanykeysinobjectline);
//             }
//             let properties = match obj {
//                 value::object(map) => map,
//                 _ => {
//                     return make_error!(&text, compilererror::linepropertiesmustbeobject);
//                 }
//             };
//             ok((key, properties))
//         }
//         _ => ok((value.coerce_to_string(), serde_json::map::new())),
//     }
// }

use std::collections::BTreeMap;

use serde_json::{Value, json};

/// Desugar properties on a line
///
/// Some properties like `coord` are simply short-hand for other properties.
/// They are converted to their long-hand form here.
pub async fn desugar(properties: &mut BTreeMap<String, Value>) {
    if let Some(value) = properties.remove("coord") {
        properties.insert("movements".to_string(), json!([value]));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_empty() {
        let mut properties = BTreeMap::new();
        desugar(&mut properties).await;
        assert_eq!(properties, BTreeMap::new());
    }

    #[tokio::test]
    async fn test_coord() {
        let mut properties = BTreeMap::new();
        properties.insert("coord".to_string(), json!([1, 2]));
        desugar(&mut properties).await;
        assert!(properties.get("coord").is_none());
        assert_eq!(properties.get("movements").unwrap(), &json!([[1, 2]]));
    }
}
