//! Packs [`MapMetadata`]

use celerctypes::MapMetadata;
use serde_json::Value;

use super::{PackerResult, PackerError};

use crate::comp::prop;

macro_rules! check_map_required_property {
    ($property:expr, $index:ident, $value:expr) => {
        match $value {
            Some(v) => Ok(v),
            None => Err(PackerError::MissingConfigProperty($index, format!("{}.{}", prop::MAP, $property))),
        }
    }
}

/// Parses the `map` property in a config json blob
pub async fn pack_map(value: Value, index: usize) -> PackerResult<MapMetadata> {
    let mut map_obj = match value {
        Value::Object(o) => o,
        _ => return Err(PackerError::InvalidConfigProperty(index, prop::MAP.to_string())),
    };

    let layers = check_map_required_property!(prop::LAYERS, index, map_obj.remove(prop::LAYERS))?;
    let coord_map = check_map_required_property!(prop::COORD_MAP, index, map_obj.remove(prop::COORD_MAP))?;
    let initial_coord = check_map_required_property!(prop::INITIAL_COORD, index, map_obj.remove(prop::INITIAL_COORD))?;
    let initial_zoom = check_map_required_property!(prop::INITIAL_ZOOM, index, map_obj.remove(prop::INITIAL_ZOOM))?;
    let initial_color = check_map_required_property!(prop::INITIAL_COLOR, index, map_obj.remove(prop::INITIAL_COLOR))?;

    todo!()
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn test_invalid_value() {
        let values = vec![
            json!(null),
            json!(false),
            json!(true),
            json!(1),
            json!([]),
            json!(""),
            json!("hello"),
        ];

        for (i, v) in values.into_iter().enumerate() {
            let result = pack_map(v, i).await;
            assert_eq!(result, Err(PackerError::InvalidConfigProperty(i, prop::MAP.to_string())));
        }
    }

    #[tokio::test]
    async fn test_missing_properties() {
        let result = pack_map(json!({}), 0).await;
        assert_eq!(result, Err(PackerError::MissingConfigProperty(0, "map.layers".to_string())));

        let result = pack_map(json!({
            "layers": {}
        }), 0).await;
        assert_eq!(result, Err(PackerError::MissingConfigProperty(0, "map.coord-map".to_string())));

        let result = pack_map(json!({
            "layers": {},
            "coord-map": {}
        }), 0).await;
        assert_eq!(result, Err(PackerError::MissingConfigProperty(0, "map.initial-coord".to_string())));

        let result = pack_map(json!({
            "layers": {},
            "coord-map": {},
            "initial-coord": {}
        }), 0).await;
        assert_eq!(result, Err(PackerError::MissingConfigProperty(0, "map.initial-zoom".to_string())));

        let result = pack_map(json!({
            "layers": {},
            "coord-map": {},
            "initial-coord": {},
            "initial-zoom": {},
        }), 0).await;
        assert_eq!(result, Err(PackerError::MissingConfigProperty(0, "map.initial-color".to_string())));
    }

    #[tokio::test]
    async fn test_invalid_property_types() {
        let result = pack_map(json!({
            "layers": {},
            "coord-map": {},
            "initial-coord": {},
            "initial-zoom": {},
            "initial-color": {},
        }), 0).await;
        assert_eq!(result, Err(PackerError::InvalidConfigProperty(0, "map.layers".to_string())));

        let result = pack_map(json!({
            "layers": [],
            "coord-map": {},
            "initial-coord": {},
            "initial-zoom": {},
            "initial-color": {},
        }), 0).await;
        assert_eq!(result, Err(PackerError::InvalidConfigProperty(0, "map.coord-map".to_string())));

        let result = pack_map(json!({
            "layers": [],
            "coord-map": {
                "2d"
            },
            "initial-coord": {},
            "initial-zoom": {},
            "initial-color": {},
        }), 0).await;
        assert_eq!(result, Err(PackerError::InvalidConfigProperty(0, "map.coord-map".to_string())));
    }
}
