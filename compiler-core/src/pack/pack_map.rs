//! Packs [`MapMetadata`]

use celerctypes::{GameCoord, MapMetadata};
use serde_json::Value;

use super::{pack_coord_map, pack_map_layer, PackerError, PackerResult};

use crate::comp::prop;
use crate::json::{Cast, Coerce};
use crate::util::async_for;

macro_rules! check_map_required_property {
    ($property:expr, $index:ident, $value:expr) => {
        match $value {
            Some(v) => Ok(v),
            None => Err(PackerError::MissingConfigProperty(
                $index,
                format!("{}.{}", prop::MAP, $property),
            )),
        }
    };
}

/// Parses the `map` property in a config json blob
pub async fn pack_map(value: Value, index: usize) -> PackerResult<MapMetadata> {
    let mut map_obj = value
        .try_into_object()
        .map_err(|_| PackerError::InvalidConfigProperty(index, prop::MAP.to_string()))?;

    let layers = check_map_required_property!(prop::LAYERS, index, map_obj.remove(prop::LAYERS))?;
    let coord_map =
        check_map_required_property!(prop::COORD_MAP, index, map_obj.remove(prop::COORD_MAP))?;
    let initial_coord = check_map_required_property!(
        prop::INITIAL_COORD,
        index,
        map_obj.remove(prop::INITIAL_COORD)
    )?;
    let initial_zoom = check_map_required_property!(
        prop::INITIAL_ZOOM,
        index,
        map_obj.remove(prop::INITIAL_ZOOM)
    )?;
    let initial_color = check_map_required_property!(
        prop::INITIAL_COLOR,
        index,
        map_obj.remove(prop::INITIAL_COLOR)
    )?;

    if let Some(k) = map_obj.keys().next() {
        return Err(PackerError::UnusedConfigProperty(
            index,
            format!("{}.{}", prop::MAP, k),
        ));
    }

    let layers = layers.try_into_array().map_err(|_| {
        PackerError::InvalidConfigProperty(index, format!("{}.{}", prop::MAP, prop::LAYERS))
    })?;

    let coord_map = pack_coord_map(coord_map, index).await?;
    let initial_coord = match serde_json::from_value::<GameCoord>(initial_coord) {
        Ok(c) => c,
        Err(_) => {
            return Err(PackerError::InvalidConfigProperty(
                index,
                format!("{}.{}", prop::MAP, prop::INITIAL_COORD),
            ))
        }
    };

    let initial_zoom = match initial_zoom.as_u64() {
        Some(z) => z,
        None => {
            return Err(PackerError::InvalidConfigProperty(
                index,
                format!("{}.{}", prop::MAP, prop::INITIAL_ZOOM),
            ))
        }
    };

    let initial_color = if initial_color.is_array() || initial_color.is_object() {
        return Err(PackerError::InvalidConfigProperty(
            index,
            format!("{}.{}", prop::MAP, prop::INITIAL_COLOR),
        ));
    } else {
        initial_color.coerce_to_string()
    };

    let layers = {
        let mut packed_layers = Vec::with_capacity(layers.len());
        async_for!((i, layer) in layers.into_iter().enumerate(), {
            packed_layers.push(pack_map_layer(layer, index, i)?);
        });
        packed_layers
    };

    Ok(MapMetadata {
        layers,
        coord_map,
        initial_coord,
        initial_zoom,
        initial_color,
    })
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
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(i, prop::MAP.to_string()))
            );
        }
    }

    #[tokio::test]
    async fn test_missing_properties() {
        let result = pack_map(json!({}), 0).await;
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                0,
                "map.layers".to_string()
            ))
        );

        let result = pack_map(
            json!({
                "layers": {}
            }),
            0,
        )
        .await;
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                0,
                "map.coord-map".to_string()
            ))
        );

        let result = pack_map(
            json!({
                "layers": {},
                "coord-map": {}
            }),
            0,
        )
        .await;
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                0,
                "map.initial-coord".to_string()
            ))
        );

        let result = pack_map(
            json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {}
            }),
            0,
        )
        .await;
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                0,
                "map.initial-zoom".to_string()
            ))
        );

        let result = pack_map(
            json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {},
                "initial-zoom": {},
            }),
            0,
        )
        .await;
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                0,
                "map.initial-color".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn test_extra_properties() {
        let result = pack_map(
            json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
                "extra": {},
            }),
            0,
        )
        .await;
        assert_eq!(
            result,
            Err(PackerError::UnusedConfigProperty(
                0,
                "map.extra".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn test_invalid_property_types() {
        let result = pack_map(
            json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
            }),
            0,
        )
        .await;
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                0,
                "map.layers".to_string()
            ))
        );

        let result = pack_map(
            json!({
                "layers": [],
                "coord-map": [],
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
            }),
            0,
        )
        .await;
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                0,
                "map.coord-map".to_string()
            ))
        );

        let result = pack_map(
            json!({
                "layers": [],
                "coord-map": {
                    "2d": ["x", "x"],
                    "3d": ["x", "x", "x"]
                },
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
            }),
            0,
        )
        .await;
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                0,
                "map.initial-coord".to_string()
            ))
        );

        let result = pack_map(
            json!({
                "layers": [],
                "coord-map": {
                    "2d": ["x", "x"],
                    "3d": ["x", "x", "x"]
                },
                "initial-coord": [0, 0, 0],
                "initial-zoom": {},
                "initial-color": {},
            }),
            0,
        )
        .await;
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                0,
                "map.initial-zoom".to_string()
            ))
        );

        let result = pack_map(
            json!({
                "layers": [],
                "coord-map": {
                    "2d": ["x", "x"],
                    "3d": ["x", "x", "x"]
                },
                "initial-coord": [0, 0, 0],
                "initial-zoom": 0,
                "initial-color": {},
            }),
            0,
        )
        .await;
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                0,
                "map.initial-color".to_string()
            ))
        );
    }
}
