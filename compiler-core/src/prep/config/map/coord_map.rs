//! Packs map.coord-map into [`MapCoordMap`]

use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::macros::derive_wasm;
use crate::res::Loader;
use crate::json::Cast;
use crate::prep::{PrepResult, PreparedConfig};
use crate::prop;


/// The mapping if 2 coordinates are specified in the route
///
/// For example, ["x", "z"] will map the coordinates
/// to the x (horizontal) and z (height) axis of the map.
///
/// Default value of 0 will be assigned to the unmapped axis.
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
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
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub enum Axis {
    /// Horizontal axis
    #[default]
    X,
    /// Vertical axis
    Y,
    /// Height axis
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



macro_rules! check_coord_map_required_property {
    ($property:expr, $trace:ident, $value:expr) => {
        match $value {
            Some(v) => Ok(v),
            None => Err(PackerError::MissingConfigProperty(
                $trace.clone(),
                format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, $property),
            )),
        }
    };
}

macro_rules! pop_coord_map_array {
    ($vec:ident, $trace:ident, $prop:expr) => {
        match $vec.pop() {
            Some(v) => Ok(v),
            None => Err(PackerError::InvalidConfigProperty(
                $trace.clone(),
                format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, $prop),
            )),
        }
    };
}

macro_rules! try_parse_axis {
    ($value:ident, $trace:ident, $prop:expr, $i:expr) => {
        match serde_json::from_value::<Axis>($value) {
            Ok(v) => Ok(v),
            Err(_) => Err(PackerError::InvalidConfigProperty(
                $trace.clone(),
                format!("{}.{}.{}[{}]", prop::MAP, prop::COORD_MAP, $prop, $i),
            )),
        }
    };
}

impl<L> PreparedConfig<L> where L: Loader {
    /// Parse the `coord-map` property in map configs
    pub fn parse_coord_map(&self, coord_map: Value) -> PrepResult<MapCoordMap> {
    let mut obj = value.try_into_object().map_err(|_| {
        PackerError::InvalidConfigProperty(
            trace.clone(),
            format!("{}.{}", prop::MAP, prop::COORD_MAP),
        )
    })?;

    let mapping_2d =
        check_coord_map_required_property!(prop::MAPPING_2D, trace, obj.remove(prop::MAPPING_2D))?;
    let mapping_3d =
        check_coord_map_required_property!(prop::MAPPING_3D, trace, obj.remove(prop::MAPPING_3D))?;

    if let Some(k) = obj.keys().next() {
        return Err(PackerError::UnusedConfigProperty(
            trace.clone(),
            format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, k),
        ));
    }

    let mut mapping_2d = mapping_2d.try_into_array().map_err(|_| {
        PackerError::InvalidConfigProperty(
            trace.clone(),
            format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_2D),
        )
    })?;

    let mut mapping_3d = mapping_3d.try_into_array().map_err(|_| {
        PackerError::InvalidConfigProperty(
            trace.clone(),
            format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_3D),
        )
    })?;

    let (c1_2d, c2_2d) = {
        let c2 = pop_coord_map_array!(mapping_2d, trace, prop::MAPPING_2D)?;
        let c1 = pop_coord_map_array!(mapping_2d, trace, prop::MAPPING_2D)?;
        if !mapping_2d.is_empty() {
            return Err(PackerError::InvalidConfigProperty(
                trace.clone(),
                format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_2D),
            ));
        }

        (c1, c2)
    };

    let (c1_3d, c2_3d, c3_3d) = {
        let c3 = pop_coord_map_array!(mapping_3d, trace, prop::MAPPING_3D)?;
        let c2 = pop_coord_map_array!(mapping_3d, trace, prop::MAPPING_3D)?;
        let c1 = pop_coord_map_array!(mapping_3d, trace, prop::MAPPING_3D)?;
        if !mapping_3d.is_empty() {
            return Err(PackerError::InvalidConfigProperty(
                trace.clone(),
                format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_3D),
            ));
        }

        (c1, c2, c3)
    };

    let mapping_2d = {
        let c1 = try_parse_axis!(c1_2d, trace, prop::MAPPING_2D, 0)?;
        let c2 = try_parse_axis!(c2_2d, trace, prop::MAPPING_2D, 1)?;

        (c1, c2)
    };

    let mapping_3d = {
        let c1 = try_parse_axis!(c1_3d, trace, prop::MAPPING_3D, 0)?;
        let c2 = try_parse_axis!(c2_3d, trace, prop::MAPPING_3D, 1)?;
        let c3 = try_parse_axis!(c3_3d, trace, prop::MAPPING_3D, 2)?;

        (c1, c2, c3)
    };

    Ok(MapCoordMap {
        mapping_2d,
        mapping_3d,
    })
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

        let mut trace = ConfigTrace::default();

        for (i, v) in values.into_iter().enumerate() {
            trace.push(i);
            let result = pack_coord_map(v, &trace);
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    trace.clone(),
                    "map.coord-map".to_string()
                ))
            );
            trace.pop();
        }
    }

    #[test]
    fn test_missing_properties() {
        let v = json!({});
        let result = pack_coord_map(v, &Default::default());
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                Default::default(),
                "map.coord-map.2d".to_string()
            ))
        );

        let v = json!({"2d": {}});
        let result = pack_coord_map(v, &Default::default());
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                Default::default(),
                "map.coord-map.3d".to_string()
            ))
        );
    }

    #[test]
    fn test_extra_properties() {
        let v = json!({"2d": {}, "3d": {}, "extra": 1});
        let result = pack_coord_map(v, &Default::default());
        assert_eq!(
            result,
            Err(PackerError::UnusedConfigProperty(
                Default::default(),
                "map.coord-map.extra".to_string()
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

        for v in values.iter() {
            let v = json!({"2d": v.clone(), "3d": {}});
            let result = pack_coord_map(v, &Default::default());
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.coord-map.2d".to_string()
                ))
            );
        }

        for v in values.into_iter() {
            let v = json!({"2d": [], "3d": v});
            let result = pack_coord_map(v, &Default::default());
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.coord-map.3d".to_string()
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

        for v in values.iter() {
            let v = json!({"2d": v.clone(), "3d": []});
            let result = pack_coord_map(v, &Default::default());
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.coord-map.2d".to_string()
                ))
            );
        }

        for v in values.into_iter() {
            let v = json!({"2d": ["x", "y"], "3d": v});
            let result = pack_coord_map(v, &Default::default());
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    "map.coord-map.3d".to_string()
                ))
            );
        }

        let v = json!({"2d": ["x", "y", "z"], "3d": []});
        let result = pack_coord_map(v, &Default::default());
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                Default::default(),
                "map.coord-map.2d".to_string()
            ))
        );

        let v = json!({"2d": ["x", "y"], "3d": ["x", "y"]});
        let result = pack_coord_map(v, &Default::default());
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                Default::default(),
                "map.coord-map.3d".to_string()
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

        for (v, j) in values.into_iter() {
            let v = json!({"2d": v, "3d": ["h", "j", "k"]});
            let result = pack_coord_map(v, &Default::default());
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    format!("map.coord-map.2d[{j}]")
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
            let result = pack_coord_map(v, &Default::default());
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    Default::default(),
                    format!("map.coord-map.3d[{j}]")
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
        let result = pack_coord_map(v, &Default::default());
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
        let result = pack_coord_map(v, &Default::default());
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
        let result = pack_coord_map(v, &Default::default());
        assert_eq!(
            result,
            Ok(MapCoordMap {
                mapping_2d: (Axis::NegX, Axis::X),
                mapping_3d: (Axis::X, Axis::NegZ, Axis::NegY),
            })
        );
    }
}
