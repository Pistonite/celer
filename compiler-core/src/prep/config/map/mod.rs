//! Process the `map` config property

use serde_json::Value;

use crate::prop;
use crate::res::Loader;
use crate::prep::PrepResult;

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


impl<L> PreparedConfig<L> where L: Loader {
    /// Load the map config into `self.map`
    /// 
    /// This does not check for existing map configs, and replaces the existing map config
    pub async fn load_map(&mut self, map_config: Value) -> PrepResult<()> {
        // ensure config is object
        let mut map_config = self.check_map(map_config, prop::MAP)?;
        // extract properties
        let layers = self.check_required_property(map_config.remove(prop::LAYERS), prop::LAYERS)?;
        let coord_map = self.check_required_property(map_config.remove(prop::COORD_MAP), prop::COORD_MAP)?;
        let initial_coord = self.check_required_property(map_config.remove(prop::INITIAL_COORD), prop::INITIAL_COORD)?;
        let initial_zoom = self.check_required_property(map_config.remove(prop::INITIAL_ZOOM), prop::INITIAL_ZOOM)?;
        let initial_color = self.check_required_property(map_config.remove(prop::INITIAL_COLOR), prop::INITIAL_COLOR)?;
        // disallow additional properties
        self.check_unused_property(map_config.keys().next().map(|k|{
            format!("{}.{}", prop::MAP, k)
        }))?;

        // type checking
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

        // parse layers
    let layers = {
        let mut packed_layers = Vec::with_capacity(layers.len());
        for (i, layer) in layers.into_iter().enumerate() {
            packed_layers.push(super::config.load_map_layer(layer, trace, i)?);
        }
        packed_layers
    };
    Ok(MapMetadata {
        layers,
        coord_map,
        initial_coord,
        initial_zoom,
        initial_color,
    });

            // todo: set self.map
            todo!()
    }
}


#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::prep::{PrepError, ConfigTrace};

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

        for (i, v) in values.into_iter().enumerate() {
            let result = config.load_map(v).await;
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigProperty(
                    ConfigTrace::default(),
                    prop::MAP
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

        let result = config.load_map(
            json!({
                "layers": {}
            })
        ).await;
        assert_missing_property(result, "map.coord-map");

        let result = config.load_map(
            json!({
                "layers": {},
                "coord-map": {}
            }),
        ).await;
        assert_missing_property(result, "map.initial-coord");

        let result = config.load_map(
            json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {}
            }),
        ).await;
        assert_missing_property(result, "map.initial-zoom");

        let result = config.load_map(
            json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {},
                "initial-zoom": {},
            }),
        ).await;
        assert_missing_property(result, "map.initial-color");
    }

    #[tokio::test]
    async fn test_extra_properties() {
        let mut config = PreparedConfig::default();
        let result = config.load_map(
            json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
                "extra": {},
            }),
        ).await;
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
        let result = config.load_map(
            json!({
                "layers": {},
                "coord-map": {},
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
            }),
        );
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.layers".into(),
                "array"
            ))
        );

        let result = config.load_map(
            json!({
                "layers": [],
                "coord-map": [],
                "initial-coord": {},
                "initial-zoom": {},
                "initial-color": {},
            }),
        );
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.coord-map".into(),
                "mapping object"
            ))
        );

        let result = config.load_map(
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
        );
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.initial-coord".into(),
                "game coord"
            ))
        );

        let result = config.load_map(
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
        );
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.initial-zoom",
                "non-negative integer"
            ))
        );

        let result = config.load_map(
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
        );
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.initial-color".into(),
                "color string"
            ))
        );
    }
}
