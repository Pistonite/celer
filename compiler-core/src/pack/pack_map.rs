//! Packs [`MapMetadata`]

use serde_json::Value;

use crate::json::{Cast, Coerce};
use crate::prop;
use crate::types::{GameCoord, MapMetadata};

use super::{ConfigTrace, PackerError, PackerResult};

macro_rules! check_map_required_property {
    ($property:expr, $trace:ident, $value:expr) => {
        match $value {
            Some(v) => Ok(v),
            None => Err(PackerError::MissingConfigProperty(
                $trace.clone(),
                format!("{}.{}", prop::MAP, $property),
            )),
        }
    };
}

/// Parses the `map` property in a config json blob
pub fn pack_map(value: Value, trace: &ConfigTrace) -> PackerResult<MapMetadata> {
    let mut map_obj = value
        .try_into_object()
        .map_err(|_| PackerError::InvalidConfigProperty(trace.clone(), prop::MAP.to_string()))?;

    let layers = check_map_required_property!(prop::LAYERS, trace, map_obj.remove(prop::LAYERS))?;
    let coord_map =
        check_map_required_property!(prop::COORD_MAP, trace, map_obj.remove(prop::COORD_MAP))?;
    let initial_coord = check_map_required_property!(
        prop::INITIAL_COORD,
        trace,
        map_obj.remove(prop::INITIAL_COORD)
    )?;
    let initial_zoom = check_map_required_property!(
        prop::INITIAL_ZOOM,
        trace,
        map_obj.remove(prop::INITIAL_ZOOM)
    )?;
    let initial_color = check_map_required_property!(
        prop::INITIAL_COLOR,
        trace,
        map_obj.remove(prop::INITIAL_COLOR)
    )?;

    if let Some(k) = map_obj.keys().next() {
        return Err(PackerError::UnusedConfigProperty(
            trace.clone(),
            format!("{}.{}", prop::MAP, k),
        ));
    }

    let layers = layers.try_into_array().map_err(|_| {
        PackerError::InvalidConfigProperty(trace.clone(), format!("{}.{}", prop::MAP, prop::LAYERS))
    })?;

    let coord_map = super::pack_coord_map(coord_map, trace)?;
    let initial_coord = match serde_json::from_value::<GameCoord>(initial_coord) {
        Ok(c) => c,
        Err(_) => {
            return Err(PackerError::InvalidConfigProperty(
                trace.clone(),
                format!("{}.{}", prop::MAP, prop::INITIAL_COORD),
            ))
        }
    };

    let initial_zoom = match initial_zoom.as_u64() {
        Some(z) => z,
        None => {
            return Err(PackerError::InvalidConfigProperty(
                trace.clone(),
                format!("{}.{}", prop::MAP, prop::INITIAL_ZOOM),
            ))
        }
    };

    let initial_color = if initial_color.is_array() || initial_color.is_object() {
        return Err(PackerError::InvalidConfigProperty(
            trace.clone(),
            format!("{}.{}", prop::MAP, prop::INITIAL_COLOR),
        ));
    } else {
        initial_color.coerce_to_string()
    };

    let layers = {
        let mut packed_layers = Vec::with_capacity(layers.len());
        for (i, layer) in layers.into_iter().enumerate() {
            packed_layers.push(super::pack_map_layer(layer, trace, i)?);
        }
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

    #[test]
    fn test_invalid_value() {
        let values = vec![
            json!(null),
            json!(false),
            json!(true),
            json!(1),
            json!([]),
            json!(""),
            json!("hello"),
        ];

        let mut trace = ConfigTrace::default();

        for (i, v) in values.into_iter().enumerate() {
            trace.push(i);
            let result = pack_map(v, &trace);
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    trace.clone(),
                    prop::MAP.to_string()
                ))
            );
            trace.pop();
        }
    }

    #[test]
    fn test_missing_properties() {
        let result = pack_map(json!({}), &Default::default());
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                Default::default(),
                "map.layers".to_string()
            ))
        );

        let result = pack_map(
            json!({
                "layers": {}
            }),
            &Default::default(),
        );
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                Default::default(),
                "map.coord-map".to_string()
            ))
        );

        let result = pack_map(
            json!({
                "layers": {},
                "coord-map": {}
            }),
            &Default::default(),
        );
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                Default::default(),
                "map.initial-coord".to_string()
            ))
        );

        let result = pack_map(
            json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {}
            }),
            &Default::default(),
        );
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                Default::default(),
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
            &Default::default(),
        );
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                Default::default(),
                "map.initial-color".to_string()
            ))
        );
    }

    #[test]
    fn test_extra_properties() {
        let result = pack_map(
            json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
                "extra": {},
            }),
            &Default::default(),
        );
        assert_eq!(
            result,
            Err(PackerError::UnusedConfigProperty(
                Default::default(),
                "map.extra".to_string()
            ))
        );
    }

    #[test]
    fn test_invalid_property_types() {
        let result = pack_map(
            json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
            }),
            &Default::default(),
        );
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                Default::default(),
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
            &Default::default(),
        );
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                Default::default(),
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
            &Default::default(),
        );
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                Default::default(),
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
            &Default::default(),
        );
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                Default::default(),
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
            &Default::default(),
        );
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                Default::default(),
                "map.initial-color".to_string()
            ))
        );
    }
}
