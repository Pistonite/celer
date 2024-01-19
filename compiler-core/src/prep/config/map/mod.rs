//! Process the `map` config property

use serde_json::Value;

use crate::json::Coerce;
use crate::macros::derive_wasm;
use crate::prep::{PrepError, PrepResult};
use crate::prop;

use super::PreparedConfig;

mod coord_map;
pub use coord_map::*;
mod layer;
pub use layer::*;

/// Metadata of the map
///
/// This includes configuration like map layers, coordinates, etc.
#[derive(PartialEq, Default, Debug, Clone)]
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
#[derive(PartialEq, Default, Debug, Clone)]
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
        let coord_map = self.parse_coord_map(coord_map)?;
        let initial_coord = serde_json::from_value::<GameCoord>(initial_coord)
            .map_err(|_| self.err_invalid_map_property_type(prop::INITIAL_COORD, "game coord"))?;
        let initial_zoom = initial_zoom.as_u64().ok_or_else(|| {
            self.err_invalid_map_property_type(prop::INITIAL_ZOOM, "non-negative integer")
        })?;
        let initial_color = if initial_color.is_array() || initial_color.is_object() {
            Err(self.err_invalid_map_property_type(prop::INITIAL_COLOR, "color string"))?
        } else {
            initial_color.coerce_to_string()
        };
        // parse layers
        let layers = {
            let mut parsed_layers = Vec::with_capacity(layers.len());
            for (i, layer) in layers.into_iter().enumerate() {
                parsed_layers.push(self.parse_map_layer(layer, i)?);
            }
            parsed_layers
        };
        let map = MapMetadata {
            layers,
            coord_map,
            initial_coord,
            initial_zoom,
            initial_color,
        };

        self.map = Some(map);

        Ok(())
    }

    fn err_invalid_map_property_type(&self, prop: &str, expected_type: &'static str) -> PrepError {
        PrepError::InvalidConfigPropertyType(
            self.trace.clone(),
            format!("{}.{}", prop::MAP, prop).into(),
            expected_type.into(),
        )
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::prep::PrepError;

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

    fn assert_missing_property(result: PrepResult<()>, p: &'static str) {
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
