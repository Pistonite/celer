use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{json::Coerce, prep::RouteBlob};
use crate::prop;
use crate::prep::GameCoord;
use crate::macros::derive_wasm;

use super::{CompError, LineContext};

/// Compiled map movement
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
#[serde(tag = "type")]
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

impl<'c, 'p> LineContext<'c, 'p> {
    /// Compiling a json blob to a movement object and add it to the line's movement
    ///
    /// The following are valid:
    /// - one coords value (array of 2 or 3)
    /// - object with `to` property, and optionally `warp`, `exclude`, `color`
    /// - string `push`
    /// - string `pop`
    pub fn compile_movement(
        &mut self,
        prop_name: &str,
        prop: &RouteBlob,
    ) {
        if prop.is_array() {
            match self.parse_coord(prop) {
                Ok(coord) => {
                    let m = CompMovement::to(coord);
                    self.line.movements.push(m);
                }
                Err(e) => {
                    self.errors.push(e);
                }
            };
            return;
        }
        match prop {
            RouteBlob::Object(props) => {
                self.compile_movement_obj(prop_name, props);
            }
            RouteBlob::Prim(Value::String(s)) => {
                if s == "push" {
                    self.line.movements.push(CompMovement::Push);
                } else if s == "pop" {
                    self.line.movements.push(CompMovement::Pop);
                } else {
                    self.errors.push(CompError::InvalidMovementType);
                }
            }
            _ => {
                self.errors.push(CompError::InvalidMovementType);
            }
        }
    }

    fn compile_movement_obj(&mut self, prop_name: &str, props: &BTreeMap<String, RouteBlob>) {
        let mut to = None;
        let mut warp = false;
        let mut exclude = false;
        let mut color = None;
        let mut icon = None;

        let mut should_fail = false;

        for (key, value) in props {
            match key.as_ref() {
                prop::TO => match self.parse_coord(value) {
                    Ok(coord) => to = Some(coord),
                    Err(e) => {
                        should_fail = true;
                        self.errors.push(e);
                    }
                },
                prop::WARP => match value.try_coerce_to_bool() {
                    Some(b) => warp = b,
                    None => {
                        should_fail = true;
                        self.errors.push(CompError::InvalidLinePropertyType(format!(
                        "{prop_name}.{}",
                        prop::WARP
                    )));
                    }
                },
                prop::EXCLUDE => match value.try_coerce_to_bool() {
                    Some(b) => exclude = b,
                    None => {
                        should_fail = true;
                        self.errors.push(CompError::InvalidLinePropertyType(format!(
                        "{prop_name}.{}",
                        prop::EXCLUDE
                    )));
                    }
                },
                prop::COLOR => match value {
                    RouteBlob::Prim(Value::Null) => color = None,
                    RouteBlob::Prim(Value::String(s)) => color = Some(s.to_owned()),
                    _ => {
                        should_fail = true;
                        self.errors.push(CompError::InvalidLinePropertyType(format!(
                        "{prop_name}.{}",
                        prop::COLOR
                    )));
                    }
                },
                prop::ICON => match value {
                    RouteBlob::Array(_) | RouteBlob::Object(_) => {
                        self.errors.push(CompError::InvalidLinePropertyType(format!(
                        "{prop_name}.{}",
                        prop::ICON
                    )));
                    }
                    _ => icon = Some(value.coerce_to_string()),
                },
                _ => {
                    self.errors.push(CompError::UnusedProperty(format!("{prop_name}.{key}")));
                }
            }
        }

        match to {
            None => {
                self.errors.push(CompError::InvalidMovementType);
            }
            Some(to) => {
                if !should_fail {
                    let m = CompMovement::To {
                        to,
                        warp,
                        exclude,
                        color,
                        icon,
                    };
                    self.line.movements.push(m);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use serde_json::json;

    use crate::comp::test_utils;

    use super::*;

    impl<'c, 'p> LineContext<'c, 'p> {
        fn test_compile_movement(&mut self, prop_name: &str, prop: Value) {
            self.compile_movement(prop_name, &prop.into())
        }
    }

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

        let mut ctx = LineContext::default();

        for v in vals.into_iter() {
            ctx.errors.clear();
            ctx.test_compile_movement("", v);
            assert_eq!(ctx.errors, vec![CompError::InvalidMovementType]);
        }
    }

    #[test]
    fn test_propagate_coord_error() {
        let mut ctx = LineContext::default();
        ctx.test_compile_movement("", json!([1, 2, 3, 4]));
        assert_eq!(ctx.errors, vec![CompError::InvalidCoordinateArray]);
    }

    #[test]
    fn test_valid_coord() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let mut ctx = LineContext::with_compiler(&compiler);
        ctx.test_compile_movement("", json!([1, 2, 4]));
        assert_eq!(
            ctx.line.movements,
            vec![CompMovement::to(GameCoord(1.0, 2.0, 4.0))]
        );
        assert_eq!(ctx.errors, vec![]);
    }

    #[test]
    fn test_object() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let mut ctx = LineContext::with_compiler(&compiler);
        ctx.test_compile_movement(
            "",
            json!({
                "to": [1, 2, 4],
            }),
        );
        assert_eq!(
            ctx.line.movements,
            vec![CompMovement::to(GameCoord(1.0, 2.0, 4.0))]
        );
        assert_eq!(ctx.errors, vec![]);
        ctx.line.movements.clear();
        ctx.errors.clear();

        ctx.test_compile_movement(
            "",
            json!({
                "to": [1, 2, 4],
                "warp": true,
            }),
        );
        assert_eq!(
            ctx.line.movements,
            vec![CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: true,
                exclude: false,
                color: None,
                icon: None,
            }]
        );
        assert_eq!(ctx.errors, vec![]);
        ctx.line.movements.clear();
        ctx.errors.clear();

        ctx.test_compile_movement(
            "",
            json!({
                "to": [1, 2, 4],
                "warp": true,
                "exclude": true,
                "color": null
            }),
        );

        assert_eq!(
            ctx.line.movements,
            vec![CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: true,
                exclude: true,
                color: None,
                icon: None,
            }]
        );
        assert_eq!(ctx.errors, vec![]);
        ctx.line.movements.clear();
        ctx.errors.clear();

        ctx.test_compile_movement(
            "",
            json!({
                "to": [1, 2, 4],
                "warp": "false",
                "exclude": true,
                "color": "red",
            }),
        );
        assert_eq!(
            ctx.line.movements,
            vec![CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: false,
                exclude: true,
                color: Some("red".to_string()),
                icon: None,
            }]
        );
        assert_eq!(ctx.errors, vec![]);
        ctx.line.movements.clear();
        ctx.errors.clear();

        ctx.test_compile_movement(
            "",
            json!({
                "to": [1, 2, 4],
                "warp": 0,
            }),
        );
        assert_eq!(
            ctx.line.movements,
            vec![CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: false,
                exclude: false,
                color: None,
                icon: None,
            }]
        );
        assert_eq!(ctx.errors, vec![]);
        ctx.line.movements.clear();
        ctx.errors.clear();

        ctx.test_compile_movement(
            "",
            json!({
                "to": [1, 2, 4],
                "icon": "something",
            }),
        );
        assert_eq!(
            ctx.line.movements,
            vec![CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: false,
                exclude: false,
                color: None,
                icon: Some("something".to_string()),
            }]
        );
        assert_eq!(ctx.errors, vec![]);
        ctx.line.movements.clear();
        ctx.errors.clear();

        ctx.test_compile_movement(
            "te",
            json!({
                "to": [1, 2, 4],
                "icon": []
            }),
        );
        assert_eq!(
            ctx.line.movements,
            vec![CompMovement::To {
                to: GameCoord(1.0, 2.0, 4.0),
                warp: false,
                exclude: false,
                color: None,
                icon: None,
            }]
        );
        assert_eq!(
            ctx.errors,
            vec![CompError::InvalidLinePropertyType("te.icon".to_string())]
        );
        ctx.line.movements.clear();
        ctx.errors.clear();

        ctx.test_compile_movement(
            "te",
            json!({
                "to": [1, 2, 4],
                "exclude": "something",
            }),
        );
        assert_eq!(ctx.line.movements, vec![]);
        assert_eq!(
            ctx.errors,
            vec![CompError::InvalidLinePropertyType("te.exclude".to_string())]
        );
        ctx.line.movements.clear();
        ctx.errors.clear();

        ctx.test_compile_movement(
            "te",
            json!({
                "to": [1, 2, 4],
                "warp": "something",
            }),
        );
        assert_eq!(ctx.line.movements, vec![]);
        assert_eq!(
            ctx.errors,
            vec![CompError::InvalidLinePropertyType("te.warp".to_string())]
        );
        ctx.line.movements.clear();
        ctx.errors.clear();

        ctx.test_compile_movement(
            "test",
            json!({
                "to": [1, 2, 4],
                "color": [],
            }),
        );
        assert_eq!(ctx.line.movements, vec![]);
        assert_eq!(
            ctx.errors,
            vec![CompError::InvalidLinePropertyType("test.color".to_string())]
        );
    }

    #[test]
    fn test_push_pop() {
        let mut ctx = LineContext::default();

        ctx.test_compile_movement("", json!("push"));
        ctx.test_compile_movement("", json!("pop"));
        assert_eq!(
            ctx.line.movements,
            vec![CompMovement::Push, CompMovement::Pop]
        );
        assert_eq!(ctx.errors, vec![]);
    }

    #[test]
    fn test_unused_property() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let mut ctx = LineContext::with_compiler(&compiler);
        ctx.test_compile_movement(
            "test",
            json!({
                "to": [1, 2, 4],
                "unused": 1,
            }),
        );
        assert_eq!(
            ctx.line.movements,
            vec![CompMovement::to(GameCoord(1.0, 2.0, 4.0))]
        );
        assert_eq!(
            ctx.errors,
            vec![CompError::UnusedProperty("test.unused".to_string())]
        );
    }
}
