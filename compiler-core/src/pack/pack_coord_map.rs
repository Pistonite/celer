//! Packs map.coord-map into [`MapCoordMap`]

use celerctypes::{Axis, MapCoordMap};
use serde_json::Value;

use crate::comp::prop;
use crate::json::Cast;

use super::{PackerError, PackerResult};

macro_rules! check_coord_map_required_property {
    ($property:expr, $index:ident, $value:expr) => {
        match $value {
            Some(v) => Ok(v),
            None => Err(PackerError::MissingConfigProperty(
                $index,
                format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, $property),
            )),
        }
    };
}

macro_rules! pop_coord_map_array {
    ($vec:ident, $index:ident, $prop:expr) => {
        match $vec.pop() {
            Some(v) => Ok(v),
            None => Err(PackerError::InvalidConfigProperty(
                $index,
                format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, $prop),
            )),
        }
    };
}

macro_rules! try_parse_axis {
    ($value:ident, $index:ident, $prop:expr, $i:expr) => {
        match serde_json::from_value::<Axis>($value) {
            Ok(v) => Ok(v),
            Err(_) => Err(PackerError::InvalidConfigProperty(
                $index,
                format!("{}.{}.{}[{}]", prop::MAP, prop::COORD_MAP, $prop, $i),
            )),
        }
    };
}

pub async fn pack_coord_map(value: Value, index: usize) -> PackerResult<MapCoordMap> {
    let mut obj = value.try_into_object().map_err(|_| {
        PackerError::InvalidConfigProperty(index, format!("{}.{}", prop::MAP, prop::COORD_MAP))
    })?;

    let mapping_2d =
        check_coord_map_required_property!(prop::MAPPING_2D, index, obj.remove(prop::MAPPING_2D))?;
    let mapping_3d =
        check_coord_map_required_property!(prop::MAPPING_3D, index, obj.remove(prop::MAPPING_3D))?;

    if let Some(k) = obj.keys().next() {
        return Err(PackerError::UnusedConfigProperty(
            index,
            format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, k),
        ));
    }

    let mut mapping_2d = mapping_2d.try_into_array().map_err(|_| {
        PackerError::InvalidConfigProperty(
            index,
            format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_2D),
        )
    })?;

    let mut mapping_3d = mapping_3d.try_into_array().map_err(|_| {
        PackerError::InvalidConfigProperty(
            index,
            format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_3D),
        )
    })?;

    let (c1_2d, c2_2d) = {
        let c2 = pop_coord_map_array!(mapping_2d, index, prop::MAPPING_2D)?;
        let c1 = pop_coord_map_array!(mapping_2d, index, prop::MAPPING_2D)?;
        if !mapping_2d.is_empty() {
            return Err(PackerError::InvalidConfigProperty(
                index,
                format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_2D),
            ));
        }

        (c1, c2)
    };

    let (c1_3d, c2_3d, c3_3d) = {
        let c3 = pop_coord_map_array!(mapping_3d, index, prop::MAPPING_3D)?;
        let c2 = pop_coord_map_array!(mapping_3d, index, prop::MAPPING_3D)?;
        let c1 = pop_coord_map_array!(mapping_3d, index, prop::MAPPING_3D)?;
        if !mapping_3d.is_empty() {
            return Err(PackerError::InvalidConfigProperty(
                index,
                format!("{}.{}.{}", prop::MAP, prop::COORD_MAP, prop::MAPPING_3D),
            ));
        }

        (c1, c2, c3)
    };

    let mapping_2d = {
        let c1 = try_parse_axis!(c1_2d, index, prop::MAPPING_2D, 0)?;
        let c2 = try_parse_axis!(c2_2d, index, prop::MAPPING_2D, 1)?;

        (c1, c2)
    };

    let mapping_3d = {
        let c1 = try_parse_axis!(c1_3d, index, prop::MAPPING_3D, 0)?;
        let c2 = try_parse_axis!(c2_3d, index, prop::MAPPING_3D, 1)?;
        let c3 = try_parse_axis!(c3_3d, index, prop::MAPPING_3D, 2)?;

        (c1, c2, c3)
    };

    Ok(MapCoordMap {
        mapping_2d,
        mapping_3d,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::json;

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
            let result = pack_coord_map(v, i).await;
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    i,
                    "map.coord-map".to_string()
                ))
            );
        }
    }

    #[tokio::test]
    async fn test_missing_properties() {
        let v = json!({});
        let result = pack_coord_map(v, 2).await;
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                2,
                "map.coord-map.2d".to_string()
            ))
        );

        let v = json!({"2d": {}});
        let result = pack_coord_map(v, 1).await;
        assert_eq!(
            result,
            Err(PackerError::MissingConfigProperty(
                1,
                "map.coord-map.3d".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn test_extra_properties() {
        let v = json!({"2d": {}, "3d": {}, "extra": 1});
        let result = pack_coord_map(v, 2).await;
        assert_eq!(
            result,
            Err(PackerError::UnusedConfigProperty(
                2,
                "map.coord-map.extra".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn test_invalid_coord_map_type() {
        let values = vec![
            json!(null),
            json!(false),
            json!(true),
            json!(1),
            json!({}),
            json!(""),
            json!("hello"),
        ];

        for (i, v) in values.iter().enumerate() {
            let v = json!({"2d": v.clone(), "3d": {}});
            let result = pack_coord_map(v, i).await;
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    i,
                    "map.coord-map.2d".to_string()
                ))
            );
        }

        for (i, v) in values.into_iter().enumerate() {
            let v = json!({"2d": [], "3d": v});
            let result = pack_coord_map(v, i).await;
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    i,
                    "map.coord-map.3d".to_string()
                ))
            );
        }
    }

    #[tokio::test]
    async fn test_invalid_axis_count() {
        let values = vec![
            json!([]),
            json!(["x"]),
            json!(["x", "y", "y", "z"]),
            json!(["h", "1", "2", "3", "x"]),
        ];

        for (i, v) in values.iter().enumerate() {
            let v = json!({"2d": v.clone(), "3d": []});
            let result = pack_coord_map(v, i).await;
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    i,
                    "map.coord-map.2d".to_string()
                ))
            );
        }

        for (i, v) in values.into_iter().enumerate() {
            let v = json!({"2d": ["x", "y"], "3d": v});
            let result = pack_coord_map(v, i).await;
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    i,
                    "map.coord-map.3d".to_string()
                ))
            );
        }

        let v = json!({"2d": ["x", "y", "z"], "3d": []});
        let result = pack_coord_map(v, 1).await;
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                1,
                "map.coord-map.2d".to_string()
            ))
        );

        let v = json!({"2d": ["x", "y"], "3d": ["x", "y"]});
        let result = pack_coord_map(v, 2).await;
        assert_eq!(
            result,
            Err(PackerError::InvalidConfigProperty(
                2,
                "map.coord-map.3d".to_string()
            ))
        );
    }

    #[tokio::test]
    async fn test_invalid_axis() {
        let values = vec![
            (json!(["h", "j"]), 0),
            (json!(["h", "x"]), 0),
            (json!(["x", "h"]), 1),
        ];

        for (i, (v, j)) in values.into_iter().enumerate() {
            let v = json!({"2d": v, "3d": ["h", "j", "k"]});
            let result = pack_coord_map(v, i).await;
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    i,
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

        for (i, (v, j)) in values.iter().enumerate() {
            let v = json!({"2d": ["z", "z"], "3d": v});
            let result = pack_coord_map(v, i).await;
            assert_eq!(
                result,
                Err(PackerError::InvalidConfigProperty(
                    i,
                    format!("map.coord-map.3d[{j}]")
                ))
            );
        }
    }

    #[tokio::test]
    async fn test_valid() {
        let v = json!({
            "2d": ["x", "y"],
            "3d": ["x", "y", "z"],
        });
        let result = pack_coord_map(v, 0).await;
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
        let result = pack_coord_map(v, 0).await;
        assert_eq!(
            result,
            Ok(MapCoordMap {
                mapping_2d: (Axis::X, Axis::X),
                mapping_3d: (Axis::X, Axis::Z, Axis::Z),
            })
        );
    }
}
