use celerctypes::{MapIcon, MapMarker, MapLine, GameCoord, ExecMapSection};

use crate::CompMovementWithColor;

#[derive(PartialEq, Default, Debug, Clone)]
pub struct MapSectionBuilder {
    /// The icons
    pub icons: Vec<MapIcon>,
    /// The markers
    pub markers: Vec<MapMarker>,
    /// The added lines
    lines: Vec<MapLine>,
    /// The current line
    current_line: Option<MapLine>,
}

impl MapSectionBuilder {

    /// Add a new point in the main movement
    pub fn add_main_movement(&mut self, color: &str, coord: &GameCoord) {
        // if let Some(color) = color.as_ref() {
            if let Some(current_line) = self.current_line.as_ref() {
                if current_line.color.as_str() != color {
                    self.commit_line(true);
                    if let Some(line) = self.current_line.as_mut() {
                        line.color = color.to_string();
                    }
                }
            }
        // }
        match self.current_line.as_mut() {
            None => self.current_line = Some(MapLine {
                color: color.to_owned(),
                points: vec![coord.clone()],
            }),
            Some(line) => {
                line.points.push(coord.clone());
            }
        }
    }

    /// Finish the current line. Add it if there is more than one point
    ///
    /// If continue_current, a new main movement will be added with the current point, if it
    /// exists.
    pub fn commit_line(&mut self, continue_current: bool) {
        if let Some(line) = self.current_line.take() {
            if continue_current {
                self.current_line = Some(MapLine {
                    color: line.color.clone(),
                    points: vec![line.points.last().cloned().unwrap_or_default()],
                });
            }
            if line.points.len() > 1 {
                self.lines.push(line);
            }
        }
    }

    /// Add another movement branch starting from current point (or none), without affecting
    /// current line
    pub fn add_other_movement(&mut self, movements: &[CompMovementWithColor]) {
        let mut delegate_builder = MapSectionBuilder::default();
        if let Some(line) = self.current_line.as_ref() {
            delegate_builder.current_line = Some(MapLine {
                color: line.color.clone(),
                points: vec![line.points.last().cloned().unwrap_or_default()],
            });
        }
        for m in movements {
            if m.movement.warp {
                delegate_builder.commit_line(false);
            }
            delegate_builder.add_main_movement(&m.color, &m.movement.to);
        }
        delegate_builder.commit_line(false);
        self.lines.extend(delegate_builder.lines);
    }

    /// Create a map section. Remove the current icons, markers and lines.
    /// Keep the current line color and the last point.
    pub fn build(&mut self) -> ExecMapSection {
        self.commit_line(true);
        ExecMapSection {
            icons: std::mem::take(&mut self.icons),
            markers: std::mem::take(&mut self.markers),
            lines: std::mem::take(&mut self.lines),
        }
    }
}

#[cfg(test)]
mod ut {
    use crate::CompMovement;

    use super::*;

    #[test]
    fn test_add_coord_when_empty() {
        let mut builder = MapSectionBuilder::default();
        builder.add_main_movement("blue", &GameCoord(1.2, 55.0, 37.8));
        assert_eq!(builder.current_line, Some(
            MapLine {
                color: "blue".to_string(),
                points: vec![GameCoord(1.2, 55.0, 37.8)],
            }
        ));
        assert_eq!(builder.lines, vec![]);
    }

    #[test]
    fn test_add_coord_to_current_line() {
        let mut builder = MapSectionBuilder::default();
        builder.add_main_movement("red", &GameCoord(1.2, 55.0, 37.8));
        builder.add_main_movement("red", &GameCoord(1.2, 65.0, 37.8));
        assert_eq!(builder.current_line, Some(
            MapLine {
                color: "red".to_string(),
                points: vec![
                    GameCoord(1.2, 55.0, 37.8),
                    GameCoord(1.2, 65.0, 37.8)
                ],
            }
        ));
        assert_eq!(builder.lines, vec![]);
    }

    #[test]
    fn test_add_coord_different_color_no_add_line() {
        let mut builder = MapSectionBuilder::default();
        builder.add_main_movement("blue", &GameCoord(1.2, 55.0, 37.8));
        builder.add_main_movement("red", &GameCoord(1.2, 65.0, 37.8));
        assert_eq!(builder.current_line, Some(
            MapLine {
                color: "red".to_string(),
                points: vec![
                    GameCoord(1.2, 55.0, 37.8),
                    GameCoord(1.2, 65.0, 37.8)
                ],
            }
        ));
        assert_eq!(builder.lines, vec![]);
    }

    #[test]
    fn test_commit_none() {
        let mut builder = MapSectionBuilder::default();
        builder.commit_line(false);
        assert_eq!(builder.current_line, None);
        assert_eq!(builder.lines, vec![]);
    }

    #[test]
    fn test_commit_one_coord_no_add() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                    ],
                };
        let mut builder = MapSectionBuilder {
            current_line: Some(test_line.clone()),
            ..Default::default()
        };
        builder.commit_line(false);
        assert_eq!(builder.current_line, None);
        assert_eq!(builder.lines, vec![]);
    }

    #[test]
    fn test_commit_many_coords_add() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                        GameCoord(1.2, 65.0, 37.8)
                    ],
                };
        let mut builder = MapSectionBuilder {
            current_line: Some(test_line.clone()),
            ..Default::default()
        };
        builder.commit_line(false);
        assert_eq!(builder.current_line, None);
        assert_eq!(builder.lines, vec![test_line]);
    }

    #[test]
    fn test_commit_many_coords_continue_current() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                        GameCoord(1.2, 65.0, 37.8)
                    ],
                };
        let mut builder = MapSectionBuilder {
            current_line: Some(test_line.clone()),
            ..Default::default()
        };
        builder.commit_line(true);
        assert_eq!(builder.current_line, Some(
            MapLine {
                color: "blue".to_string(),
                points: vec![
                    GameCoord(1.2, 65.0, 37.8)
                ],
            }
        ));
        assert_eq!(builder.lines, vec![test_line]);
    }

    #[test]
    fn test_other_movement_from_none_no_add() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                        GameCoord(1.2, 65.0, 37.8)
                    ],
                };
        let mut builder = MapSectionBuilder {
            lines: vec![test_line.clone()],
            current_line: None,
            ..Default::default()
        };
        builder.add_other_movement(&[
            CompMovementWithColor {
                color: "blue".to_string(),
                movement: CompMovement {
                    to: GameCoord(1.2, 65.0, 37.8),
                    warp: false,
                }
            }
        ]);
        assert_eq!(builder.current_line, None);
        assert_eq!(builder.lines, vec![test_line]);
    }
    
    #[test]
    fn test_other_movement_from_none_add() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                        GameCoord(1.2, 65.0, 37.8)
                    ],
                };
        let mut builder = MapSectionBuilder {
            lines: vec![test_line.clone()],
            current_line: None,
            ..Default::default()
        };
        builder.add_other_movement(&[
            CompMovementWithColor {
                color: "red".to_string(),
                movement: CompMovement {
                    to: GameCoord(1.2, 65.0, 37.8),
                    warp: false,
                }
            },
            CompMovementWithColor {
                color: "red".to_string(),
                movement: CompMovement {
                    to: GameCoord(1.2, 66.0, 37.8),
                    warp: false,
                }
            }
        ]);
        assert_eq!(builder.current_line, None);
        assert_eq!(builder.lines, vec![test_line, MapLine {
            color: "red".to_string(),
            points: vec![
                GameCoord(1.2, 65.0, 37.8),
                GameCoord(1.2, 66.0, 37.8)
            ],
        }]);
    }

    #[test]
    fn test_other_movement_from_none_add_change_color() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                        GameCoord(1.2, 65.0, 37.8)
                    ],
                };
        let mut builder = MapSectionBuilder {
            lines: vec![test_line.clone()],
            current_line: None,
            ..Default::default()
        };
        builder.add_other_movement(&[
            CompMovementWithColor {
                color: "red".to_string(),
                movement: CompMovement {
                    to: GameCoord(1.2, 65.0, 37.8),
                    warp: false,
                }
            },
            CompMovementWithColor {
                color: "yellow".to_string(),
                movement: CompMovement {
                    to: GameCoord(1.2, 66.0, 37.8),
                    warp: false,
                }
            }
        ]);
        assert_eq!(builder.current_line, None);
        assert_eq!(builder.lines, vec![test_line, MapLine {
            color: "yellow".to_string(),
            points: vec![
                GameCoord(1.2, 65.0, 37.8),
                GameCoord(1.2, 66.0, 37.8)
            ],
        }]);
    }

    #[test]
    fn test_other_movement_from_existing_add_empty() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                        GameCoord(1.2, 65.0, 37.8)
                    ],
                };
        let test_current_line = MapLine {
                    color: "red".to_string(),
                    points: vec![
                        GameCoord(1.2, 66.0, 37.8),
                        GameCoord(1.2, 67.0, 37.8)
                    ],
                };
        let mut builder = MapSectionBuilder {
            lines: vec![test_line.clone()],
            current_line: Some(test_current_line.clone()),
            ..Default::default()
        };
        builder.add_other_movement(&[]);
        assert_eq!(builder.current_line, Some(test_current_line));
        assert_eq!(builder.lines, vec![test_line]);
    }

    #[test]
    fn test_other_movement_from_existing_add_one_no_color() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                        GameCoord(1.2, 65.0, 37.8)
                    ],
                };
        let test_current_line = MapLine {
                    color: "red".to_string(),
                    points: vec![
                        GameCoord(1.2, 66.0, 37.8),
                        GameCoord(1.2, 67.0, 37.8)
                    ],
                };
        let mut builder = MapSectionBuilder {
            lines: vec![test_line.clone()],
            current_line: Some(test_current_line.clone()),
            ..Default::default()
        };
        builder.add_other_movement(&[CompMovementWithColor {
            color: "red".to_string(),
            movement: CompMovement {
                to: GameCoord(1.2, 68.0, 37.8),
                warp: false,
            }
        }]);
        assert_eq!(builder.current_line, Some(test_current_line));
        assert_eq!(builder.lines, vec![test_line, MapLine {
            color: "red".to_string(),
            points: vec![
                GameCoord(1.2, 67.0, 37.8),
                GameCoord(1.2, 68.0, 37.8)
            ],
        }]);
    }

    #[test]
    fn test_other_movement_from_existing_add_one_color() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                        GameCoord(1.2, 65.0, 37.8)
                    ],
                };
        let test_current_line = MapLine {
                    color: "red".to_string(),
                    points: vec![
                        GameCoord(1.2, 66.0, 37.8),
                        GameCoord(1.2, 67.0, 37.8)
                    ],
                };
        let mut builder = MapSectionBuilder {
            lines: vec![test_line.clone()],
            current_line: Some(test_current_line.clone()),
            ..Default::default()
        };
        builder.add_other_movement(&[CompMovementWithColor {
            color: "green".to_string(),
            movement: CompMovement {
                to: GameCoord(1.2, 68.0, 37.8),
                warp: false,
            }
        }]);
        assert_eq!(builder.current_line, Some(test_current_line));
        assert_eq!(builder.lines, vec![test_line, MapLine {
            color: "green".to_string(),
            points: vec![
                GameCoord(1.2, 67.0, 37.8),
                GameCoord(1.2, 68.0, 37.8)
            ],
        }]);
    }

    #[test]
    fn test_other_movement_from_existing_add_more_than_one() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                        GameCoord(1.2, 65.0, 37.8)
                    ],
                };
        let test_current_line = MapLine {
                    color: "red".to_string(),
                    points: vec![
                        GameCoord(1.2, 66.0, 37.8),
                        GameCoord(1.2, 67.0, 37.8)
                    ],
                };
        let mut builder = MapSectionBuilder {
            lines: vec![test_line.clone()],
            current_line: Some(test_current_line.clone()),
            ..Default::default()
        };
        builder.add_other_movement(&[CompMovementWithColor {
            color: "red".to_string(),
            movement: CompMovement {
                to: GameCoord(1.2, 68.0, 37.8),
                warp: false,
            }
        },CompMovementWithColor {
            color: "green".to_string(),
            movement: CompMovement {
                to: GameCoord(1.2, 69.0, 37.8),
                warp: false,
            }
        }]);
        assert_eq!(builder.current_line, Some(test_current_line));
        assert_eq!(builder.lines, vec![test_line, MapLine {
            color: "red".to_string(),
            points: vec![
                GameCoord(1.2, 67.0, 37.8),
                GameCoord(1.2, 68.0, 37.8)
            ],
        },MapLine {
            color: "green".to_string(),
            points: vec![
                GameCoord(1.2, 68.0, 37.8),
                GameCoord(1.2, 69.0, 37.8)
            ],
        }]);
    }

    #[test]
    fn test_other_movement_from_existing_warp() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                        GameCoord(1.2, 65.0, 37.8)
                    ],
                };
        let test_current_line = MapLine {
                    color: "red".to_string(),
                    points: vec![
                        GameCoord(1.2, 66.0, 37.8),
                        GameCoord(1.2, 67.0, 37.8)
                    ],
                };
        let mut builder = MapSectionBuilder {
            lines: vec![test_line.clone()],
            current_line: Some(test_current_line.clone()),
            ..Default::default()
        };
        builder.add_other_movement(&[CompMovementWithColor {
            color: "red".to_string(),
            movement: CompMovement {
                to: GameCoord(1.2, 68.0, 37.8),
                warp: false,
            }
        },CompMovementWithColor {
            color: "green".to_string(),
            movement: CompMovement {
                to: GameCoord(1.2, 69.0, 37.8),
                warp: true,
            }
        },CompMovementWithColor {
            color: "green".to_string(),
            movement: CompMovement {
                to: GameCoord(1.2, 70.0, 37.8),
                warp: false,
            }
        }]);
        assert_eq!(builder.current_line, Some(test_current_line));
        assert_eq!(builder.lines, vec![test_line, MapLine {
            color: "red".to_string(),
            points: vec![
                GameCoord(1.2, 67.0, 37.8),
                GameCoord(1.2, 68.0, 37.8)
            ],
        },MapLine {
            color: "green".to_string(),
            points: vec![
                GameCoord(1.2, 69.0, 37.8),
                GameCoord(1.2, 70.0, 37.8)
            ],
        }]);
    }

    #[test]
    fn test_build_icons_and_markers() {
        let test_icons = vec![
            MapIcon {
                id: "test".to_string(),
                ..Default::default()
            },
            MapIcon {
                id: "test1".to_string(),
                ..Default::default()
            }
        ];
        let test_markers = vec![
            MapMarker {
                color: "red".to_string(),
                ..Default::default()
            },
            MapMarker {
                color: "blue".to_string(),
                ..Default::default()
            }
        ];
        let mut builder = MapSectionBuilder {
            icons: test_icons.clone(),
            markers: test_markers.clone(),
            ..Default::default()
        };
        let section = builder.build();
        assert_eq!(section.icons, test_icons);
        assert_eq!(section.markers, test_markers);
        assert_eq!(builder.icons, vec![]);
        assert_eq!(builder.markers, vec![]);
    }

    #[test]
    fn test_build_commit_line() {
        let test_line = MapLine {
                    color: "blue".to_string(),
                    points: vec![
                        GameCoord(1.2, 55.0, 37.8),
                        GameCoord(1.2, 65.0, 37.8)
                    ],
                };
        let test_current_line = MapLine {
                    color: "red".to_string(),
                    points: vec![
                        GameCoord(1.2, 66.0, 37.8),
                        GameCoord(1.2, 67.0, 37.8)
                    ],
                };
        let mut builder = MapSectionBuilder {
            lines: vec![test_line.clone()],
            current_line: Some(test_current_line.clone()),
            ..Default::default()
        };
        let section = builder.build();
        assert_eq!(section.lines, vec![test_line, test_current_line]);
        assert_eq!(builder.current_line, Some(
            MapLine {
                color: "red".to_string(),
                points: vec![
                    GameCoord(1.2, 67.0, 37.8),
                ],
            }
        ));
        assert_eq!(builder.lines, vec![]);
    }
}
