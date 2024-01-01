use crate::comp::CompResult;
use crate::json::{Cast, Coerce, SafeRouteArray, SafeRouteBlob};
use crate::prep::{Axis, GameCoord};

use super::{CompError, LineContext};

macro_rules! map_coord {
    ($mapping:ident, $coord:expr, $output:ident, $i:tt) => {{
        match $coord.try_coerce_to_f64() {
            Some(n) => {
                match $mapping.$i {
                    Axis::X => $output.0 = n,
                    Axis::Y => $output.1 = n,
                    Axis::Z => $output.2 = n,
                    Axis::NegX => $output.0 = -n,
                    Axis::NegY => $output.1 = -n,
                    Axis::NegZ => $output.2 = -n,
                }
                Ok(())
            }
            None => Err(CompError::InvalidCoordinateValue($coord.coerce_to_repl())),
        }
    }};
}

impl<'c, 'p> LineContext<'c, 'p> {
    pub fn parse_coord(&self, prop: SafeRouteBlob<'_>) -> CompResult<GameCoord> {
        let array = prop
            .try_into_array()
            .map_err(|prop| CompError::InvalidCoordinateType(prop.coerce_to_repl()))?;
        self.parse_coord_array(array)
    }
    /// Transforms the coordinate specified in the route into a game coordinate with the coord map
    /// specified in the config
    pub fn parse_coord_array(&self, array: SafeRouteArray<'_>) -> CompResult<GameCoord> {
        let mut output = GameCoord::default();

        let mut iter = array.into_iter();
        let c1 = iter.next().ok_or(CompError::InvalidCoordinateArray)?;
        let c2 = iter.next().ok_or(CompError::InvalidCoordinateArray)?;
        let c3 = iter.next();
        if iter.next().is_some() {
            return Err(CompError::InvalidCoordinateArray);
        }
        match c3 {
            Some(c3) => {
                if let Some(map) = &self.compiler.config.map {
                    let mapping = &map.coord_map.mapping_3d;
                    map_coord!(mapping, c1, output, 0)?;
                    map_coord!(mapping, c2, output, 1)?;
                    map_coord!(mapping, c3, output, 2)?;
                }
            }
            None => {
                if let Some(map) = &self.compiler.config.map {
                    let mapping = &map.coord_map.mapping_2d;
                    map_coord!(mapping, c1, output, 0)?;
                    map_coord!(mapping, c2, output, 1)?;
                }
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use serde_json::{json, Value};

    use crate::comp::test_utils::CompilerBuilder;
    use crate::json::IntoSafeRouteBlob;
    use crate::prep::{MapCoordMap, MapMetadata, RouteConfig};

    use super::*;

    impl<'c, 'p> LineContext<'c, 'p> {
        fn test_parse_coord(&self, prop: Value) -> CompResult<GameCoord> {
            self.parse_coord(prop.into_unchecked())
        }
    }

    #[test]
    fn test_invalid_type() {
        let vals = vec![
            (json!(null), "null"),
            (json!(true), "true"),
            (json!(false), "false"),
            (json!(0), "0"),
            (json!(1), "1"),
            (json!(-1), "-1"),
            (json!(""), ""),
            (json!("hello"), "hello"),
            (json!({}), "[object object]"),
        ];

        let ctx = LineContext::default();

        for (prop, text) in vals {
            let res = ctx.test_parse_coord(prop);
            assert_eq!(res, Err(CompError::InvalidCoordinateType(text.to_string())));
        }
    }

    #[test]
    fn test_no_map() {
        let ctx = LineContext::default();
        let res = ctx.test_parse_coord(json!([1, 2, 3]));
        assert_eq!(res, Ok(GameCoord(0.0, 0.0, 0.0)));
        let res = ctx.test_parse_coord(json!([2, 3]));
        assert_eq!(res, Ok(GameCoord(0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_array_invalid_length() {
        let ctx = LineContext::default();
        let res0 = ctx.test_parse_coord(json!([]));
        assert_eq!(res0, Err(CompError::InvalidCoordinateArray));
        let res1 = ctx.test_parse_coord(json!([1]));
        assert_eq!(res1, Err(CompError::InvalidCoordinateArray));
        let res4 = ctx.test_parse_coord(json!([1, 2, 3, 4]));
        assert_eq!(res4, Err(CompError::InvalidCoordinateArray));
        let res5 = ctx.test_parse_coord(json!([1, 2, 3, 4, 5]));
        assert_eq!(res5, Err(CompError::InvalidCoordinateArray));
    }

    #[test]
    fn test_array_invalid_value() {
        let ctx = LineContext::default();
        let res2 = ctx.test_parse_coord(json!([1, true]));
        assert_eq!(
            res2,
            Err(CompError::InvalidCoordinateValue("true".to_string()))
        );
        let res3 = ctx.test_parse_coord(json!(["1", [], "hello"]));
        assert_eq!(
            res3,
            Err(CompError::InvalidCoordinateValue(
                "[object array]".to_string()
            ))
        );
        let res2 = ctx.test_parse_coord(json!([null, 0]));
        assert_eq!(
            res2,
            Err(CompError::InvalidCoordinateValue("null".to_string()))
        );
    }

    #[test]
    fn test_array_valid() {
        let project = RouteConfig {
            map: Some(MapMetadata {
                coord_map: MapCoordMap {
                    mapping_2d: (Axis::X, Axis::Z),
                    mapping_3d: (Axis::Z, Axis::Z, Axis::Y),
                },
                ..Default::default()
            }),
            ..Default::default()
        };
        let builder = CompilerBuilder::new(project);
        let compiler = builder.build();
        let ctx = LineContext::with_compiler(&compiler);

        let input = json!([1, 2]);
        assert_eq!(Ok(GameCoord(1.0, 0.0, 2.0)), ctx.test_parse_coord(input));
        let input = json!([1, 2, 3]);
        assert_eq!(Ok(GameCoord(0.0, 3.0, 2.0)), ctx.test_parse_coord(input));
        let input = json!(["1", "2.3", 3]);
        assert_eq!(Ok(GameCoord(0.0, 3.0, 2.3)), ctx.test_parse_coord(input));
        let input = json!(["-1", "0.000"]);
        assert_eq!(Ok(GameCoord(-1.0, 0.0, 0.0)), ctx.test_parse_coord(input));
    }
}
