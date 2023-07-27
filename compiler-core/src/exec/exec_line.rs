use celerctypes::{ExecLine, MapIcon, MapMarker};
use tokio_stream::StreamExt;

use crate::CompLine;

use super::MapSectionBuilder;

impl CompLine {
    /// Execute the line.
    ///
    /// Map features will be added to the builder
    pub async fn exec(
        self, 
        section_number: usize, 
        line_number: usize, 
        map_builder: &mut MapSectionBuilder
    ) -> ExecLine {
        if let Some(icon) = self.map_icon {
            map_builder.icons.push(MapIcon {
                id: icon,
                coord: self.map_coord,
                line_index: line_number,
                section_index: section_number,
                priority: self.map_icon_priority,
            })
        }

        let mut marker_iter = tokio_stream::iter(self.markers);
        while let Some(marker) = marker_iter.next().await {
            map_builder.markers.push(MapMarker {
                coord: marker.at,
                line_index: line_number,
                section_index: section_number,
                color: marker.color,
            });
        }

        let mut map_coords = vec![];
        for other_movement in self.other_movements {
            map_builder.add_other_movement(&other_movement).await;
            let mut m_iter = tokio_stream::iter(other_movement);
            while let Some(movement) = m_iter.next().await {
                map_coords.push(movement.movement.to.clone());
            }
        }
        for movement in self.movements {
            if movement.warp {
                map_builder.commit_line(false).await;
            }
            map_builder.add_main_movement(&self.line_color, &movement.to).await;
            map_coords.push(movement.to.clone());
        }
        ExecLine {
            section: section_number,
            index: line_number,
            text: self.text,
            line_color: self.line_color,
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
mod ut {
    use celerctypes::{DocRichText, DocDiagnostic, DocNote, GameCoord, MapLine};
    use crate::{CompMovement, CompMovementWithColor, CompMarker};

    use super::*;

    fn create_test_movements() -> Vec<CompMovement> {
        vec![
            CompMovement { to: GameCoord(1.2, 55.0, 37.8), warp: false },
            CompMovement { to: GameCoord(100.2, -3.0, 7.8), warp: true },
            CompMovement { to: GameCoord(1.2, 55.0, 37.8), warp: false },
        ]
    }
    fn create_test_other_movements() -> Vec<Vec<CompMovementWithColor>> {
        vec![
            vec![
                CompMovementWithColor { color: "blue".to_string(), movement: CompMovement { to: GameCoord(3.4, 5.0, 6.0), warp: false } },
                CompMovementWithColor { color: "red".to_string(), movement: CompMovement { to: GameCoord(3.4, 7.0, 6.0), warp: false } },
            ],
            vec![
                CompMovementWithColor { color: "blue".to_string(), movement: CompMovement { to: GameCoord(3.5, 5.0, 6.1), warp: false } },
                CompMovementWithColor { color: "red".to_string(), movement: CompMovement { to: GameCoord(3.5, 7.4, 6.2), warp: false } },
            ],
        ]
    }

    #[tokio::test]
    async fn test_copy() {
        let mut map_section = Default::default();
        let test_text = vec![DocRichText {
            tag: None,
            text: "test1".to_string(),
        },DocRichText {
                tag: Some("test tag".to_string()),
                text: "test2".to_string(),
            }];
        let test_color = "test color".to_string();
        let test_diagnostics = vec![
            DocDiagnostic {
                msg: "test msg".to_string(),
                msg_type: "test msg type".to_string(),
                source: "test msg source".to_string(),
            }
        ];
        let test_doc_icon = Some("test-icon".to_string());
        let test_secondary_text = vec![DocRichText {
            tag: None,
            text: "secondary test1".to_string(),
        },DocRichText {
                tag: Some("secondary test tag".to_string()),
                text: "secondary test2".to_string(),
            }];
        let test_counter_text = Some(DocRichText {
            tag: Some("counter test tag".to_string()),
            text: "counter test".to_string(),
        });
        let test_notes = vec![
            DocNote::Text {
                content: vec![DocRichText {
                    tag: None,
                    text: "note test1".to_string(),
                },DocRichText {
                        tag: Some("note test tag".to_string()),
                        text: "note test2".to_string(),
                    }],
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
            line_color: test_color.clone(),
            diagnostics: test_diagnostics.clone(),
            doc_icon: test_doc_icon.clone(),
            secondary_text: test_secondary_text.clone(),
            counter_text: test_counter_text.clone(),
            notes: test_notes.clone(),
            ..Default::default()
        };
        let exec_line = line.exec(3, 4, &mut map_section).await;
        assert_eq!(exec_line.section, 3);
        assert_eq!(exec_line.index, 4);
        assert_eq!(exec_line.text, test_text);
        assert_eq!(exec_line.line_color, test_color);
        assert_eq!(exec_line.diagnostics, test_diagnostics);
        assert_eq!(exec_line.icon, test_doc_icon);
        assert_eq!(exec_line.secondary_text, test_secondary_text);
        assert_eq!(exec_line.counter_text, test_counter_text);
        assert_eq!(exec_line.notes, test_notes);
    }

    #[tokio::test]
    async fn test_map_coords() {
        let test_line = CompLine {
            movements: create_test_movements(),
            other_movements: create_test_other_movements(),
            ..Default::default()
        };
        let mut map_section = Default::default();
        let exec_line = test_line.exec(0, 0, &mut map_section).await;
        let expected = vec![
            GameCoord(3.4, 5.0, 6.0),
            GameCoord(3.4, 7.0, 6.0),
            GameCoord(3.5, 5.0, 6.1),
            GameCoord(3.5, 7.4, 6.2),
            GameCoord(1.2, 55.0, 37.8),
            GameCoord(100.2, -3.0, 7.8),
            GameCoord(1.2, 55.0, 37.8),
        ];
        assert_eq!(exec_line.map_coords, expected);
    }

    #[tokio::test]
    async fn test_add_map_icon_and_markers() {
        let test_line = CompLine {
            map_icon: Some("test icon".to_string()),
            map_icon_priority: 3,
            markers: vec![
                CompMarker {
                    at: GameCoord(1.2, 55.0, 37.8),
                    color: "test color".to_string(),
                },
                CompMarker {
                    at: GameCoord(1.2, 85.0, 37.8),
                    color: "test color 2".to_string(),
                }
            ],
            map_coord: GameCoord(1.2, 0.0, 87.8),
            ..Default::default()
        };
        let mut builder = Default::default();
        test_line.exec(4, 5, &mut builder).await;
        assert_eq!(builder.icons, vec![
            MapIcon {
                id: "test icon".to_string(),
                coord: GameCoord(1.2, 0.0, 87.8),
                line_index: 5,
                section_index: 4,
                priority: 3,
            }
        ]);
        assert_eq!(builder.markers, vec![
            MapMarker {
                coord: GameCoord(1.2, 55.0, 37.8),
                color: "test color".to_string(),
                line_index: 5,
                section_index: 4,
            },
            MapMarker {
                coord: GameCoord(1.2, 85.0, 37.8),
                color: "test color 2".to_string(),
                line_index: 5,
                section_index: 4,
            }
        ]);
    }

    #[tokio::test]
    async fn test_map_lines() {
        let test_line = CompLine {
            line_color: "test color".to_string(),
            movements: create_test_movements(),
            other_movements: create_test_other_movements(),
            ..Default::default()
        };
        let mut map_builder = MapSectionBuilder::default();
        map_builder.add_main_movement("blue", &GameCoord::default()).await;
        test_line.exec(0, 0, &mut map_builder).await;
        let map_section = map_builder.build().await;
        assert_eq!(map_section.lines, vec![
            MapLine {
                color: "blue".to_string(),
                points: vec![
                    GameCoord::default(),
                    GameCoord(3.4, 5.0, 6.0)
                ],
            },
            MapLine {
                color: "red".to_string(),
                points: vec![
                    GameCoord(3.4, 5.0, 6.0),
                    GameCoord(3.4, 7.0, 6.0)
                ],
            },
            MapLine {
                color: "blue".to_string(),
                points: vec![
                    GameCoord::default(),
                    GameCoord(3.5, 5.0, 6.1)
                ],
            },
            MapLine {
                color: "red".to_string(),
                points: vec![
                    GameCoord(3.5, 5.0, 6.1),
                    GameCoord(3.5, 7.4, 6.2)
                ],
            },
            MapLine {
                color: "test color".to_string(),
                points: vec![
                    GameCoord::default(),
                    GameCoord(1.2, 55.0, 37.8),
                ],
            },
            MapLine {
                color: "test color".to_string(),
                points: vec![
                    GameCoord(100.2, -3.0, 7.8),
                    GameCoord(1.2, 55.0, 37.8)
                ],
            },
        ]);
    }

    #[tokio::test]
    async fn test_change_color_no_movement() {
        let test_line = CompLine {
            line_color: "test color".to_string(),
            ..Default::default()
        };
        let mut map_builder = MapSectionBuilder::default();
        map_builder.add_main_movement("blue", &GameCoord::default()).await;
        map_builder.add_main_movement("blue", &GameCoord::default()).await;
        test_line.exec(0, 0, &mut map_builder).await;

        map_builder.add_main_movement("test color", &GameCoord::default()).await;
        let map = map_builder.build().await;
        assert_eq!(map.lines, vec![
            MapLine {
                color: "blue".to_string(),
                points: vec![
                    GameCoord::default(),
                    GameCoord::default()
                ],
            },
            MapLine {
                color: "test color".to_string(),
                points: vec![
                    GameCoord::default(),
                    GameCoord::default()
                ],
            },
        ]);
    }
}
