//! Pack the `entry-point` property of the project.

use serde_json::Value;

use crate::prop;
use crate::types::EntryPoints;

use super::PackerResult;

/// Converts the entry point map property value to [`EntryPoints`] object
pub async fn pack_entry_points(value: Value) -> PackerResult<EntryPoints> {
    todo!()
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::pack::PackerError;

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
}
