use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::json::Coerce;
use crate::prop;
use crate::types::GameCoord;

use super::{CompError, Compiler};

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

        /// Optional icon at the movement point
        icon: Option<String>,
    },
    Push,
    Pop,
}

impl CompMovement {
    /// Create movement to a coord without any special properties
    pub fn to(coord: GameCoord) -> Self {
        Self::To {
            to: coord,
            warp: false,
            exclude: false,
            color: None,
            icon: None,
        }
    }
}

impl<'a> Compiler<'a> {
    /// Try compiling a json blob to a movement object
    ///
    /// The following are valid:
    /// - one coords value (array of 2 or 3)
    /// - object with `to` property, and optionally `warp`, `exclude`, `color`
    /// - string `push`
    /// - string `pop`
    pub fn comp_movement(
        &self,
        prop_name: &str,
        prop: Value,
        errors: &mut Vec<CompError>,
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
                    errors.push(CompError::InvalidMovementType);
                    None
                }
            }
            Value::Object(props) => {
                let mut to = None;
                let mut warp = false;
                let mut exclude = false;
                let mut color = None;
                let mut icon = None;

                let mut should_fail = false;

                for (key, value) in props {
                    match key.as_ref() {
                        prop::TO => match self.transform_coord(value) {
                            Ok(coord) => to = Some(coord),
                            Err(e) => {
                                should_fail = true;
                                errors.push(e);
                            }
                        },
                        prop::WARP => match value.try_coerce_to_bool() {
                            Some(b) => warp = b,
                            None => {
                                should_fail = true;
                                errors.push(CompError::InvalidLinePropertyType(format!(
                                    "{prop_name}.{}", prop::WARP
                                )));
                            }
                        },
                        prop::EXCLUDE => match value.try_coerce_to_bool() {
                            Some(b) => exclude = b,
                            None => {
                                should_fail = true;
                                errors.push(CompError::InvalidLinePropertyType(format!(
                                    "{prop_name}.{}", prop::EXCLUDE
                                )));
                            }
                        },
                        prop::COLOR => match value {
                            Value::Null => color = None,
                            Value::String(s) => color = Some(s),
                            _ => {
                                should_fail = true;
                                errors.push(CompError::InvalidLinePropertyType(format!(
                                    "{prop_name}.{}", prop::COLOR
                                )));
                            }
                        },
                        prop::ICON => match value {
                            Value::Array(_) | Value::Object(_) => {
                                errors.push(CompError::InvalidLinePropertyType(format!(
                                    "{prop_name}.{}", prop::ICON
                                )));
                    }
                    _ => icon = Some(value.coerce_to_string()),
                        }
                        _ => {
                            errors
                                .push(CompError::UnusedProperty(format!("{prop_name}.{key}")));
                        }
                    }
                }

                match to {
                    None => {
                        errors.push(CompError::InvalidMovementType);
                        None
                    }
                    Some(to) => {
                        if should_fail {
                            None
                        } else {
                            Some(CompMovement::To {
                                to,
                                warp,
                                exclude,
                                color,
                                icon,
                            })
                        }
                    }
                }
            }
            _ => {
                errors.push(CompError::InvalidMovementType);
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::comp::test_utils;

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
            assert_eq!(compiler.comp_movement("", v, &mut errors), None,);
            assert_eq!(errors, vec![CompError::InvalidMovementType]);
        }
    }

    #[test]
    fn test_propagate_coord_error() {
        let compiler = Compiler::default();
        let mut errors = vec![];
        assert_eq!(
            compiler
                .comp_movement("", json!([1, 2, 3, 4]), &mut errors),
            None
        );
        assert_eq!(errors, vec![CompError::InvalidCoordinateArray]);
    }

    #[test]
    fn test_valid_coord() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let mut errors = vec![];
        assert_eq!(
            compiler
                .comp_movement("", json!([1, 2, 4]), &mut errors),
            Some(CompMovement::to(GameCoord(1.0, 2.0, 4.0)))
        );
        assert_eq!(errors, vec![]);
    }

    #[test]
    fn test_object() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let mut errors = vec![];
        assert_eq!(
            compiler
                .comp_movement(
                    "",
                    json!({
                        "to": [1, 2, 4],
                    }),
                    &mut errors
                ),
            Some(CompMovement::to(GameCoord(1.0, 2.0, 4.0)))
        );
        assert_eq!(errors, vec![]);

        assert_eq!(
            compiler
                .comp_movement(
                    "",
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
                icon: None,
            })
        );
        assert_eq!(errors, vec![]);

        assert_eq!(
            compiler
                .comp_movement(
                    "",
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
                icon: None,
            })
        );
        assert_eq!(errors, vec![]);

        assert_eq!(
            compiler
                .comp_movement(
                    "",
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
                icon: None,
            })
        );
        assert_eq!(errors, vec![]);

        errors.clear();
        assert_eq!(
            compiler
                .comp_movement(
                    "",
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
                icon: None,
            })
        );
        assert_eq!(errors, vec![]);

        errors.clear();
        assert_eq!(
            compiler
                .comp_movement(
                    "",
                    json!({
                        "to": [1, 2, 4],
                        "icon": "something",
                    }),
                    &mut errors
                ),
            Some(CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: false,
                exclude: false,
                color: None,
                icon: Some("something".to_string()),
            })
        );
        assert_eq!(errors, vec![]);

        errors.clear();
        assert_eq!(
            compiler
                .comp_movement(
                    "te",
                    json!({
                        "to": [1, 2, 4],
                        "icon": []
                    }),
                    &mut errors
                ),
            Some(CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: false,
                exclude: false,
                color: None,
                icon: None,
            })
        );
        assert_eq!(
            errors,
            vec![CompError::InvalidLinePropertyType("te.icon".to_string())]
        );

        errors.clear();
        assert_eq!(
            compiler
                .comp_movement(
                    "te",
                    json!({
                        "to": [1, 2, 4],
                        "exclude": "something",
                    }),
                    &mut errors
                ),
            None
        );
        assert_eq!(
            errors,
            vec![CompError::InvalidLinePropertyType("te.exclude".to_string())]
        );

        errors.clear();
        assert_eq!(
            compiler
                .comp_movement(
                    "te",
                    json!({
                        "to": [1, 2, 4],
                        "warp": "something",
                    }),
                    &mut errors
                ),
            None
        );
        assert_eq!(
            errors,
            vec![CompError::InvalidLinePropertyType("te.warp".to_string())]
        );

        errors.clear();
        assert_eq!(
            compiler
                .comp_movement(
                    "test",
                    json!({
                        "to": [1, 2, 4],
                        "color": [],
                    }),
                    &mut errors
                ),
            None
        );
        assert_eq!(
            errors,
            vec![CompError::InvalidLinePropertyType("test.color".to_string())]
        );
    }

    #[test]
    fn test_push_pop() {
        let compiler = Compiler::default();

        let mut errors = vec![];
        assert_eq!(
            compiler.comp_movement("", json!("push"), &mut errors),
            Some(CompMovement::Push)
        );
        assert_eq!(
            compiler.comp_movement("", json!("pop"), &mut errors),
            Some(CompMovement::Pop)
        );
        assert_eq!(errors, vec![]);
    }

    #[test]
    fn test_unused_property() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let mut errors = vec![];
        assert_eq!(
            compiler
                .comp_movement(
                    "test",
                    json!({
                        "to": [1, 2, 4],
                        "unused": 1,
                    }),
                    &mut errors
                ),
            Some(CompMovement::to(GameCoord(1.0, 2.0, 4.0)))
        );
        assert_eq!(
            errors,
            vec![CompError::UnusedProperty("test.unused".to_string())]
        );
    }
}
