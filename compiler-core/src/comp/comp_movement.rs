use celerctypes::GameCoord;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::json::Coerce;

use super::{Compiler, CompilerError};

/// Compiled map movement
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum CompMovement {
    To {
        /// The target coord to move to
        to: GameCoord,
        /// If the movement is a warp
        warp: bool,
        /// If the movement coord should be excluded
        ///
        /// This affects if the map will consider this coord when
        /// changing view to the line
        exclude: bool,
        /// Optional color to override the color of the line
        color: Option<String>,
    },
    Push,
    Pop,
}

impl CompMovement {
    pub fn to(coord: GameCoord) -> Self {
        Self::To {
            to: coord,
            warp: false,
            exclude: false,
            color: None,
        }
    }
}

impl Compiler {
    /// Try compiling a json blob to a movement object
    ///
    /// The following are valid:
    /// - one coords value (array of 2 or 3)
    /// - object with `to` property, and optionally `warp`, `exclude`, `color`
    /// - string `push`
    /// - string `pop`
    pub fn comp_movement(
        &self,
        prop: Value,
        errors: &mut Vec<CompilerError>,
    ) -> Option<CompMovement> {
        if prop.is_array() {
            return match self.transform_coord(prop) {
                Ok(coord) => Some(CompMovement::to(coord)),
                Err(e) => {
                    errors.push(e);
                    None
                }
            };
        }
        match prop {
            Value::String(s) => {
                if s == "push" {
                    Some(CompMovement::Push)
                } else if s == "pop" {
                    Some(CompMovement::Pop)
                } else {
                    errors.push(CompilerError::InvalidMovementType);
                    None
                }
            }
            Value::Object(props) => {
                let mut to = None;
                let mut warp = false;
                let mut exclude = false;
                let mut color = None;

                for (key, value) in props.into_iter() {
                    match key.as_ref() {
                        "to" => {
                            match self.transform_coord(value) {
                                Ok(coord) => to = Some(coord),
                                Err(e) => errors.push(e),
                            }
                        },
                        "warp" => {
                            match value.try_coerce_to_bool() {
                                Some(b) => warp = b,
                                None => {
                                    errors.push(CompilerError::InvalidMovementType);
                                }
                            }
                        }
                        "exclude" => {
                            match value.try_coerce_to_bool() {
                                Some(b) => exclude = b,
                                None => {
                                    errors.push(CompilerError::InvalidMovementType);
                                }
                            }
                        }
                        "color" => {
                            match value {
                                Value::Null => color = None,
                                Value::String(s) => color = Some(s),
                                _ => {
                                    errors.push(CompilerError::InvalidMovementType);
                                }
                            }
                        }
                        _ => todo!()
                    }
                }
                match to {
                    None => {
                        errors.push(CompilerError::InvalidMovementType);
                        None
                    },
                    Some(to) => {
                        if !errors.is_empty() {
                            None
                        } else {
                            Some(CompMovement::To {
                                to,
                                warp,
                                exclude,
                                color,
                            })
                        }
                    },
                }
            }, 
            _ => {
                errors.push(CompilerError::InvalidMovementType);
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use celerctypes::{Axis, MapCoordMap, MapMetadata, RouteMetadata};
    use serde_json::json;

    use crate::comp::CompilerBuilder;

    use super::*;

    #[test]
    fn test_value_invalid() {
        let vals = vec![
            json!(1),
            json!(null),
            json!(true),
            json!(false),
            json!("hello"),
            json!(""),
            json!({}),
        ];

        let compiler = Compiler::default();

        for v in vals.into_iter() {
            let mut errors = vec![];
            assert_eq!(compiler.comp_movement(v, &mut errors), None,);
            assert_eq!(errors, vec![CompilerError::InvalidMovementType]);
        }
    }

    #[test]
    fn test_propagate_coord_error() {
        let compiler = Compiler::default();
        let mut errors = vec![];
        assert_eq!(
            compiler.comp_movement(json!([1, 2, 3, 4]), &mut errors),
            None
        );
        assert_eq!(errors, vec![CompilerError::InvalidCoordinateArray]);
    }

    fn create_test_compiler_with_coord_transform() -> Compiler {
        let project = RouteMetadata {
            map: MapMetadata {
                coord_map: MapCoordMap {
                    mapping_3d: (Axis::X, Axis::Y, Axis::Z),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let builder = CompilerBuilder::new(project, Default::default(), Default::default());
        builder.build()
    }

    #[test]
    fn test_valid_coord() {
        let compiler = create_test_compiler_with_coord_transform();
        let mut errors = vec![];
        assert_eq!(
            compiler.comp_movement(json!([1, 2, 4]), &mut errors),
            Some(CompMovement::to(GameCoord(1.0, 2.0, 4.0)))
        );
        assert_eq!(errors, vec![]);
    }

    #[test]
    fn test_object() {
        let compiler = create_test_compiler_with_coord_transform();
        let mut errors = vec![];
        assert_eq!(
            compiler.comp_movement(
                json!({
                    "to": [1, 2, 4],
                }),
                &mut errors
            ),
            Some(CompMovement::to(GameCoord(1.0, 2.0, 4.0)))
        );
        assert_eq!(errors, vec![]);

        assert_eq!(
            compiler.comp_movement(
                json!({
                    "to": [1, 2, 4],
                    "warp": true,
                }),
                &mut errors
            ),
            Some(CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: true,
                exclude: false,
                color: None,
            })
        );
        assert_eq!(errors, vec![]);

        assert_eq!(
            compiler.comp_movement(
                json!({
                    "to": [1, 2, 4],
                    "warp": true,
                    "exclude": true,
                    "color": null
                }),
                &mut errors
            ),
            Some(CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: true,
                exclude: true,
                color: None,
            })
        );
        assert_eq!(errors, vec![]);

        assert_eq!(
            compiler.comp_movement(
                json!({
                    "to": [1, 2, 4],
                    "warp": "false",
                    "exclude": true,
                    "color": "red",
                }),
                &mut errors
            ),
            Some(CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: false,
                exclude: true,
                color: Some("red".to_string()),
            })
        );
        assert_eq!(errors, vec![]);

        errors.clear();
        assert_eq!(
            compiler.comp_movement(
                json!({
                    "to": [1, 2, 4],
                    "warp": 0,
                }),
                &mut errors
            ),
            Some(CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: false,
                exclude: false,
                color: None,
            })
        );
        assert_eq!(errors, vec![]);

        errors.clear();
        assert_eq!(
            compiler.comp_movement(
                json!({
                    "to": [1, 2, 4],
                    "exclude": "something",
                }),
                &mut errors
            ),
            None
        );
        assert_eq!(errors, vec![CompilerError::InvalidMovementType]);

        errors.clear();
        assert_eq!(
            compiler.comp_movement(
                json!({
                    "to": [1, 2, 4],
                    "color": [],
                }),
                &mut errors
            ),
            None
        );
        assert_eq!(errors, vec![CompilerError::InvalidMovementType]);

    }

    #[test]
    fn test_push_pop() {
        let compiler = Compiler::default();

        let mut errors = vec![];
        assert_eq!(
            compiler.comp_movement(json!("push"), &mut errors),
            Some(CompMovement::Push)
        );
        assert_eq!(
            compiler.comp_movement(json!("pop"), &mut errors),
            Some(CompMovement::Pop)
        );
        assert_eq!(errors, vec![]);
    }

    #[test]
    fn test_unused_property() {
        todo!()
    }

}
