use celerctypes::{Axis, GameCoord};
use serde_json::Value;

use crate::json::{Coerce, Cast};

use super::{Compiler, CompilerError};

macro_rules! map_coord {
    ($mapping:ident, $array:ident, $output:ident, $i:tt) => {{
        let i = $i;
        match $array[i].try_coerce_to_f64() {
            Some(n) => {
                match $mapping.$i {
                    Axis::X => $output.0 = n,
                    Axis::Y => $output.1 = n,
                    Axis::Z => $output.2 = n,
                }
                Ok(())
            }
            None => Err(CompilerError::InvalidCoordinateValue(
                $array[i].coerce_to_repl(),
            )),
        }
    }};
}

impl Compiler {
    pub fn transform_coord(&self, prop: Value) -> Result<GameCoord, CompilerError> {
        let array = prop.try_into_array().map_err(|prop| {
            CompilerError::InvalidCoordinateType(prop.coerce_to_repl())
        })?;
        let mut output = GameCoord::default();
        match array.len() {
            2 => {
                let mapping = &self.project.map.coord_map.mapping_2d;
                map_coord!(mapping, array, output, 0)?;
                map_coord!(mapping, array, output, 1)?;
            }
            3 => {
                let mapping = &self.project.map.coord_map.mapping_3d;
                map_coord!(mapping, array, output, 0)?;
                map_coord!(mapping, array, output, 1)?;
                map_coord!(mapping, array, output, 2)?;
            }
            _ => return Err(CompilerError::InvalidCoordinateArray),
        }

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use celerctypes::{MapCoordMap, MapMetadata, RouteMetadata};
    use serde_json::json;

    use crate::comp::CompilerBuilder;

    use super::*;

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

        let compiler = Compiler::default();

        for (prop, text) in vals {
            let res = compiler.transform_coord(prop);
            assert_eq!(
                res,
                Err(CompilerError::InvalidCoordinateType(text.to_string()))
            );
        }
    }

    #[test]
    fn test_array_invalid_length() {
        let compiler = Compiler::default();
        let res0 = compiler.transform_coord(json!([]));
        assert_eq!(res0, Err(CompilerError::InvalidCoordinateArray));
        let res1 = compiler.transform_coord(json!([1]));
        assert_eq!(res1, Err(CompilerError::InvalidCoordinateArray));
        let res4 = compiler.transform_coord(json!([1, 2, 3, 4]));
        assert_eq!(res4, Err(CompilerError::InvalidCoordinateArray));
        let res5 = compiler.transform_coord(json!([1, 2, 3, 4, 5]));
        assert_eq!(res5, Err(CompilerError::InvalidCoordinateArray));
    }

    #[test]
    fn test_array_invalid_value() {
        let compiler = Compiler::default();
        let res2 = compiler.transform_coord(json!([1, true]));
        assert_eq!(
            res2,
            Err(CompilerError::InvalidCoordinateValue("true".to_string()))
        );
        let res3 = compiler.transform_coord(json!(["1", [], "hello"]));
        assert_eq!(
            res3,
            Err(CompilerError::InvalidCoordinateValue(
                "[object array]".to_string()
            ))
        );
        let res2 = compiler.transform_coord(json!([null, 0]));
        assert_eq!(
            res2,
            Err(CompilerError::InvalidCoordinateValue("null".to_string()))
        );
    }

    #[test]
    fn test_array_valid() {
        let project = RouteMetadata {
            map: MapMetadata {
                coord_map: MapCoordMap {
                    mapping_2d: (Axis::X, Axis::Z),
                    mapping_3d: (Axis::Z, Axis::Z, Axis::Y),
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let builder = CompilerBuilder::new(project, Default::default(), Default::default());
        let compiler = builder.build();

        let input = json!([1, 2]);
        assert_eq!(
            Ok(GameCoord(1.0, 0.0, 2.0)),
            compiler.transform_coord(input)
        );
        let input = json!([1, 2, 3]);
        assert_eq!(
            Ok(GameCoord(0.0, 3.0, 2.0)),
            compiler.transform_coord(input)
        );
        let input = json!(["1", "2.3", 3]);
        assert_eq!(
            Ok(GameCoord(0.0, 3.0, 2.3)),
            compiler.transform_coord(input)
        );
        let input = json!(["-1", "0.000"]);
        assert_eq!(
            Ok(GameCoord(-1.0, 0.0, 0.0)),
            compiler.transform_coord(input)
        );
    }
}
