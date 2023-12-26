//!
//!

use crate::json::{IntoSafeRouteBlob, RouteBlob};

use crate::pack::Compiler;

use super::{CompLine, CompResult, LineContext};

impl<'p> Compiler<'p> {
    /// Parse the line (parallel pass)
    pub fn parse_line(&self, value: &'p RouteBlob) -> CompLine {
        let mut ctx = self.create_line_context();
        let value = match value.checked() {
            Ok(value) => {
                ctx.parse_line(value);
            }
            Err(e) => {
                ctx.errors.push(e.into());
            }
        };
        for error in ctx.errors {
            error.add_to_diagnostics(&mut ctx.line.diagnostics);
        }

        ctx.line
    }

    fn create_line_context(&self) -> LineContext<'_, 'p> {
        let line = CompLine {
            map_icon_priority: self.meta.default_icon_priority,
            ..Default::default()
        };
        LineContext {
            compiler: self,
            line,
            errors: vec![],
        }
    }
}

impl CompLine {
    /// Execute the sequential pass of line compilation
    ///
    /// This updates the coordinate and color of the line and the compiler
    pub fn sequential_pass(&mut self, compiler: &mut Compiler<'_>) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use celerb::lang::DocRichTextBlock;
    use map_macro::btree_map;
    use serde_json::{json, Value};

    use crate::comp::line::DocNote;
    use crate::comp::test_utils::{self, CompilerBuilder};
    use crate::comp::{CompError, CompMarker, CompMovement};
    use crate::lang::{self, DocRichText, Preset};
    use crate::prep::{Axis, GameCoord, MapCoordMap, MapMetadata, RouteConfig, RouteMetadata};

    use super::*;

    fn test_comp_ok(compiler: &mut Compiler<'static>, input: Value, expected: CompLine) {
        let mut line = compiler.parse_line(&RouteBlob::Prim(input));
        line.sequential_pass(compiler);
        assert_eq!(line, expected);
    }

    fn test_comp_err(
        compiler: &mut Compiler<'static>,
        input: Value,
        mut expected: CompLine,
        errors: Vec<CompError>,
    ) {
        let mut line = compiler.parse_line(&RouteBlob::Prim(input));
        line.sequential_pass(compiler);
        for error in errors {
            error.add_to_diagnostics(&mut expected.diagnostics);
        }
        assert_eq!(line, expected);
    }

    #[test]
    fn test_primitive() {
        let mut compiler = Compiler::default();
        test_comp_ok(&mut compiler, json!(null), CompLine::default());

        test_comp_ok(
            &mut compiler,
            json!(true),
            CompLine {
                text: lang::parse_rich("true"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!(false),
            CompLine {
                text: lang::parse_rich("false"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!(0),
            CompLine {
                text: lang::parse_rich("0"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!(-123),
            CompLine {
                text: lang::parse_rich("-123"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!(456),
            CompLine {
                text: lang::parse_rich("456"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!("hello world"),
            CompLine {
                text: lang::parse_rich("hello world"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!(".tag(foo) world"),
            CompLine {
                text: lang::parse_rich(".tag(foo) world"),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_invalid() {
        let mut compiler = Compiler::default();

        test_comp_err(
            &mut compiler,
            json!([]),
            CompLine {
                text: DocRichText::text("[object array]"),
                ..Default::default()
            },
            vec![CompError::ArrayCannotBeLine],
        );

        test_comp_err(
            &mut compiler,
            json!({}),
            CompLine {
                text: DocRichText::text("[object object]"),
                ..Default::default()
            },
            vec![CompError::EmptyObjectCannotBeLine],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "one": {},
                "two": {},
            }),
            CompLine {
                text: DocRichText::text("[object object]"),
                ..Default::default()
            },
            vec![CompError::TooManyKeysInObjectLine],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "one": "not an object",
            }),
            CompLine {
                text: DocRichText::text("[object object]"),
                ..Default::default()
            },
            vec![CompError::LinePropertiesMustBeObject],
        );
    }

    #[test]
    fn test_text_overrides() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "text": "hello world",
                }
            }),
            CompLine {
                text: DocRichText::text("hello world"),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "text": ["hello world"],
                }
            }),
            CompLine {
                text: DocRichText::text("[object array]"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("text".to_string())],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "comment": "hello world",
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                secondary_text: DocRichText::text("hello world"),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "comment": ["hello world"],
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                secondary_text: DocRichText::text("[object array]"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("comment".to_string())],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "notes": "hello world",
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                notes: vec![DocNote::Text {
                    content: DocRichText::text("hello world"),
                }],
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "notes": ["hello world", "foo bar"],
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                notes: vec![
                    DocNote::Text {
                        content: DocRichText::text("hello world"),
                    },
                    DocNote::Text {
                        content: DocRichText::text("foo bar"),
                    },
                ],
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "notes": {},
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("notes".to_string())],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "notes": ["hello", {}],
                    "comment": {},
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                notes: vec![
                    DocNote::Text {
                        content: DocRichText::text("hello"),
                    },
                    DocNote::Text {
                        content: DocRichText::text("[object object]"),
                    },
                ],
                secondary_text: DocRichText::text("[object object]"),
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("comment".to_string()),
                CompError::InvalidLinePropertyType("notes[1]".to_string()),
            ],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "split-name": "test .v(boo)",
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                split_name: Some(DocRichText(vec![
                    DocRichTextBlock::text("test "),
                    DocRichTextBlock::with_tag("v", "boo"),
                ])),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                    "foo": {
                    "split-name": ["hello world"],
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("split-name".to_string())],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "split-name": "",
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                split_name: Some(DocRichText(vec![])),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_preset_one_level() {
        let mut builder = CompilerBuilder::default();
        builder
            .add_preset(
                "_preset",
                Preset::compile(json!({
                    "text": "hello world",
                    "comment": "foo bar",
                }))
                .unwrap(),
            )
            .add_preset(
                "_notext",
                Preset::compile(json!({
                    "comment": "foo bar",
                }))
                .unwrap(),
            );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!("_preset"),
            CompLine {
                text: DocRichText::text("hello world"),
                secondary_text: DocRichText::text("foo bar"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_preset": {
                    "comment": "foo bar 2",
                }
            }),
            CompLine {
                text: DocRichText::text("hello world"),
                secondary_text: DocRichText::text("foo bar 2"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_notext": {
                    "comment": "foo bar 2",
                }
            }),
            CompLine {
                text: DocRichText::text("_notext"),
                secondary_text: DocRichText::text("foo bar 2"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_notext": {
                    "text": "foo bar 2",
                }
            }),
            CompLine {
                text: DocRichText::text("foo bar 2"),
                secondary_text: DocRichText::text("foo bar"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_invalid": {
                    "comment": "foo bar 2",
                }
            }),
            CompLine {
                text: DocRichText::text("_invalid"),
                secondary_text: DocRichText::text("foo bar 2"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "text": "_preset",
                }
            }),
            CompLine {
                text: DocRichText::text("_preset"),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_preset_nested() {
        let mut builder = CompilerBuilder::default();
        builder
            .add_preset(
                "_preset::one",
                Preset::compile(json!({
                    "comment": "preset one",
                }))
                .unwrap(),
            )
            .add_preset(
                "_preset::two",
                Preset::compile(json!({
                    "comment": "preset two",
                    "text": "preset two text",
                }))
                .unwrap(),
            )
            .add_preset(
                "_preset::three",
                Preset::compile(json!({
                    "text": "preset three",
                    "presets": ["_preset::two"]
                }))
                .unwrap(),
            )
            .add_preset(
                "_preset::four",
                Preset::compile(json!({
                    "text": "preset four: arg is $(0)",
                    "presets": ["_preset::one", "_preset::three"]
                }))
                .unwrap(),
            )
            .add_preset(
                "_preset::overflow",
                Preset::compile(json!({
                    "presets": ["_preset::overflow"]
                }))
                .unwrap(),
            );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "_preset::one": {
                    "presets": ["_preset::two"],
                }
            }),
            CompLine {
                text: DocRichText::text("preset two text"),
                secondary_text: DocRichText::text("preset one"),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "test": {
                    "presets": "foo",
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                ..Default::default()
            },
            vec![CompError::InvalidPresetString("foo".to_string())],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "test": {
                    "presets": [{}, "foo", "_foo", "_hello::", 123],
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("presets[0]".to_string()),
                CompError::InvalidPresetString("foo".to_string()),
                CompError::PresetNotFound("_foo".to_string()),
                CompError::InvalidPresetString("_hello::".to_string()),
                CompError::InvalidPresetString("123".to_string()),
            ],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_preset::three<1>": {
                    "presets": ["_preset::one"],
                }
            }),
            CompLine {
                text: DocRichText::text("preset three"),
                secondary_text: DocRichText::text("preset two"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!("_preset::four< abcde >"),
            CompLine {
                text: DocRichText::text("preset four: arg is  abcde "),
                secondary_text: DocRichText::text("preset two"),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!("_preset::overflow"),
            CompLine {
                text: DocRichText::text("_preset::overflow"),
                ..Default::default()
            },
            vec![CompError::MaxPresetDepthExceeded(
                "_preset::overflow".to_string(),
            )],
        );
    }

    #[test]
    fn test_icon_overrides() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "icon is string": {
                    "icon": "my-icon",
                },
            }),
            CompLine {
                text: DocRichText::text("icon is string"),
                doc_icon: Some("my-icon".to_string()),
                map_icon: Some("my-icon".to_string()),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "icon is string": {
                    "icon": ["my-icon"],
                },
            }),
            CompLine {
                text: DocRichText::text("icon is string"),
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("icon-doc".to_string()),
                CompError::InvalidLinePropertyType("icon-map".to_string()),
            ],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "icon is array": {
                    "icon": ["my-icon"],
                },
            }),
            CompLine {
                text: DocRichText::text("icon is array"),
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("icon-doc".to_string()),
                CompError::InvalidLinePropertyType("icon-map".to_string()),
            ],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "icon is empty object": {
                    "icon": {}
                },
            }),
            CompLine {
                text: DocRichText::text("icon is empty object"),
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("icon-doc".to_string()),
                CompError::InvalidLinePropertyType("icon-map".to_string()),
            ],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "icon all 3": {
                    "icon-doc": "my-doc-icon",
                    "icon-map": "my-map-icon",
                    "icon-priority": "1",
                },
            }),
            CompLine {
                text: DocRichText::text("icon all 3"),
                doc_icon: Some("my-doc-icon".to_string()),
                map_icon: Some("my-map-icon".to_string()),
                map_icon_priority: 1,
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "icon is object": {
                    "icon-doc":{},
                    "icon-map": ["my-map-icon"],
                    "icon-priority": 1.2,
                    "icon-boo": "foo",
                },
            }),
            CompLine {
                text: DocRichText::text("icon is object"),
                properties: btree_map! {
                    "icon-boo".to_string() => json!("foo"),
                }
                .into(),
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("icon-doc".to_string()),
                CompError::InvalidLinePropertyType("icon-map".to_string()),
                CompError::InvalidLinePropertyType("icon-priority".to_string()),
            ],
        );
    }

    #[test]
    fn test_default_icon_priority() {
        let mut builder = CompilerBuilder::default();
        builder.set_default_icon_priority(10);
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "icon is partial": {
                    "icon-map": "my-map-icon",
                },
            }),
            CompLine {
                text: DocRichText::text("icon is partial"),
                doc_icon: None,
                map_icon: Some("my-map-icon".to_string()),
                map_icon_priority: 10,
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_icon_hide() {
        let mut builder = CompilerBuilder::default();
        builder.add_preset(
            "_Example",
            Preset::compile(json!({
                "icon-doc": "my-doc-icon",
                "icon-map": "my-map-icon",
            }))
            .unwrap(),
        );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "_Example": {
                    "icon-map": null,
                },
            }),
            CompLine {
                text: DocRichText::text("_Example"),
                map_icon: None,
                doc_icon: Some("my-doc-icon".to_string()),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_Example": {
                    "icon-doc": false,
                },
            }),
            CompLine {
                text: DocRichText::text("_Example"),
                doc_icon: None,
                map_icon: Some("my-map-icon".to_string()),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_counter_override() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is string": {
                    "counter": "hello",
                },
            }),
            CompLine {
                text: DocRichText::text("counter is string"),
                counter_text: Some(DocRichTextBlock::text("hello")),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is tagged string": {
                    "counter": ".test(hello)",
                },
            }),
            CompLine {
                text: DocRichText::text("counter is tagged string"),
                counter_text: Some(DocRichTextBlock::with_tag("test", "hello")),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is empty tagged string": {
                    "counter": ".test()",
                },
            }),
            CompLine {
                text: DocRichText::text("counter is empty tagged string"),
                counter_text: Some(DocRichTextBlock::with_tag("test", "")),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is empty string": {
                    "counter": "",
                },
            }),
            CompLine {
                text: DocRichText::text("counter is empty string"),
                counter_text: None,
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "counter is invalid": {
                    "counter": ["hello"],
                },
            }),
            CompLine {
                text: DocRichText::text("counter is invalid"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("counter".to_string())],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "counter is more than one text block": {
                    "counter": ".v(hello) foo",
                },
            }),
            CompLine {
                text: DocRichText::text("counter is more than one text block"),
                counter_text: Some(DocRichTextBlock::with_tag("v", "hello")),
                ..Default::default()
            },
            vec![CompError::TooManyTagsInCounter],
        );
    }

    #[test]
    fn test_inherit_color_coord() {
        let builder = CompilerBuilder::new(
            Default::default(),
            "color".to_string(),
            GameCoord(1.0, 2.0, 3.0),
        );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!("no color or coord"),
            CompLine {
                text: DocRichText::text("no color or coord"),
                line_color: "color".to_string(),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_change_color() {
        let builder = CompilerBuilder::new(
            Default::default(),
            "color".to_string(),
            GameCoord(1.0, 2.0, 3.0),
        );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "change color": {
                    "color": "new-color",
                }
            }),
            CompLine {
                text: DocRichText::text("change color"),
                line_color: "new-color".to_string(),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "change color 2": {
                    "color": ["newer-color"],
                }
            }),
            CompLine {
                text: DocRichText::text("change color 2"),
                line_color: "new-color".to_string(),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("color".to_string())],
        );
    }

    #[test]
    fn test_change_coord() {
        let builder = CompilerBuilder::new(
            RouteConfig {
                map: Some(MapMetadata {
                    coord_map: MapCoordMap {
                        mapping_3d: (Axis::X, Axis::Y, Axis::Z),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            },
            "".to_string(),
            GameCoord(1.0, 2.0, 3.0),
        );
        let mut compiler = builder.clone().build();

        test_comp_ok(
            &mut compiler,
            json!({
                "change coord": {
                    "coord": [4.0, 5.0, 6.0],
                }
            }),
            CompLine {
                text: DocRichText::text("change coord"),
                map_coord: GameCoord(4.0, 5.0, 6.0),
                movements: vec![CompMovement::to(GameCoord(4.0, 5.0, 6.0))],
                ..Default::default()
            },
        );
        assert_eq!(compiler.coord, GameCoord(4.0, 5.0, 6.0));

        let mut compiler = builder.clone().build();
        test_comp_ok(
            &mut compiler,
            json!({
                "push pop": {
                    "movements": [
                        "push",
                        [4.0, 5.0, 6.0],
                        "pop",
                    ]
                }
            }),
            CompLine {
                text: DocRichText::text("push pop"),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                movements: vec![
                    CompMovement::Push,
                    CompMovement::to(GameCoord(4.0, 5.0, 6.0)),
                    CompMovement::Pop,
                ],
                ..Default::default()
            },
        );

        let mut compiler = builder.build();
        test_comp_err(
            &mut compiler,
            json!({
                "invalid": {
                    "movements": {}
                }
            }),
            CompLine {
                text: DocRichText::text("invalid"),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("movements".to_string())],
        );
    }

    #[test]
    fn test_movements_preset() {
        let mut builder = CompilerBuilder::new(
            RouteConfig {
                map: Some(MapMetadata {
                    coord_map: MapCoordMap {
                        mapping_3d: (Axis::X, Axis::Y, Axis::Z),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..Default::default()
            },
            "".to_string(),
            GameCoord(1.0, 2.0, 3.0),
        );
        builder.add_preset(
            "_preset::one",
            Preset::compile(json!({
                "movements": [
                    [7, "8", 9],
                    [7, "8", 9],
                ]
            }))
            .unwrap(),
        );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "preset": {
                    "movements": [
                        [3, 4, 5],
                        "_preset::one",
                    ]
                }
            }),
            CompLine {
                text: DocRichText::text("preset"),
                map_coord: GameCoord(7.0, 8.0, 9.0),
                movements: vec![
                    CompMovement::to(GameCoord(3.0, 4.0, 5.0)),
                    CompMovement::to(GameCoord(7.0, 8.0, 9.0)),
                    CompMovement::to(GameCoord(7.0, 8.0, 9.0)),
                ],
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_markers() {
        let mut compiler = test_utils::create_test_compiler_with_coord_transform();

        test_comp_ok(
            &mut compiler,
            json!({
                "test markers": {
                    "markers": [
                        {"at": [1, 2, 4], "color": "marker 1"},
                        [1, "2", 3]
                    ]
                }
            }),
            CompLine {
                text: DocRichText::text("test markers"),
                markers: vec![
                    CompMarker {
                        at: GameCoord(1.0, 2.0, 4.0),
                        color: Some("marker 1".to_string()),
                    },
                    CompMarker::at(GameCoord(1.0, 2.0, 3.0)),
                ],
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "test markers invalid type": {
                    "markers": {}
                }
            }),
            CompLine {
                text: DocRichText::text("test markers invalid type"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("markers".to_string())],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "test markers invalid marker type": {
                    "markers": [
                        "hello"
                    ]
                }
            }),
            CompLine {
                text: DocRichText::text("test markers invalid marker type"),
                ..Default::default()
            },
            vec![CompError::InvalidMarkerType],
        );
    }

    #[test]
    fn test_unused_properties() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "unused": "property"
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                properties: btree_map! {
                    "unused".to_string() => json!("property"),
                }
                .into(),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_banner() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "banner": "true"
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                is_banner: true,
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "banner": true
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                is_banner: true,
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "banner": false
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                is_banner: false,
                ..Default::default()
            },
        );
    }
}
