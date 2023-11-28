//! Packs json blob into [`MapLayerAttr`]

use serde_json::Value;

use crate::json::{Cast, Coerce};
use crate::macros::test_suite;
use crate::prop;
use crate::types::{MapAttribution, MapLayerAttr, MapTilesetTransform};

use super::{ConfigTrace, PackerError, PackerResult};

macro_rules! check_layer_required_property {
    ($property:expr, $trace:ident, $layer_index:ident, $obj:ident) => {
        match $obj.remove($property) {
            Some(v) => Ok(v),
            None => Err(PackerError::MissingConfigProperty(
                $trace.clone(),
                format!(
                    "{}.{}[{}].{}",
                    prop::MAP,
                    prop::LAYERS,
                    $layer_index,
                    $property
                ),
            )),
        }
    };
}

pub fn pack_map_layer(
    value: Value,
    trace: &ConfigTrace,
    layer_index: usize,
) -> PackerResult<MapLayerAttr> {
    let mut obj = value.try_into_object().map_err(|_| {
        PackerError::InvalidConfigProperty(trace.clone(), format!("map.layers[{layer_index}]"))
    })?;

    let name = check_layer_required_property!(prop::NAME, trace, layer_index, obj)?;
    let template_url = check_layer_required_property!(prop::TEMPLATE_URL, trace, layer_index, obj)?;
    let size = check_layer_required_property!(prop::SIZE, trace, layer_index, obj)?;
    let zoom_bounds = check_layer_required_property!(prop::ZOOM_BOUNDS, trace, layer_index, obj)?;
    let max_native_zoom =
        check_layer_required_property!(prop::MAX_NATIVE_ZOOM, trace, layer_index, obj)?;
    let transform = check_layer_required_property!(prop::TRANSFORM, trace, layer_index, obj)?;
    let start_z = check_layer_required_property!(prop::START_Z, trace, layer_index, obj)?;
    let attribution = check_layer_required_property!(prop::ATTRIBUTION, trace, layer_index, obj)?;

    if let Some(k) = obj.keys().next() {
        return Err(PackerError::UnusedConfigProperty(
            trace.clone(),
            format!("{}.{}[{}].{}", prop::MAP, prop::LAYERS, layer_index, k),
        ));
    }

    let name = if name.is_array() || name.is_object() {
        return Err(PackerError::InvalidConfigProperty(
            trace.clone(),
            format!(
                "{}.{}[{}].{}",
                prop::MAP,
                prop::LAYERS,
                layer_index,
                prop::NAME
            ),
        ));
    } else {
        name.coerce_to_string()
    };

    let template_url = if template_url.is_array() || template_url.is_object() {
        return Err(PackerError::InvalidConfigProperty(
            trace.clone(),
            format!(
                "{}.{}[{}].{}",
                prop::MAP,
                prop::LAYERS,
                layer_index,
                prop::TEMPLATE_URL
            ),
        ));
    } else {
        template_url.coerce_to_string()
    };

    let size = parse_array_with_2_elements(size).ok_or_else(|| {
        PackerError::InvalidConfigProperty(
            trace.clone(),
            format!(
                "{}.{}[{}].{}",
                prop::MAP,
                prop::LAYERS,
                layer_index,
                prop::SIZE
            ),
        )
    })?;

    let zoom_bounds = parse_array_with_2_elements(zoom_bounds).ok_or_else(|| {
        PackerError::InvalidConfigProperty(
            trace.clone(),
            format!(
                "{}.{}[{}].{}",
                prop::MAP,
                prop::LAYERS,
                layer_index,
                prop::ZOOM_BOUNDS
            ),
        )
    })?;

    let max_native_zoom = max_native_zoom.try_coerce_to_u64().ok_or_else(|| {
        PackerError::InvalidConfigProperty(
            trace.clone(),
            format!(
                "{}.{}[{}].{}",
                prop::MAP,
                prop::LAYERS,
                layer_index,
                prop::MAX_NATIVE_ZOOM
            ),
        )
    })?;

    let transform = serde_json::from_value::<MapTilesetTransform>(transform).map_err(|_| {
        PackerError::InvalidConfigProperty(
            trace.clone(),
            format!(
                "{}.{}[{}].{}",
                prop::MAP,
                prop::LAYERS,
                layer_index,
                prop::TRANSFORM
            ),
        )
    })?;

    let start_z = start_z.try_coerce_to_f64().ok_or_else(|| {
        PackerError::InvalidConfigProperty(
            trace.clone(),
            format!(
                "{}.{}[{}].{}",
                prop::MAP,
                prop::LAYERS,
                layer_index,
                prop::START_Z
            ),
        )
    })?;

    let attribution = serde_json::from_value::<MapAttribution>(attribution).map_err(|_| {
        PackerError::InvalidConfigProperty(
            trace.clone(),
            format!(
                "{}.{}[{}].{}",
                prop::MAP,
                prop::LAYERS,
                layer_index,
                prop::ATTRIBUTION
            ),
        )
    })?;

    Ok(MapLayerAttr {
        name,
        template_url,
        size,
        zoom_bounds,
        max_native_zoom,
        transform,
        start_z,
        attribution,
    })
}

#[inline]
fn parse_array_with_2_elements(value: Value) -> Option<(u64, u64)> {
    let arr = value.try_into_array().ok()?;
    if arr.len() != 2 {
        return None;
    }
    let x = arr[0].try_coerce_to_u64()?;
    let y = arr[1].try_coerce_to_u64()?;
    Some((x, y))
}

#[test_suite]
mod test {
    use super::*;

    use serde_json::json;

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
            let result = pack_map_layer(v, &trace, i);
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    trace.clone(),
                    format!("map.layers[{i}]")
                ))
            );
            trace.pop();
        }
    }

    #[test]
    fn test_missing_properties() {
        let props = vec![
            prop::NAME,
            prop::TEMPLATE_URL,
            prop::SIZE,
            prop::ZOOM_BOUNDS,
            prop::MAX_NATIVE_ZOOM,
            prop::TRANSFORM,
            prop::START_Z,
            prop::ATTRIBUTION,
        ];

        let mut value = json!({});
        for p in props {
            let result = pack_map_layer(value.clone(), &Default::default(), 0);
            assert_eq!(
                result,
                Err(PackerError::MissingConfigProperty(
                    Default::default(),
                    format!("map.layers[0].{p}")
                ))
            );

            value[p] = json!("test");
        }
    }

    #[test]
    fn test_extra_property() {
        let value = json!({
            "name": "",
            "template-url": "",
            "size": "",
            "zoom-bounds": "",
            "max-native-zoom": "",
            "transform": "",
            "start-z": "",
            "attribution": "",
            "extra": "",
        });

        let result = pack_map_layer(value, &Default::default(), 0);
        assert_eq!(
            result,
            Err(PackerError::UnusedConfigProperty(
                Default::default(),
                "map.layers[0].extra".to_string()
            ))
        );
    }

    #[test]
    fn test_invalid_name() {
        let values = vec![json!([]), json!({})];
        for v in values {
            let value = json!({
                "name": v,
                "template-url": "",
                "size": "",
                "zoom-bounds": "",
                "max-native-zoom": "",
                "transform": "",
                "start-z": "",
                "attribution": "",
            });
            let result = pack_map_layer(value, &Default::default(), 0);
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.layers[0].name".to_string()
                ))
            );
        }
    }

    #[test]
    fn test_invalid_template_url() {
        let values = vec![json!([]), json!({})];
        for v in values {
            let value = json!({
                "name": "",
                "template-url": v,
                "size": "",
                "zoom-bounds": "",
                "max-native-zoom": "",
                "transform": "",
                "start-z": "",
                "attribution": "",
            });
            let result = pack_map_layer(value, &Default::default(), 0);
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.layers[0].template-url".to_string()
                ))
            );
        }
    }

    #[test]
    fn test_invalid_size() {
        let values = vec![
            json!([]),
            json!({}),
            json!(true),
            json!(false),
            json!(1),
            json!(null),
            json!([1]),
            json!([1.5, 2]),
        ];
        for v in values {
            let value = json!({
                "name": "",
                "template-url": "",
                "size": v,
                "zoom-bounds": "",
                "max-native-zoom": "",
                "transform": "",
                "start-z": "",
                "attribution": "",
            });
            let result = pack_map_layer(value, &Default::default(), 0);
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.layers[0].size".to_string()
                ))
            );
        }
    }

    #[test]
    fn test_invalid_zoom_bounds() {
        let values = vec![
            json!([]),
            json!({}),
            json!(true),
            json!(false),
            json!(1),
            json!(null),
            json!([1]),
            json!([1.5, 2]),
        ];
        for v in values {
            let value = json!({
                "name": "",
                "template-url": "",
                "size": [1, 2],
                "zoom-bounds": v,
                "max-native-zoom": "",
                "transform": "",
                "start-z": "",
                "attribution": "",
            });
            let result = pack_map_layer(value, &Default::default(), 0);
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.layers[0].zoom-bounds".to_string()
                ))
            );
        }
    }

    #[test]
    fn test_invalid_max_native_zoom() {
        let values = vec![
            json!([]),
            json!({}),
            json!(true),
            json!(false),
            json!(""),
            json!("hello"),
            json!(null),
            json!([1]),
            json!(-1),
            json!([1.5, 2]),
        ];
        for v in values {
            let value = json!({
                "name": "",
                "template-url": "",
                "size": [1, 2],
                "zoom-bounds": [3, 4],
                "max-native-zoom": v,
                "transform": "",
                "start-z": "",
                "attribution": "",
            });
            let result = pack_map_layer(value, &Default::default(), 0);
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.layers[0].max-native-zoom".to_string()
                ))
            );
        }
    }

    #[test]
    fn test_invalid_transform() {
        let values = vec![
            json!({}),
            json!(true),
            json!(null),
            json!({
                "scale": []
            }),
            json!({
                "translate": []
            }),
            json!({
                "scale": [1],
                "translate": [2]
            }),
        ];
        for v in values {
            let value = json!({
                "name": "",
                "template-url": "",
                "size": [1, 2],
                "zoom-bounds": [3, 4],
                "max-native-zoom": 3,
                "transform": v,
                "start-z": "",
                "attribution": "",
            });
            let result = pack_map_layer(value, &Default::default(), 0);
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.layers[0].transform".to_string()
                ))
            );
        }
    }

    #[test]
    fn test_invalid_start_z() {
        let values = vec![json!({}), json!([]), json!(true), json!(null)];
        for v in values {
            let value = json!({
                "name": "",
                "template-url": "",
                "size": [1, 2],
                "zoom-bounds": [3, 4],
                "max-native-zoom": 3,
                "transform": {
                "scale": [1, 2],
                "translate": [1, 2],
                },
                "start-z": v,
                "attribution": "",
            });
            let result = pack_map_layer(value, &Default::default(), 0);
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.layers[0].start-z".to_string()
                ))
            );
        }
    }

    #[test]
    fn test_invalid_attribution() {
        let values = vec![
            json!({}),
            json!([]),
            json!(true),
            json!(null),
            json!({
                "link": []
            }),
            json!({
                "copyright": []
            }),
            json!({
                "copyright": true
            }),
            json!({
                "link": "",
                "copyright": []
            }),
        ];
        for v in values {
            let value = json!({
                "name": "",
                "template-url": "",
                "size": [1, 2],
                "zoom-bounds": [3, 4],
                "max-native-zoom": 3,
                "transform": {
                "scale": [1, 2],
                "translate": [1, 2],
                },
                "start-z": 0,
                "attribution": v,
            });
            let result = pack_map_layer(value, &Default::default(), 0);
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.layers[0].attribution".to_string()
                ))
            );
        }
    }

    #[test]
    fn test_ok() {
        let value = json!({
            "name": "Test Name",
            "template-url": "Test URL",
            "size": [1, 2],
            "zoom-bounds": [3, 4],
            "max-native-zoom": 3,
            "transform": {
                "scale": [1, 2],
                "translate": [-1.05, -2],
            },
            "start-z": -123.45,
            "attribution": {
                "link": "Test Link"
            },
        });

        let result = pack_map_layer(value, &Default::default(), 0);

        assert_eq!(
            result,
            Ok(MapLayerAttr {
                name: "Test Name".to_string(),
                template_url: "Test URL".to_string(),
                size: (1, 2),
                zoom_bounds: (3, 4),
                max_native_zoom: 3,
                transform: MapTilesetTransform {
                    scale: (1.0, 2.0),
                    translate: (-1.05, -2.0),
                },
                start_z: -123.45,
                attribution: MapAttribution {
                    link: "Test Link".to_string(),
                    copyright: false
                },
            })
        );
    }
}
