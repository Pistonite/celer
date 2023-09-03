use celerctypes::GameCoord;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    json::{Cast, Coerce},
    util::async_for,
};

use super::{prop, Compiler, CompilerError};

#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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

impl Compiler {
    /// Try compiling a json blob to a marker object
    ///
    /// The following are valid:
    /// - one coords value (array of 2 or 3)
    /// - object with `at` property, an optionally `color`
    pub async fn comp_marker(
        &self,
        prop_name: &str,
        prop: Value,
        errors: &mut Vec<CompilerError>,
    ) -> Option<CompMarker> {
        if prop.is_array() {
            return match self.transform_coord(prop) {
                Ok(coord) => Some(CompMarker::at(coord)),
                Err(e) => {
                    errors.push(e);
                    None
                }
            };
        }
        let mapping = prop.try_into_object().ok().or_else(|| {
            errors.push(CompilerError::InvalidMarkerType);
            None
        })?;

        let mut at = None;
        let mut color = None;
        let mut should_fail = false;

        async_for!((key, value) in mapping, {
            match key.as_ref() {
                prop::AT => match self.transform_coord(value) {
                    Ok(coord) => at = Some(coord),
                    Err(e) => {
                        errors.push(e);
                        should_fail = true;
                    }
                },
                prop::COLOR => {
                    if value.is_array() || value.is_object() {
                        errors.push(CompilerError::InvalidLinePropertyType(format!(
                            "{prop_name}.{}", prop::COLOR
                        )))
                    } else {
                        color = Some(value.coerce_to_string())
                    }
                }
                _ => errors.push(CompilerError::UnusedProperty(format!("{prop_name}.{key}"))),
            }
        });

        match at {
            None => {
                errors.push(CompilerError::InvalidMarkerType);
                None
            }
            Some(at) => {
                if should_fail {
                    None
                } else {
                    Some(CompMarker { at, color })
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::comp::test_utils;

    use super::*;

    #[tokio::test]
    async fn test_value_invalid() {
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
            assert_eq!(compiler.comp_marker("", v, &mut errors).await, None,);
            assert_eq!(errors, vec![CompilerError::InvalidMarkerType]);
        }
    }

    #[tokio::test]
    async fn test_propagate_coord_error() {
        let compiler = Compiler::default();
        let mut errors = vec![];
        assert_eq!(
            compiler
                .comp_marker("", json!([1, 2, 3, 4]), &mut errors)
                .await,
            None
        );
        assert_eq!(errors, vec![CompilerError::InvalidCoordinateArray]);
    }

    #[tokio::test]
    async fn test_valid_coord() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let mut errors = vec![];
        assert_eq!(
            compiler
                .comp_marker("", json!([1, 2, 4]), &mut errors)
                .await,
            Some(CompMarker::at(GameCoord(1.0, 2.0, 4.0)))
        );
        assert_eq!(errors, vec![]);
    }

    #[tokio::test]
    async fn test_object() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let mut errors = vec![];
        assert_eq!(
            compiler
                .comp_marker("", json!({"at": [1, 2, 4]}), &mut errors)
                .await,
            Some(CompMarker::at(GameCoord(1.0, 2.0, 4.0)))
        );
        assert_eq!(errors, vec![]);

        assert_eq!(
            compiler
                .comp_marker("", json!({"at": [1, 2, 4], "color": 123}), &mut errors)
                .await,
            Some(CompMarker {
                at: GameCoord(1.0, 2.0, 4.0),
                color: Some("123".to_string())
            })
        );
        assert_eq!(errors, vec![]);

        assert_eq!(
            compiler
                .comp_marker("test", json!({"at": {}, "color": {}}), &mut errors)
                .await,
            None
        );
        assert_eq!(
            errors,
            vec![
                CompilerError::InvalidCoordinateType("[object object]".to_string()),
                CompilerError::InvalidLinePropertyType("test.color".to_string()),
                CompilerError::InvalidMarkerType
            ]
        );
    }

    #[tokio::test]
    async fn test_unused_property() {
        let compiler = test_utils::create_test_compiler_with_coord_transform();
        let mut errors = vec![];
        assert_eq!(
            compiler
                .comp_marker(
                    "test",
                    json!({
                        "at": [1, 2, 4],
                        "unused": 1,
                    }),
                    &mut errors
                )
                .await,
            Some(CompMarker::at(GameCoord(1.0, 2.0, 4.0)))
        );
        assert_eq!(
            errors,
            vec![CompilerError::UnusedProperty("test.unused".to_string())]
        );
    }
}
