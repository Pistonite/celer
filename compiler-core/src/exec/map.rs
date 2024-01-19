//! Convert declared movements and colors to lines

use crate::macros::derive_wasm;
use crate::prep::GameCoord;

/// Map features for one section
#[derive(PartialEq, Default, Debug, Clone)]
#[derive_wasm]
pub struct MapSection {
    /// The icons
    pub icons: Vec<MapIcon>,
    /// The markers
    pub markers: Vec<MapMarker>,
    /// The lines
    pub lines: Vec<MapLine>,
}

/// Icon on the map
#[derive(PartialEq, Default, Debug, Clone)]
#[derive_wasm]
pub struct MapIcon {
    /// Internal icon name (usually kebab-case)
    pub id: String,
    /// Game coordinate for the icon
    pub coord: GameCoord,
    /// The corresponding line index in section of the document
    pub line_index: usize,
    /// The corresponding section number in the document
    pub section_index: usize,
    /// The priority of the icon (0 = primary, 1 = secondary)
    pub priority: i64,
}

/// Markers on the map
#[derive(PartialEq, Default, Debug, Clone)]
#[derive_wasm]
pub struct MapMarker {
    pub coord: GameCoord,
    /// The corresponding line index in section of the document
    pub line_index: usize,
    /// The corresponding section number in the document
    pub section_index: usize,
    /// Color of the marker
    pub color: String,
}

/// Paths on the map
///
/// The coordinates do not have to be on the same map layer.
/// The map will automatically split the path if it croses map layers.
#[derive(PartialEq, Default, Debug, Clone)]
#[derive_wasm]
pub struct MapLine {
    /// Color of the line
    pub color: String,
    /// Points on the line
    pub points: Vec<GameCoord>,
}

/// Builder for producing [`MapSection`]s.
#[derive(PartialEq, Debug, Clone)]
pub struct MapBuilder {
    /// The icons
    pub icons: Vec<MapIcon>,
    /// The markers
    pub markers: Vec<MapMarker>,
    /// The added lines
    lines: Vec<MapLine>,
    /// The current line stack
    line_stack: Vec<MapLine>,

    initial_color: String,
    initial_coord: GameCoord,
}

impl Default for MapBuilder {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl MapBuilder {
    pub fn new(initial_color: String, initial_coord: GameCoord) -> Self {
        Self {
            icons: vec![],
            markers: vec![],
            lines: vec![],
            line_stack: vec![MapLine {
                color: initial_color.clone(),
                points: vec![initial_coord.clone()],
            }],
            initial_color,
            initial_coord,
        }
    }

    /// Advance the current line to the next coordinate
    pub fn move_to(&mut self, coord: GameCoord) {
        if let Some(line) = self.line_stack.last_mut() {
            line.points.push(coord.clone());
        } else {
            self.line_stack.push(MapLine {
                color: self.initial_color.clone(),
                points: vec![self.initial_coord.clone(), coord.clone()],
            });
        }
    }

    /// Change the color of the current line
    pub fn change_color(&mut self, color: String) {
        if self.color() == &color {
            return;
        }
        self.commit();
        if let Some(line) = self.line_stack.last_mut() {
            line.color = color;
        }
    }

    /// Stops the current line and starts a new line from the coordinate
    pub fn warp_to(&mut self, coord: GameCoord) {
        self.commit();
        if let Some(line) = self.line_stack.last_mut() {
            if let Some(last_point) = line.points.last_mut() {
                *last_point = coord.clone();
            }
        }
    }

    /// Save the current color and coordinate to the stack
    pub fn push(&mut self) {
        if let Some(line) = self.line_stack.last() {
            if let Some(last_point) = line.points.last() {
                self.line_stack.push(MapLine {
                    color: line.color.clone(),
                    points: vec![last_point.clone()],
                });
            } else {
                self.line_stack.push(MapLine {
                    color: line.color.clone(),
                    points: vec![self.initial_coord.clone()],
                });
            }
        } else {
            self.line_stack.push(MapLine {
                color: self.initial_color.clone(),
                points: vec![self.initial_coord.clone()],
            });
        }
    }

    /// Stops the current line and starts a new line from the
    /// last saved position (color and coordinate)
    pub fn pop(&mut self) {
        self.commit();
        self.line_stack.pop();
        if self.line_stack.is_empty() {
            self.line_stack.push(MapLine {
                color: self.initial_color.clone(),
                points: vec![self.initial_coord.clone()],
            });
        }
    }

    /// Get the current color
    pub fn color(&self) -> &str {
        match self.line_stack.last() {
            Some(line) => &line.color,
            None => &self.initial_color,
        }
    }

    /// Get the current coordinate
    pub fn coord(&self) -> &GameCoord {
        self.line_stack
            .last()
            .and_then(|line| line.points.last())
            .unwrap_or(&self.initial_coord)
    }

    /// Remove the stack top and add it to the commited lines if it has more than 1 coord
    /// Put a new line on the stack with only the last coordinate
    fn commit(&mut self) {
        let line = match self.line_stack.pop() {
            Some(line) => line,
            None => return,
        };
        let new_line = if line.points.len() > 1 {
            // unwrap is safe because length > 1
            let last_point = line.points.last().unwrap().clone();
            let new_line = MapLine {
                color: line.color.clone(),
                points: vec![last_point],
            };
            self.lines.push(line); // commit the popped line
            new_line
        } else {
            line // put the line back untouched
        };
        self.line_stack.push(new_line);
    }

    /// Stops the current line and move all the lines to the output,
    /// then starts a new line from the same position.
    pub fn build_section(&mut self) -> MapSection {
        self.commit();
        MapSection {
            icons: std::mem::take(&mut self.icons),
            markers: std::mem::take(&mut self.markers),
            lines: std::mem::take(&mut self.lines),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_coord {
        (A) => {
            GameCoord(1.2, 55.0, 37.8)
        };
        (B) => {
            GameCoord(1.2, 65.0, 37.8)
        };
        (C) => {
            GameCoord(1.2, 67.0, 37.8)
        };
        (D) => {
            GameCoord(11.2, 67.0, 37.8)
        };
    }

    #[test]
    fn test_add_coord_to_current_line() {
        // before
        // red: A
        //
        // after
        // red: A -> B
        let mut builder = MapBuilder {
            line_stack: vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A)],
            }],
            ..Default::default()
        };
        builder.move_to(test_coord!(B));
        assert_eq!(builder.lines.len(), 0);
        assert_eq!(
            builder.line_stack,
            vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A), test_coord!(B)],
            }]
        );
        assert_eq!(builder.color(), "red");
        assert_eq!(builder.coord(), &test_coord!(B));
    }

    #[test]
    fn test_change_color_no_commit() {
        // before
        // red: A
        //
        // after
        // blue: A
        let mut builder = MapBuilder {
            line_stack: vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A)],
            }],
            ..Default::default()
        };
        builder.change_color("blue".to_string());
        assert_eq!(builder.lines.len(), 0);
        assert_eq!(
            builder.line_stack,
            vec![MapLine {
                color: "blue".to_string(),
                points: vec![test_coord!(A)],
            }]
        );
        assert_eq!(builder.color(), "blue");
        assert_eq!(builder.coord(), &test_coord!(A));
    }

    #[test]
    fn test_change_color_commit() {
        // before
        // red: A -> B
        //
        // after
        // blue: B
        //
        // commited
        // red: A -> B
        let mut builder = MapBuilder {
            line_stack: vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A), test_coord!(B)],
            }],
            ..Default::default()
        };
        builder.change_color("blue".to_string());
        assert_eq!(
            builder.lines,
            vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A), test_coord!(B)],
            },]
        );
        assert_eq!(
            builder.line_stack,
            vec![MapLine {
                color: "blue".to_string(),
                points: vec![test_coord!(B)],
            }]
        );
        assert_eq!(builder.color(), "blue");
        assert_eq!(builder.coord(), &test_coord!(B));
    }

    #[test]
    fn test_change_color_same() {
        // before
        // red: A -> B -> C
        //
        // after
        // red: A -> B -> C
        let mut builder = MapBuilder {
            line_stack: vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A), test_coord!(B), test_coord!(C)],
            }],
            ..Default::default()
        };
        builder.change_color("red".to_string());
        assert_eq!(builder.lines, vec![]);
        assert_eq!(
            builder.line_stack,
            vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A), test_coord!(B), test_coord!(C)],
            }]
        );
        assert_eq!(builder.color(), "red");
        assert_eq!(builder.coord(), &test_coord!(C));
    }

    #[test]
    fn test_warp_no_commit() {
        // before
        // red: A
        //
        // after
        // red: B
        let mut builder = MapBuilder {
            line_stack: vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A)],
            }],
            ..Default::default()
        };
        builder.warp_to(test_coord!(B));
        assert_eq!(builder.lines, vec![]);
        assert_eq!(
            builder.line_stack,
            vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(B)],
            }]
        );
        assert_eq!(builder.color(), "red");
        assert_eq!(builder.coord(), &test_coord!(B));
    }

    #[test]
    fn test_warp_commit() {
        // before
        // red: A -> B
        //
        // after
        // red: C
        //
        // commited
        // red: A -> B
        let mut builder = MapBuilder {
            line_stack: vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A), test_coord!(B)],
            }],
            ..Default::default()
        };
        builder.warp_to(test_coord!(C));
        assert_eq!(
            builder.lines,
            vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A), test_coord!(B)],
            },]
        );
        assert_eq!(
            builder.line_stack,
            vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(C)],
            }]
        );
        assert_eq!(builder.color(), "red");
        assert_eq!(builder.coord(), &test_coord!(C));
    }

    #[test]
    fn test_push() {
        // before
        // red: A -> B
        //
        // after
        // red: A -> B
        // red: B
        let mut builder = MapBuilder {
            line_stack: vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A), test_coord!(B)],
            }],
            ..Default::default()
        };
        builder.push();
        assert_eq!(builder.lines, vec![]);
        assert_eq!(
            builder.line_stack,
            vec![
                MapLine {
                    color: "red".to_string(),
                    points: vec![test_coord!(A), test_coord!(B)],
                },
                MapLine {
                    color: "red".to_string(),
                    points: vec![test_coord!(B)],
                }
            ]
        );
        assert_eq!(builder.color(), "red");
        assert_eq!(builder.coord(), &test_coord!(B));
    }

    #[test]
    fn test_push_pop() {
        // before
        // red: A -> B
        let mut builder = MapBuilder {
            line_stack: vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A), test_coord!(B)],
            }],
            ..Default::default()
        };
        //
        // after push
        // red: A -> B
        // red: B
        builder.push();
        assert_eq!(builder.lines, vec![]);
        assert_eq!(
            builder.line_stack,
            vec![
                MapLine {
                    color: "red".to_string(),
                    points: vec![test_coord!(A), test_coord!(B)],
                },
                MapLine {
                    color: "red".to_string(),
                    points: vec![test_coord!(B)],
                }
            ]
        );
        assert_eq!(builder.color(), "red");
        assert_eq!(builder.coord(), &test_coord!(B));
        //
        // after move
        // red: A -> B
        // red: B -> C
        builder.move_to(test_coord!(C));
        assert_eq!(builder.lines, vec![]);
        assert_eq!(
            builder.line_stack,
            vec![
                MapLine {
                    color: "red".to_string(),
                    points: vec![test_coord!(A), test_coord!(B)],
                },
                MapLine {
                    color: "red".to_string(),
                    points: vec![test_coord!(B), test_coord!(C)],
                }
            ]
        );
        assert_eq!(builder.color(), "red");
        assert_eq!(builder.coord(), &test_coord!(C));
        //
        // after pop
        // red: A -> B
        builder.pop();
        assert_eq!(
            builder.lines,
            vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(B), test_coord!(C)],
            },]
        );
        assert_eq!(
            builder.line_stack,
            vec![MapLine {
                color: "red".to_string(),
                points: vec![test_coord!(A), test_coord!(B)],
            },]
        );
        assert_eq!(builder.color(), "red");
        assert_eq!(builder.coord(), &test_coord!(B));
    }

    #[test]
    fn test_pop_last() {
        let mut builder = MapBuilder::default();
        assert_eq!(builder.line_stack.len(), 1);
        builder.pop();
        assert_eq!(builder.line_stack.len(), 1);
    }
}
