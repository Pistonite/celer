use crate::comp::{CompLine, CompMovement, DocNote};
use crate::lang::{DocDiagnostic, DocRichText, DocRichTextBlock, IntoDiagnostic};
use crate::macros::derive_wasm;
use crate::prep::{GameCoord, RouteConfig};

use super::{ExecError, MapBuilder, MapIcon, MapMarker};

/// A line in the executed document
#[derive(PartialEq, Default, Debug, Clone)]
#[derive_wasm]
pub struct ExecLine {
    /// Section number
    pub section: usize,
    /// Line index in section
    pub index: usize,
    /// Primary text content of the line
    pub text: DocRichText,
    /// Line color
    pub line_color: String,
    /// Corresponding map coordinates
    pub map_coords: Vec<GameCoord>,
    /// Diagnostic messages
    pub diagnostics: Vec<DocDiagnostic>,
    /// The icon id to show on the document
    pub icon: Option<String>,
    /// Secondary text to show below the primary text
    pub secondary_text: DocRichText,
    /// Counter text to display
    pub counter_text: Option<DocRichTextBlock>,
    /// The notes
    pub notes: Vec<DocNote>,
    /// If the line text is a banner
    pub is_banner: bool,
}

impl CompLine {
    /// Execute the line.
    ///
    /// Map features will be added to the builder
    pub fn exec(
        mut self,
        project: &RouteConfig,
        section_number: usize,
        line_number: usize,
        map_builder: &mut MapBuilder,
    ) -> ExecLine {
        let line_color = match self.line_color {
            Some(color) => {
                map_builder.change_color(color.clone());
                color
            }
            None => map_builder.color().to_string(),
        };
        // trace coordinates
        let mut map_coords = vec![];
        for movement in self.movements {
            match movement {
                CompMovement::To {
                    to,
                    warp,
                    exclude,
                    color,
                    icon,
                } => {
                    // update builder properties
                    if let Some(color) = color {
                        map_builder.change_color(color);
                    }
                    if !exclude {
                        map_coords.push(to.clone());
                    }
                    // perform the movement
                    if warp {
                        map_builder.warp_to(to);
                    } else {
                        map_builder.move_to(to);
                    }
                    // add the icon at the end of the movement
                    if let Some(icon) = icon {
                        if !project.icons.contains_key(&icon) {
                            self.diagnostics
                                .push(ExecError::IconNotFound(icon).into_diagnostic())
                        } else {
                            map_builder.icons.push(MapIcon {
                                id: icon,
                                coord: map_builder.coord().clone(),
                                line_index: line_number,
                                section_index: section_number,
                                priority: self.map_icon_priority,
                            });
                        }
                    }
                }
                CompMovement::Push => {
                    map_builder.push();
                }
                CompMovement::Pop => {
                    map_builder.pop();
                }
            }
        }
        // change color back if it was changed in movements temporarily
        map_builder.change_color(line_color.clone());

        // add the main icon to map
        if let Some(icon) = self.map_icon {
            if !project.icons.contains_key(&icon) {
                self.diagnostics
                    .push(ExecError::IconNotFound(icon).into_diagnostic())
            } else {
                map_builder.icons.push(MapIcon {
                    id: icon,
                    coord: map_builder.coord().clone(),
                    line_index: line_number,
                    section_index: section_number,
                    priority: self.map_icon_priority,
                });
            }
        }

        // add markers
        for marker in self.markers {
            map_builder.markers.push(MapMarker {
                coord: marker.at,
                line_index: line_number,
                section_index: section_number,
                color: marker.color.unwrap_or_else(|| line_color.clone()),
            });
        }

        ExecLine {
            is_banner: self.is_banner,
            section: section_number,
            index: line_number,
            text: self.text,
            line_color,
            diagnostics: self.diagnostics,
            icon: self.doc_icon,
            secondary_text: self.secondary_text,
            counter_text: self.counter_text,
            notes: self.notes,
            map_coords,
        }
    }
}

#[cfg(test)]
mod test {
    use map_macro::btree_map;

    use crate::comp::{CompMarker, CompMovement, DocNote};
    use crate::exec::MapLine;
    use crate::lang::{
        DocDiagnostic, DocPoorText, DocPoorTextBlock, DocRichText, DocRichTextBlock,
    };
    use crate::prep::GameCoord;

    use super::*;

    fn create_test_movements() -> Vec<CompMovement> {
        vec![
            CompMovement::Push,
            CompMovement::To {
                to: GameCoord(3.4, 5.0, 6.0),
                warp: false,
                exclude: false,
                color: Some("blue".to_string()),
                icon: None,
            },
            CompMovement::To {
                to: GameCoord(3.4, 7.0, 6.0),
                warp: false,
                exclude: false,
                color: Some("red".to_string()),
                icon: Some("test icon 1".to_string()),
            },
            CompMovement::Pop,
            CompMovement::Push,
            CompMovement::To {
                to: GameCoord(3.5, 5.0, 6.1),
                warp: false,
                exclude: false,
                color: Some("blue".to_string()),
                icon: None,
            },
            CompMovement::To {
                to: GameCoord(3.5, 7.4, 6.2),
                warp: false,
                exclude: true,
                color: Some("red".to_string()),
                icon: Some("test icon 2".to_string()),
            },
            CompMovement::Pop,
            CompMovement::to(GameCoord(1.2, 55.0, 37.8)),
            CompMovement::To {
                to: GameCoord(100.2, -3.0, 7.8),
                warp: true,
                exclude: false,
                color: None,
                icon: None,
            },
            CompMovement::to(GameCoord(1.2, 55.0, 37.8)),
        ]
    }

    #[test]
    fn test_copy() {
        let mut map_section = Default::default();
        let test_text = DocRichText(vec![
            DocRichTextBlock {
                tag: None,
                text: "test1".to_string(),
                link: None,
            },
            DocRichTextBlock {
                tag: Some("test tag".to_string()),
                text: "test2".to_string(),
                link: Some("test link".to_string()),
            },
        ]);
        let test_color = "test color".to_string();
        let test_diagnostics = vec![DocDiagnostic {
            msg: DocPoorText(vec![
                DocPoorTextBlock::Text("test msg1".to_string()),
                DocPoorTextBlock::Link("test link".to_string()),
            ]),
            msg_type: "test msg type".into(),
            source: "test msg source".into(),
        }];
        let test_doc_icon = Some("test-icon".to_string());
        let test_secondary_text = DocRichText(vec![
            DocRichTextBlock {
                tag: None,
                text: "secondary test1".to_string(),
                link: None,
            },
            DocRichTextBlock {
                tag: Some("secondary test tag".to_string()),
                text: "secondary test2".to_string(),
                link: Some("secondary test link".to_string()),
            },
        ]);
        let test_counter_text = Some(DocRichTextBlock {
            tag: Some("counter test tag".to_string()),
            text: "counter test".to_string(),
            link: None,
        });
        let test_notes = vec![
            DocNote::Text {
                content: DocRichText(vec![
                    DocRichTextBlock {
                        tag: None,
                        text: "note test1".to_string(),
                        link: None,
                    },
                    DocRichTextBlock {
                        tag: Some("note test tag".to_string()),
                        text: "note test2".to_string(),
                        link: Some("note test link".to_string()),
                    },
                ]),
            },
            DocNote::Image {
                link: "note test src image".to_string(),
            },
            DocNote::Video {
                link: "note test src video".to_string(),
            },
        ];

        let line = CompLine {
            text: test_text.clone(),
            line_color: Some(test_color.clone()),
            diagnostics: test_diagnostics.clone(),
            doc_icon: test_doc_icon.clone(),
            secondary_text: test_secondary_text.clone(),
            counter_text: test_counter_text.clone(),
            notes: test_notes.clone(),
            is_banner: true,
            ..Default::default()
        };
        let project = RouteConfig {
            icons: btree_map! {
                "test-icon".to_string() => "".to_string(),
            }
            .into(),
            ..Default::default()
        };
        let exec_line = line.exec(&project, 3, 4, &mut map_section);
        assert_eq!(exec_line.section, 3);
        assert_eq!(exec_line.index, 4);
        assert_eq!(exec_line.text, test_text);
        assert_eq!(exec_line.line_color, test_color);
        assert_eq!(exec_line.diagnostics, test_diagnostics);
        assert_eq!(exec_line.icon, test_doc_icon);
        assert_eq!(exec_line.secondary_text, test_secondary_text);
        assert_eq!(exec_line.counter_text, test_counter_text);
        assert_eq!(exec_line.notes, test_notes);
        assert!(exec_line.is_banner);
    }

    #[test]
    fn test_map_coords() {
        let test_line = CompLine {
            movements: create_test_movements(),
            ..Default::default()
        };
        let mut map_section = Default::default();
        let exec_line = test_line.exec(&Default::default(), 0, 0, &mut map_section);
        let expected = vec![
            GameCoord(3.4, 5.0, 6.0),
            GameCoord(3.4, 7.0, 6.0),
            GameCoord(3.5, 5.0, 6.1),
            GameCoord(1.2, 55.0, 37.8),
            GameCoord(100.2, -3.0, 7.8),
            GameCoord(1.2, 55.0, 37.8),
        ];
        assert_eq!(exec_line.map_coords, expected);
    }

    #[test]
    fn test_add_map_icon_and_markers() {
        let test_line = CompLine {
            line_color: Some("test color".to_string()),
            map_icon: Some("test icon".to_string()),
            map_icon_priority: 3,
            markers: vec![
                CompMarker::at(GameCoord(1.2, 55.0, 37.8)),
                CompMarker {
                    at: GameCoord(1.2, 85.0, 37.8),
                    color: Some("test marker override".to_string()),
                },
            ],
            movements: create_test_movements(),
            ..Default::default()
        };
        let mut builder = Default::default();
        let project = RouteConfig {
            icons: btree_map! {
                "test icon".to_string() =>  "".to_string(),
                "test icon 1".to_string() => "".to_string(),
                "test icon 2".to_string() => "".to_string(),
            }
            .into(),
            ..Default::default()
        };
        test_line.exec(&project, 4, 5, &mut builder);
        assert_eq!(
            builder.icons,
            vec![
                MapIcon {
                    id: "test icon 1".to_string(),
                    coord: GameCoord(3.4, 7.0, 6.0),
                    line_index: 5,
                    section_index: 4,
                    priority: 3,
                },
                MapIcon {
                    id: "test icon 2".to_string(),
                    coord: GameCoord(3.5, 7.4, 6.2),
                    line_index: 5,
                    section_index: 4,
                    priority: 3,
                },
                MapIcon {
                    id: "test icon".to_string(),
                    coord: GameCoord(1.2, 55.0, 37.8),
                    line_index: 5,
                    section_index: 4,
                    priority: 3,
                },
            ]
        );
        assert_eq!(
            builder.markers,
            vec![
                MapMarker {
                    coord: GameCoord(1.2, 55.0, 37.8),
                    color: "test color".to_string(),
                    line_index: 5,
                    section_index: 4,
                },
                MapMarker {
                    coord: GameCoord(1.2, 85.0, 37.8),
                    color: "test marker override".to_string(),
                    line_index: 5,
                    section_index: 4,
                }
            ]
        );
    }

    #[test]
    fn test_map_lines() {
        let test_line = CompLine {
            line_color: Some("test color".to_string()),
            movements: create_test_movements(),
            ..Default::default()
        };
        let mut map_builder = MapBuilder::new("blue".to_string(), GameCoord::default());
        test_line.exec(&Default::default(), 0, 0, &mut map_builder);
        let map_section = map_builder.build_section();
        assert_eq!(
            map_section.lines,
            vec![
                MapLine {
                    color: "blue".to_string(),
                    points: vec![GameCoord::default(), GameCoord(3.4, 5.0, 6.0)],
                },
                MapLine {
                    color: "red".to_string(),
                    points: vec![GameCoord(3.4, 5.0, 6.0), GameCoord(3.4, 7.0, 6.0)],
                },
                MapLine {
                    color: "blue".to_string(),
                    points: vec![GameCoord::default(), GameCoord(3.5, 5.0, 6.1)],
                },
                MapLine {
                    color: "red".to_string(),
                    points: vec![GameCoord(3.5, 5.0, 6.1), GameCoord(3.5, 7.4, 6.2)],
                },
                MapLine {
                    color: "test color".to_string(),
                    points: vec![GameCoord::default(), GameCoord(1.2, 55.0, 37.8),],
                },
                MapLine {
                    color: "test color".to_string(),
                    points: vec![GameCoord(100.2, -3.0, 7.8), GameCoord(1.2, 55.0, 37.8)],
                },
            ]
        );
    }

    #[test]
    fn test_change_color_no_movement() {
        let test_line = CompLine {
            line_color: Some("test color".to_string()),
            ..Default::default()
        };
        let mut map_builder = MapBuilder::new("blue".to_string(), GameCoord::default());
        map_builder.move_to(GameCoord::default());
        test_line.exec(&Default::default(), 0, 0, &mut map_builder);

        map_builder.move_to(GameCoord::default());
        let map = map_builder.build_section();
        assert_eq!(
            map.lines,
            vec![
                MapLine {
                    color: "blue".to_string(),
                    points: vec![GameCoord::default(), GameCoord::default()],
                },
                MapLine {
                    color: "test color".to_string(),
                    points: vec![GameCoord::default(), GameCoord::default()],
                },
            ]
        );
    }

    #[test]
    fn test_missing_icons() {
        let test_line = CompLine {
            map_icon: Some("test icon".to_string()),
            movements: create_test_movements(),
            ..Default::default()
        };
        let mut builder = Default::default();
        let exec_line = test_line.exec(&Default::default(), 4, 5, &mut builder);
        assert_eq!(exec_line.diagnostics.len(), 3);
        assert_eq!(exec_line.diagnostics[0].msg_type, "warning");
        assert_eq!(exec_line.diagnostics[0].source, "celerc/exec");
        assert_eq!(exec_line.diagnostics[1].msg_type, "warning");
        assert_eq!(exec_line.diagnostics[1].source, "celerc/exec");
        assert_eq!(exec_line.diagnostics[2].msg_type, "warning");
        assert_eq!(exec_line.diagnostics[2].source, "celerc/exec");
    }
}
