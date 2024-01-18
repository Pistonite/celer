//! Packs map.coord-map into [`MapCoordMap`]
use serde_json::Value;

use crate::macros::derive_wasm;
use crate::prep::{config, PrepError, PrepResult, PreparedConfig};
use crate::prop;

/// The mapping if 2 coordinates are specified in the route
///
/// For example, ["x", "z"] will map the coordinates
/// to the x (horizontal) and z (height) axis of the map.
///
/// Default value of 0 will be assigned to the unmapped axis.
#[derive(PartialEq, Default, Debug, Clone)]
#[derive_wasm]
pub struct MapCoordMap {
    /// Mapping for 2d coordinates in the route.
    #[serde(rename = "2d")]
    pub mapping_2d: (Axis, Axis),
    // Mapping for 3d coordinates in the route.
    #[serde(rename = "3d")]
    pub mapping_3d: (Axis, Axis, Axis),
}

/// Axis of the map
#[derive(PartialEq, Default, Debug, Clone)]
#[derive_wasm]
pub enum Axis {
    /// Horizontal axis
    #[default]
    #[serde(rename = "x")]
    X,
    /// Vertical axis
    #[serde(rename = "y")]
    Y,
    /// Height axis
    #[serde(rename = "z")]
    Z,
    /// Negative Horizontal axis
    #[serde(rename = "-x")]
    NegX,
    /// Negative Vertical axis
    #[serde(rename = "-y")]
    NegY,
    /// Negative Height axis
    #[serde(rename = "-z")]
    NegZ,
}

impl<'a> PreparedConfig<'a> {
    /// Parse the `coord-map` property in map configs
    pub fn parse_coord_map(&self, coord_map: Value) -> PrepResult<MapCoordMap> {
        let mut obj = config::check_map!(
            self,
            coord_map,
            format!("{}.{}", prop::MAP, prop::COORD_MAP)
        )?;

        let mapping_2d = config::check_required_property!(
            self,
            obj.remove(prop::MAPPING_2D),
            format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_2D)
        )?;
        let mapping_3d = config::check_required_property!(
            self,
            obj.remove(prop::MAPPING_3D),
            format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_3D)
        )?;

        self.check_unused_property(
            obj.keys()
                .next()
                .map(|k| format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, k)),
        )?;

        let mut mapping_2d = config::check_array!(
            self,
            mapping_2d,
            format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_2D)
        )?;
        let mut mapping_3d = config::check_array!(
            self,
            mapping_3d,
            format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_3D)
        )?;

        let (c1_2d, c2_2d) = {
            let c2 = self.pop_coord_map_array(&mut mapping_2d, prop::MAPPING_2D, 2)?;
            let c1 = self.pop_coord_map_array(&mut mapping_2d, prop::MAPPING_2D, 2)?;
            if !mapping_2d.is_empty() {
                return Err(self.err_invalid_coord_array(prop::MAPPING_2D, 2));
            }
            (c1, c2)
        };

        let (c1_3d, c2_3d, c3_3d) = {
            let c3 = self.pop_coord_map_array(&mut mapping_3d, prop::MAPPING_3D, 3)?;
            let c2 = self.pop_coord_map_array(&mut mapping_3d, prop::MAPPING_3D, 3)?;
            let c1 = self.pop_coord_map_array(&mut mapping_3d, prop::MAPPING_3D, 3)?;
            if !mapping_3d.is_empty() {
                return Err(self.err_invalid_coord_array(prop::MAPPING_3D, 3));
            }

            (c1, c2, c3)
        };

        let mapping_2d = {
            let c1 = self.parse_axis(c1_2d, prop::MAPPING_2D, 0)?;
            let c2 = self.parse_axis(c2_2d, prop::MAPPING_2D, 1)?;

            (c1, c2)
        };

        let mapping_3d = {
            let c1 = self.parse_axis(c1_3d, prop::MAPPING_3D, 0)?;
            let c2 = self.parse_axis(c2_3d, prop::MAPPING_3D, 1)?;
            let c3 = self.parse_axis(c3_3d, prop::MAPPING_3D, 2)?;

            (c1, c2, c3)
        };

        Ok(MapCoordMap {
            mapping_2d,
            mapping_3d,
        })
    }

    fn pop_coord_map_array(
        &self,
        vec: &mut Vec<Value>,
        prop: &str,
        dim: usize,
    ) -> PrepResult<Value> {
        match vec.pop() {
            Some(v) => Ok(v),
            None => Err(self.err_invalid_coord_array(prop, dim)),
        }
    }

    fn err_invalid_coord_array(&self, prop: &str, i: usize) -> PrepError {
        PrepError::InvalidConfigPropertyType(
            self.trace.clone(),
            format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop).into(),
            format!("array of {} axes", i).into(),
        )
    }

    fn parse_axis(&self, value: Value, prop: &str, i: usize) -> PrepResult<Axis> {
        match serde_json::from_value::<Axis>(value) {
            Ok(v) => Ok(v),
            Err(_) => Err(PrepError::InvalidConfigPropertyType(
                self.trace.clone(),
                format!("{}.{}.{}[{}]", prop::MAP, prop::COORD_MAP, prop, i).into(),
                "axis string".into(),
            )),
        }
    }
}

#[cfg(test)]
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

        let config = PreparedConfig::default();

        for v in values.into_iter() {
            let result = config.parse_coord_map(v);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.coord-map".into(),
                    "mapping object".into()
                ))
            );
        }
    }

    #[test]
    fn test_missing_properties() {
        let v = json!({});
        let config = PreparedConfig::default();
        let result = config.parse_coord_map(v);
        assert_eq!(
            result,
            Err(PrepError::MissingConfigProperty(
                Default::default(),
                "map.coord-map.2d".into()
            ))
        );

        let v = json!({"2d": {}});
        let result = config.parse_coord_map(v);
        assert_eq!(
            result,
            Err(PrepError::MissingConfigProperty(
                Default::default(),
                "map.coord-map.3d".into()
            ))
        );
    }

    #[test]
    fn test_extra_properties() {
        let v = json!({"2d": {}, "3d": {}, "extra": 1});
        let config = PreparedConfig::default();
        let result = config.parse_coord_map(v);
        assert_eq!(
            result,
            Err(PrepError::UnusedConfigProperty(
                Default::default(),
                "map.coord-map.extra".into()
            ))
        );
    }

    #[test]
    fn test_invalid_coord_map_type() {
        let values = vec![
            json!(null),
            json!(false),
            json!(true),
            json!(1),
            json!({}),
            json!(""),
            json!("hello"),
        ];

        let config = PreparedConfig::default();

        for v in values.iter() {
            let v = json!({"2d": v.clone(), "3d": {}});
            let result = config.parse_coord_map(v);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.coord-map.2d".into(),
                    "array".into()
                ))
            );
        }

        for v in values.into_iter() {
            let v = json!({"2d": [], "3d": v});
            let result = config.parse_coord_map(v);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.coord-map.3d".into(),
                    "array".into()
                ))
            );
        }
    }

    #[test]
    fn test_invalid_axis_count() {
        let values = vec![
            json!([]),
            json!(["x"]),
            json!(["x", "y", "y", "z"]),
            json!(["h", "1", "2", "3", "x"]),
        ];

        let config = PreparedConfig::default();

        for v in values.iter() {
            let v = json!({"2d": v.clone(), "3d": []});
            let result = config.parse_coord_map(v);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.coord-map.2d".into(),
                    "array of 2 axes".into()
                ))
            );
        }

        for v in values.into_iter() {
            let v = json!({"2d": ["x", "y"], "3d": v});
            let result = config.parse_coord_map(v);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    "map.coord-map.3d".into(),
                    "array of 3 axes".into()
                ))
            );
        }

        let v = json!({"2d": ["x", "y", "z"], "3d": []});
        let result = config.parse_coord_map(v);
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.coord-map.2d".into(),
                "array of 2 axes".into()
            ))
        );

        let v = json!({"2d": ["x", "y"], "3d": ["x", "y"]});
        let result = config.parse_coord_map(v);
        assert_eq!(
            result,
            Err(PrepError::InvalidConfigPropertyType(
                Default::default(),
                "map.coord-map.3d".into(),
                "array of 3 axes".into()
            ))
        );
    }

    #[test]
    fn test_invalid_axis() {
        let values = vec![
            (json!(["h", "j"]), 0),
            (json!(["h", "x"]), 0),
            (json!(["x", "h"]), 1),
        ];

        let config = PreparedConfig::default();

        for (v, j) in values.into_iter() {
            let v = json!({"2d": v, "3d": ["h", "j", "k"]});
            let result = config.parse_coord_map(v);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    format!("map.coord-map.2d[{j}]").into(),
                    "axis string".into()
                ))
            );
        }

        let values = vec![
            (json!(["h", "j", "k"]), 0),
            (json!(["h", "x", "k"]), 0),
            (json!(["h", "i", "y"]), 0),
            (json!(["h", "z", "y"]), 0),
            (json!(["x", "h", "l"]), 1),
            (json!(["y", "h", "z"]), 1),
            (json!(["z", "x", "b"]), 2),
        ];

        for (v, j) in values.iter() {
            let v = json!({"2d": ["z", "z"], "3d": v});
            let result = config.parse_coord_map(v);
            assert_eq!(
                result,
                Err(PrepError::InvalidConfigPropertyType(
                    Default::default(),
                    format!("map.coord-map.3d[{j}]").into(),
                    "axis string".into()
                ))
            );
        }
    }

    #[test]
    fn test_valid() {
        let v = json!({
            "2d": ["x", "y"],
            "3d": ["x", "y", "z"],
        });
        let config = PreparedConfig::default();
        let result = config.parse_coord_map(v);
        assert_eq!(
            result,
            Ok(MapCoordMap {
                mapping_2d: (Axis::X, Axis::Y),
                mapping_3d: (Axis::X, Axis::Y, Axis::Z),
            })
        );

        let v = json!({
            "2d": ["x", "x"],
            "3d": ["x", "z", "z"],
        });
        let result = config.parse_coord_map(v);
        assert_eq!(
            result,
            Ok(MapCoordMap {
                mapping_2d: (Axis::X, Axis::X),
                mapping_3d: (Axis::X, Axis::Z, Axis::Z),
            })
        );

        let v = json!({
            "2d": ["-x", "x"],
            "3d": ["x", "-z", "-y"],
        });
        let result = config.parse_coord_map(v);
        assert_eq!(
            result,
            Ok(MapCoordMap {
                mapping_2d: (Axis::NegX, Axis::X),
                mapping_3d: (Axis::X, Axis::NegZ, Axis::NegY),
            })
        );
    }
}
