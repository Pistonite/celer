//! Packs json blob into [`MapLayerAttr`]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::json::{Cast, Coerce};
use crate::macros::derive_wasm;
use crate::prep::{config, PrepError, PrepResult, PreparedConfig};
use crate::prop;

// use super::{ConfigTrace, PrepError, PackerResult};

/// Attribute (definition) of a map layer in the route
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct MapLayer {
    /// Display name of the layer
    ///
    /// This is visible in the layer switch UI
    pub name: String,

    /// The tileset url template, with {x} {y} {z} as placeholders.
    ///
    /// The url should conform to the leaflet tile layer API:
    /// https://leafletjs.com/reference.html#tilelayer
    pub template_url: String,

    /// The raster coordinate size
    ///
    /// See: https://github.com/commenthol/leaflet-rastercoords.
    /// Form is [width, height]
    pub size: (u64, u64),

    /// Min and max zoom levels
    pub zoom_bounds: (u64, u64),

    /// Max native zoom of the tileset
    pub max_native_zoom: u64,

    /// Coordinate transformation
    ///
    /// This should transform (x, y) from the game's coordinate space to (x, y) in the raster image.
    pub transform: MapTilesetTransform,

    /// The minimum Z value this layer should be used
    ///
    /// This value is ignored for the first (lowest) layer
    pub start_z: f64,

    /// Attribution
    pub attribution: MapAttribution,
}

/// The tileset transform
///
/// The transformed coordiante will be:
/// ```no-compile
/// (x, y) -> (x * scale[0] + translate[0], y * scale[1] + translate[1])
/// ```
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct MapTilesetTransform {
    /// The scale of the transformation
    pub scale: (f64, f64),
    /// The translation of the transformation
    pub translate: (f64, f64),
}

/// Attribution to display on the map
///
/// (displayed as &copy; LINK)
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct MapAttribution {
    /// Url of the attribution
    pub link: String,
    /// If the copyright sign should be displayed
    #[serde(default)]
    pub copyright: bool,
}

macro_rules! check_layer_required_property {
    ($self:ident, $property:expr, $layer_index:ident, $obj:ident) => {{
        let property = $property;
        config::check_required_property!(
            $self,
            $obj.remove(property),
            format!("map.layers[{}].{}", $layer_index, property)
        )
    }};
}

impl<'a> PreparedConfig<'a> {
    pub fn parse_map_layer(&self, value: Value, layer_index: usize) -> PrepResult<MapLayer> {
        let mut obj = config::check_map!(self, value, format!("map.layers[{layer_index}]"))?;

        let name = check_layer_required_property!(self, prop::NAME, layer_index, obj)?;
        let template_url =
            check_layer_required_property!(self, prop::TEMPLATE_URL, layer_index, obj)?;
        let size = check_layer_required_property!(self, prop::SIZE, layer_index, obj)?;
        let zoom_bounds =
            check_layer_required_property!(self, prop::ZOOM_BOUNDS, layer_index, obj)?;
        let max_native_zoom =
            check_layer_required_property!(self, prop::MAX_NATIVE_ZOOM, layer_index, obj)?;
        let transform = check_layer_required_property!(self, prop::TRANSFORM, layer_index, obj)?;
        let start_z = check_layer_required_property!(self, prop::START_Z, layer_index, obj)?;
        let attribution =
            check_layer_required_property!(self, prop::ATTRIBUTION, layer_index, obj)?;

        self.check_unused_property(
            obj.keys()
                .next()
                .map(|k| format!("{}.{}[{}].{}", prop::MAP, prop::LAYERS, layer_index, k)),
        )?;

        let name = if name.is_array() || name.is_object() {
            return Err(PrepError::InvalidConfigPropertyType(
                self.trace.clone(),
                format!(
                    "{}.{}[{}].{}",
                    prop::MAP,
                    prop::LAYERS,
                    layer_index,
                    prop::NAME
                )
                .into(),
                "string".into(),
            ));
        } else {
            name.coerce_to_string()
        };
        // let template_url = if template_url.is_array() || template_url.is_object() {
        //     return Err(PrepError::InvalidConfigProperty(
        //         trace.clone(),
        //         format!(
        //             "{}.{}[{}].{}",
        //             prop::MAP,
        //             prop::LAYERS,
        //             layer_index,
        //             prop::TEMPLATE_URL
        //         ),
        //     ));
        // } else {
        //     template_url.coerce_to_string()
        // };
        //
        // let size = parse_array_with_2_elements(size).ok_or_else(|| {
        //     PrepError::InvalidConfigProperty(
        //         trace.clone(),
        //         format!(
        //             "{}.{}[{}].{}",
        //             prop::MAP,
        //             prop::LAYERS,
        //             layer_index,
        //             prop::SIZE
        //         ),
        //     )
        // })?;
        //
        // let zoom_bounds = parse_array_with_2_elements(zoom_bounds).ok_or_else(|| {
        //     PrepError::InvalidConfigProperty(
        //         trace.clone(),
        //         format!(
        //             "{}.{}[{}].{}",
        //             prop::MAP,
        //             prop::LAYERS,
        //             layer_index,
        //             prop::ZOOM_BOUNDS
        //         ),
        //     )
        // })?;
        //
        // let max_native_zoom = max_native_zoom.try_coerce_to_u64().ok_or_else(|| {
        //     PrepError::InvalidConfigProperty(
        //         trace.clone(),
        //         format!(
        //             "{}.{}[{}].{}",
        //             prop::MAP,
        //             prop::LAYERS,
        //             layer_index,
        //             prop::MAX_NATIVE_ZOOM
        //         ),
        //     )
        // })?;
        //
        // let transform = serde_json::from_value::<MapTilesetTransform>(transform).map_err(|_| {
        //     PrepError::InvalidConfigProperty(
        //         trace.clone(),
        //         format!(
        //             "{}.{}[{}].{}",
        //             prop::MAP,
        //             prop::LAYERS,
        //             layer_index,
        //             prop::TRANSFORM
        //         ),
        //     )
        // })?;
        //
        // let start_z = start_z.try_coerce_to_f64().ok_or_else(|| {
        //     PrepError::InvalidConfigProperty(
        //         trace.clone(),
        //         format!(
        //             "{}.{}[{}].{}",
        //             prop::MAP,
        //             prop::LAYERS,
        //             layer_index,
        //             prop::START_Z
        //         ),
        //     )
        // })?;
        //
        // let attribution = serde_json::from_value::<MapAttribution>(attribution).map_err(|_| {
        //     PrepError::InvalidConfigProperty(
        //         trace.clone(),
        //         format!(
        //             "{}.{}[{}].{}",
        //             prop::MAP,
        //             prop::LAYERS,
        //             layer_index,
        //             prop::ATTRIBUTION
        //         ),
        //     )
        // })?;

        // Ok(MapLayerAttr {
        //     name,
        //     template_url,
        //     size,
        //     zoom_bounds,
        //     max_native_zoom,
        //     transform,
        //     start_z,
        //     attribution,
        // })

        todo!()
    }
}

// pub fn parse_map_layer(
//     value: Value,
//     layer_index: usize,
// ) -> PackerResult<MapLayerAttr> {
//
// }

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

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::json;

    use crate::res::test_utils::StubLoader;

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

        let config = PreparedConfig::default();

        for (i, v) in values.into_iter().enumerate() {
            let result = config.parse_map_layer(v, i);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    format!("map.layers[{i}]").into(),
                    "mapping object".into(),
                ))
            );
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

        let config = PreparedConfig::default();

        let mut value = json!({});
        for p in props {
            let result = config.parse_map_layer(value.clone(), 0);
            assert_eq!(
                result,
                Err(PrepError::MissingConfigProperty(
                    Default::default(),
                    format!("map.layers[0].{p}").into()
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

        let config = PreparedConfig::default();

        let result = config.parse_map_layer(value, 0);
        assert_eq!(
            result,
            Err(PrepError::UnusedConfigProperty(
                Default::default(),
                "map.layers[0].extra".into()
            ))
        );
    }

    #[test]
    fn test_invalid_name() {
        let config = PreparedConfig::default();
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
            let result = config.parse_map_layer(value, 0);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.layers[0].name".into(),
                    "string".into(),
                ))
            );
        }
    }

    #[test]
    fn test_invalid_template_url() {
        let config = PreparedConfig::default();
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
            let result = config.parse_map_layer(value, 0);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.layers[0].template-url".into(),
                    "string".into(),
                ))
            );
        }
    }

    #[test]
    fn test_invalid_size() {
        let config = PreparedConfig::default();
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
            let result = config.parse_map_layer(value, 0);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.layers[0].size".into(),
                    "array of 2 non-negative integers".into(),
                ))
            );
        }
    }

    #[test]
    fn test_invalid_zoom_bounds() {
        let config = PreparedConfig::default();
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
            let result = config.parse_map_layer(value, 0);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.layers[0].zoom-bounds".into(),
                    "array of 2 non-negative integers".into(),
                ))
            );
        }
    }

    #[test]
    fn test_invalid_max_native_zoom() {
        let config = PreparedConfig::default();
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
            let result = config.parse_map_layer(value, 0);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.layers[0].max-native-zoom".into(),
                    "non-negative integer".into(),
                ))
            );
        }
    }

    #[test]
    fn test_invalid_transform() {
        let config = PreparedConfig::default();
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
            let result = config.parse_map_layer(value, 0);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.layers[0].transform".into(),
                    "transform object".into(),
                ))
            );
        }
    }

    #[test]
    fn test_invalid_start_z() {
        let config = PreparedConfig::default();
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
            let result = config.parse_map_layer(value, 0);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.layers[0].start-z".into(),
                    "number".into(),
                ))
            );
        }
    }

    #[test]
    fn test_invalid_attribution() {
        let config = PreparedConfig::default();
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
            let result = config.parse_map_layer(value, 0);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.layers[0].attribution".into(),
                    "attribution object".into(),
                ))
            );
        }
    }

    #[test]
    fn test_ok() {
        let config = PreparedConfig::default();
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

        let result = config.parse_map_layer(value, 0);

        assert_eq!(
            result,
            Ok(MapLayer {
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
