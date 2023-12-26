use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::json::{Cast, Coerce};
use crate::prep::{GameCoord, RouteBlob};
use crate::prop;
use crate::macros::derive_wasm;

use super::{CompError, LineContext};

/// Data of a marker specified by the `markers` property
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct CompMarker {
    /// The coord of the marker
    pub at: GameCoord,
    /// The color of the marker
    pub color: Option<String>,
}

impl CompMarker {
    /// Create a marker at the given coord with color inherited from line
    pub fn at(at: GameCoord) -> Self {
        Self { at, color: None }
    }
}

impl<'c, 'p> LineContext<'c, 'p> {
    /// Try compiling a json blob to a marker object and add it to the line.
    ///
    /// The errors are added to `self.errors`
    ///
    /// The following are valid:
    /// - one coords value (array of 2 or 3)
    /// - object with `at` property, an optionally `color`
    pub fn compile_marker(
        &mut self,
        prop_name: &str,
        prop: &RouteBlob,
    ) {
        if prop.is_array() {
            match self.parse_coord(prop) {
                Ok(coord) => {
                    let marker = CompMarker::at(coord);
                    self.line.markers.push(marker);
                }
                Err(e) => {
                    self.errors.push(e);
                }
            };
            return;
        }
        let mapping = match prop.as_object() {
            Some(mapping) => mapping,
            None => {
                self.errors.push(CompError::InvalidMarkerType);
                return;
            }
        };

        let mut at = None;
        let mut color = None;
        let mut should_fail = false;

        for (key, value) in mapping {
            match key.as_ref() {
                prop::AT => match self.parse_coord(value) {
                    Ok(coord) => at = Some(coord),
                    Err(e) => {
                        self.errors.push(e);
                        should_fail = true;
                    }
                },
                prop::COLOR => {
                    if value.is_array() || value.is_object() {
                        self.errors.push(CompError::InvalidLinePropertyType(format!(
                            "{prop_name}.{}",
                            prop::COLOR
                        )))
                    } else {
                        color = Some(value.coerce_to_string())
                    }
                }
                _ => self.errors.push(CompError::UnusedProperty(format!("{prop_name}.{key}"))),
            }
        }

        match at {
            None => {
                self.errors.push(CompError::InvalidMarkerType);
            }
            Some(at) => {
                if !should_fail {
                    let marker = CompMarker { at, color };
                    self.line.markers.push(marker);
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
        fn test_compile_marker(&self, prop_name: &str, prop: Value) {
            self.compile_marker(prop_name, &prop.into())
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
            ctx.test_compile_marker("", v);
            assert_eq!(ctx.errors, vec![CompError::InvalidMarkerType]);
        }
    }

    #[test]
    fn test_propagate_coord_error() {
        let ctx = LineContext::default();
        ctx.test_compile_marker("", json!([1, 2, 3, 4]));
        assert_eq!(ctx.errors, vec![CompError::InvalidCoordinateArray]);
    }

    #[test]
    fn test_valid_coord() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let ctx = LineContext {
            compiler: Cow::Borrowed(&compiler),
            ..Default::default()
        };
        ctx.test_compile_marker("", json!([1, 2, 4]));
        let marker = CompMarker::at(GameCoord(1.0, 2.0, 4.0));
        assert_eq!(ctx.line.markers, vec![marker]);
        assert_eq!(ctx.errors, vec![]);
    }

    #[test]
    fn test_object() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let mut errors = vec![];
        assert_eq!(
            compiler.comp_marker("", json!({"at": [1, 2, 4]}), &mut errors),
            Some(CompMarker::at(GameCoord(1.0, 2.0, 4.0)))
        );
        assert_eq!(errors, vec![]);

        assert_eq!(
            compiler.comp_marker("", json!({"at": [1, 2, 4], "color": 123}), &mut errors),
            Some(CompMarker {
                at: GameCoord(1.0, 2.0, 4.0),
                color: Some("123".to_string())
            })
        );
        assert_eq!(errors, vec![]);

        assert_eq!(
            compiler.comp_marker("test", json!({"at": {}, "color": {}}), &mut errors),
            None
        );
        assert_eq!(
            errors,
            vec![
                CompError::InvalidCoordinateType("[object object]".to_string()),
                CompError::InvalidLinePropertyType("test.color".to_string()),
                CompError::InvalidMarkerType
            ]
        );
    }

    #[test]
    fn test_unused_property() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let mut errors = vec![];
        assert_eq!(
            compiler.comp_marker(
                "test",
                json!({
                    "at": [1, 2, 4],
                    "unused": 1,
                }),
                &mut errors
            ),
            Some(CompMarker::at(GameCoord(1.0, 2.0, 4.0)))
        );
        assert_eq!(
            errors,
            vec![CompError::UnusedProperty("test.unused".to_string())]
        );
    }
}