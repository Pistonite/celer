//! Process the `map` config property

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::macros::derive_wasm;
use crate::prep::PrepResult;
use crate::prop;

use super::PreparedConfig;

mod coord_map;
pub use coord_map::*;
mod layer;
pub use layer::*;

/// Metadata of the map
///
/// This includes configuration like map layers, coordinates, etc.
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct MapMetadata {
    /// The map layers. First is the lowest layer.
    pub layers: Vec<MapLayer>,
    /// Mapping for the coordinates in the route.
    pub coord_map: MapCoordMap,
    /// Initial coordinates
    pub initial_coord: GameCoord,
    /// Initial zoom level
    pub initial_zoom: u64,
    /// Initial map line color
    pub initial_color: String,
}

/// Coordinates representing a point (x, y, z) in the game
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct GameCoord(pub f64, pub f64, pub f64);

macro_rules! check_map_required_property {
    ($self:ident, $map_config:ident, $prop:expr) => {{
        let prop = $prop;
        super::check_required_property!(
            $self,
            $map_config.remove(prop),
            format!("{}.{}", prop::MAP, prop)
        )
    }};
}

impl<'a> PreparedConfig<'a> {
    /// Load the map config into `self.map`
    ///
    /// This does not check for existing map configs, and replaces the existing map config
    pub async fn load_map(&mut self, map_config: Value) -> PrepResult<()> {
        // ensure config is object
        let mut map_config = super::check_map!(self, map_config, prop::MAP)?;
        // extract properties
        let layers = check_map_required_property!(self, map_config, prop::LAYERS)?;
        let coord_map = check_map_required_property!(self, map_config, prop::COORD_MAP)?;
        let initial_coord = check_map_required_property!(self, map_config, prop::INITIAL_COORD)?;
        let initial_zoom = check_map_required_property!(self, map_config, prop::INITIAL_ZOOM)?;
        let initial_color = check_map_required_property!(self, map_config, prop::INITIAL_COLOR)?;
        // disallow additional properties
        self.check_unused_property(
            map_config
                .keys()
                .next()
                .map(|k| format!("{}.{}", prop::MAP, k)),
        )?;

        // type checking
        let layers = super::check_array!(self, layers, format!("{}.{}", prop::MAP, prop::LAYERS))?;
        //     let coord_map = super::pack_coord_map(coord_map, trace)?;
        // let initial_coord = match serde_json::from_value::<GameCoord>(initial_coord) {
        //     Ok(c) => c,
        //     Err(_) => {
        //         return Err(PackerError::InvalidConfigProperty(
        //             trace.clone(),
        //             format!("{}.{}", prop::MAP, prop::INITIAL_COORD),
        //         ))
        //     }
        // };
        //
        // let initial_zoom = match initial_zoom.as_u64() {
        //     Some(z) => z,
        //     None => {
        //         return Err(PackerError::InvalidConfigProperty(
        //             trace.clone(),
        //             format!("{}.{}", prop::MAP, prop::INITIAL_ZOOM),
        //         ))
        //     }
        // };
        //
        // let initial_color = if initial_color.is_array() || initial_color.is_object() {
        //     return Err(PackerError::InvalidConfigProperty(
        //         trace.clone(),
        //         format!("{}.{}", prop::MAP, prop::INITIAL_COLOR),
        //     ));
        // } else {
        //     initial_color.coerce_to_string()
        // };
        //
        //     // parse layers
        // let layers = {
        //     let mut packed_layers = Vec::with_capacity(layers.len());
        //     for (i, layer) in layers.into_iter().enumerate() {
        //         packed_layers.push(super::config.load_map_layer(layer, trace, i)?);
        //     }
        //     packed_layers
        // };
        // Ok(MapMetadata {
        //     layers,
        //     coord_map,
        //     initial_coord,
        //     initial_zoom,
        //     initial_color,
        // });

        // todo: set self.map
        todo!()
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::prep::PrepError;
    use crate::res::test_utils::StubLoader;

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

        let mut config = PreparedConfig::default();

        for v in values.into_iter() {
            let result = config.load_map(v).await;
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    prop::MAP.into(),
                    "mapping object".into()
                ))
            );
        }
    }

    fn assert_missing_property(result: PrepResult<()>, p: &str) {
        assert_eq!(
            result,
            Err(PrepError::MissingConfigProperty(
                Default::default(),
                p.into()
            ))
        );
    }

    #[tokio::test]
    async fn test_missing_properties() {
        let mut config = PreparedConfig::default();

        let result = config.load_map(json!({})).await;
        assert_missing_property(result, "map.layers");

        let result = config
            .load_map(json!({
                "layers": {}
            }))
            .await;
        assert_missing_property(result, "map.coord-map");

        let result = config
            .load_map(json!({
                "layers": {},
                "coord-map": {}
            }))
            .await;
        assert_missing_property(result, "map.initial-coord");

        let result = config
            .load_map(json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {}
            }))
            .await;
        assert_missing_property(result, "map.initial-zoom");

        let result = config
            .load_map(json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {},
                "initial-zoom": {},
            }))
            .await;
        assert_missing_property(result, "map.initial-color");
    }

    #[tokio::test]
    async fn test_extra_properties() {
        let mut config = PreparedConfig::default();
        let result = config
            .load_map(json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
                "extra": {},
            }))
            .await;
        assert_eq!(
            result,
            Err(PrepError::UnusedConfigProperty(
                Default::default(),
                "map.extra".into()
            ))
        );
    }

    #[tokio::test]
    async fn test_invalid_property_types() {
        let mut config = PreparedConfig::default();
        let result = config
            .load_map(json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
            }))
            .await;
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.layers".into(),
                "array".into()
            ))
        );

        let result = config
            .load_map(json!({
                "layers": [],
                "coord-map": [],
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
            }))
            .await;
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.coord-map".into(),
                "mapping object".into()
            ))
        );

        let result = config
            .load_map(json!({
                "layers": [],
                "coord-map": {
                    "2d": ["x", "x"],
                    "3d": ["x", "x", "x"]
                },
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
            }))
            .await;
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.initial-coord".into(),
                "game coord".into()
            ))
        );

        let result = config
            .load_map(json!({
                "layers": [],
                "coord-map": {
                    "2d": ["x", "x"],
                    "3d": ["x", "x", "x"]
                },
                "initial-coord": [0, 0, 0],
                "initial-zoom": {},
                "initial-color": {},
            }))
            .await;
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.initial-zoom".into(),
                "non-negative integer".into()
            ))
        );

        let result = config
            .load_map(json!({
                "layers": [],
                "coord-map": {
                    "2d": ["x", "x"],
                    "3d": ["x", "x", "x"]
                },
                "initial-coord": [0, 0, 0],
                "initial-zoom": 0,
                "initial-color": {},
            }))
            .await;
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.initial-color".into(),
                "color string".into()
            ))
        );
    }
}
