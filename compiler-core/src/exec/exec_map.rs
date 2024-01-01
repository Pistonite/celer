
// impl MapBuilder {
//     /// Create a new builder starting from the given color and coord
//     pub fn new(initial_color: &str, initial_coord: GameCoord) -> Self {
//         let mut builder = Self {
//             initial_color: initial_color.to_owned(),
//             initial_coord: initial_coord.clone(),
//             ..Default::default()
//         };
//         builder.move_to(initial_coord);
//         builder
//     }
//
//     /// Add a new point in the main movement
//     pub fn move_to(&mut self, coord: GameCoord) {
//         todo!()
//     }
//
//     /// Start a warp.
//     ///
//     /// Finish the current line, and a new line will start the next time `move_to` is called.
//     pub fn start_warp(&mut self) {
//         todo!()
//     }
//
//
//
//     pub fn add_coord(&mut self, color: Option<String>, coord: GameCoord) {
//         self.change_color(color);
//         match self.line_stack.last_mut() {
//             None => self.line_stack.push(MapLine {
//                 color: color.to_owned(),
//                 points: vec![coord.clone()],
//             }),
//             Some(line) => {
//                 line.points.push(coord.clone());
//             }
//         }
//     }
//
//     /// Change the color of the current line
//     ///
//     /// No-op if the current color is the same as the new color,
//     /// or if there's no current line on the stack
//     pub fn change_color(&mut self, color: &str) {
//         if let Some(current_line) = self.line_stack.last() {
//             if &current_line.color != color {
//                 self.commit(true);
//                 if let Some(line) = self.line_stack.last_mut() {
//                     line.color = color.to_string();
//                 }
//             }
//         }
//     }
//
//     /// Get the current line color
//     pub fn color(&self) -> &str {
//         todo!()
//         // self.line_stack.last().map(|line| line.color.as_str())
//     }
//
//     /// Get the current coord
//     pub fn coord(&self) -> &GameCoord {
//         todo!()
//     }
//
//     /// Finish the current line. Add it if there is more than one point
//     ///
//     /// If `continue_current`, the current line will be reset
//     /// with the current color and the last point.
//     /// Otherwise, the stack will be popped
//     fn commit(&mut self, continue_current: bool) {
//         if let Some(line) = self.line_stack.pop() {
//             if continue_current {
//                 self.line_stack.push(new_with_last_point(&line));
//             }
//             if line.points.len() > 1 {
//                 self.lines.push(line);
//             }
//         }
//     }
//
//     /// Save the current line (position and color) on the stack.
//     /// Then start a new line with the current position and color
//     ///
//     /// Use `commit(false)` to pop the stack top
//     pub fn push(&mut self) {
//         if let Some(line) = self.line_stack.last() {
//             self.line_stack.push(new_with_last_point(line));
//         } else {
//             self.line_stack.push(MapLine {
//                 color: self.initial_color.clone(),
//                 points: vec![],
//             });
//         }
//     }
//
//     /// Create a map section. Remove the current icons, markers and lines.
//     /// Keep the current line color and the last point.
//     pub fn build_section(&mut self) -> MapSection {
//         self.commit(true);
//         MapSection {
//             icons: std::mem::take(&mut self.icons),
//             markers: std::mem::take(&mut self.markers),
//             lines: std::mem::take(&mut self.lines),
//         }
//     }
// }
//
// fn new_with_last_point(line: &MapLine) -> MapLine {
//     let mut new_line = MapLine {
//         color: line.color.clone(),
//         points: vec![],
//     };
//     if let Some(last_point) = line.points.last() {
//         new_line.points.push(last_point.clone());
//     }
//     new_line
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_add_coord_when_empty() {
//         let mut builder = MapBuilder::default();
//         builder.add_coord("blue", &GameCoord(1.2, 55.0, 37.8));
//         assert_eq!(
//             builder.line_stack,
//             vec![MapLine {
//                 color: "blue".to_string(),
//                 points: vec![GameCoord(1.2, 55.0, 37.8)],
//             }]
//         );
//         assert_eq!(builder.color(), "blue");
//         assert_eq!(builder.coord(), &GameCoord(1.2, 55.0, 37.8));
//         assert_eq!(builder.lines, vec![]);
//     }
//
//     #[test]
//     fn test_add_coord_to_current_line() {
//         let mut builder = MapBuilder::default();
//         builder.add_coord("red", &GameCoord(1.2, 55.0, 37.8));
//         builder.add_coord("red", &GameCoord(1.2, 65.0, 37.8));
//         assert_eq!(
//             builder.line_stack,
//             vec![MapLine {
//                 color: "red".to_string(),
//                 points: vec![GameCoord(1.2, 55.0, 37.8), GameCoord(1.2, 65.0, 37.8)],
//             }]
//         );
//         assert_eq!(builder.color(), "red");
//         assert_eq!(builder.coord(), &GameCoord(1.2, 65.0, 37.8));
//         assert_eq!(builder.lines, vec![]);
//     }
//
//     #[test]
//     fn test_add_coord_different_color_no_add_line() {
//         let mut builder = MapBuilder::default();
//         builder.add_coord("blue", &GameCoord(1.2, 55.0, 37.8));
//         builder.add_coord("red", &GameCoord(1.2, 65.0, 37.8));
//         assert_eq!(
//             builder.line_stack,
//             vec![MapLine {
//                 color: "red".to_string(),
//                 points: vec![GameCoord(1.2, 55.0, 37.8), GameCoord(1.2, 65.0, 37.8)],
//             }]
//         );
//         assert_eq!(builder.lines, vec![]);
//     }
//
//     #[test]
//     fn test_change_color_no_current() {
//         let mut builder = MapSectionBuilder::default();
//         builder.change_color("red");
//         assert_eq!(builder.line_stack, vec![]);
//         assert_eq!(builder.lines, vec![]);
//     }
//
//     #[test]
//     fn test_change_color_same() {
//         let mut builder = MapSectionBuilder::default();
//         builder.add_coord("red", &GameCoord(1.2, 55.0, 37.8));
//         builder.add_coord("red", &GameCoord(1.2, 56.0, 37.8));
//         builder.change_color("red");
//         assert_eq!(
//             builder.line_stack,
//             vec![MapLine {
//                 color: "red".to_string(),
//                 points: vec![GameCoord(1.2, 55.0, 37.8), GameCoord(1.2, 56.0, 37.8)],
//             }]
//         );
//         assert_eq!(builder.lines, vec![]);
//     }
//
//     #[test]
//     fn test_change_color_different_should_commit() {
//         let mut builder = MapSectionBuilder::default();
//         builder.add_coord("red", &GameCoord(1.2, 55.0, 37.8));
//         builder.add_coord("red", &GameCoord(1.2, 56.0, 37.8));
//         builder.change_color("blue");
//         assert_eq!(
//             builder.line_stack,
//             vec![MapLine {
//                 color: "blue".to_string(),
//                 points: vec![GameCoord(1.2, 56.0, 37.8),],
//             }]
//         );
//         assert_eq!(
//             builder.lines,
//             vec![MapLine {
//                 color: "red".to_string(),
//                 points: vec![GameCoord(1.2, 55.0, 37.8), GameCoord(1.2, 56.0, 37.8),],
//             }]
//         );
//     }
//
//     #[test]
//     fn test_commit_none() {
//         let mut builder = MapSectionBuilder::default();
//         builder.commit(false);
//         assert_eq!(builder.line_stack, vec![]);
//         assert_eq!(builder.lines, vec![]);
//
//         builder.push();
//         builder.commit(true);
//         assert_eq!(
//             builder.line_stack,
//             vec![MapLine {
//                 color: "".to_string(),
//                 points: vec![],
//             }]
//         );
//         assert_eq!(builder.lines, vec![]);
//     }
//
//     #[test]
//     fn test_commit_one_coord_no_add() {
//         let test_line = MapLine {
//             color: "blue".to_string(),
//             points: vec![GameCoord(1.2, 55.0, 37.8)],
//         };
//         let mut builder = MapSectionBuilder {
//             line_stack: vec![test_line.clone()],
//             ..Default::default()
//         };
//         builder.commit(false);
//         assert_eq!(builder.line_stack, vec![]);
//         assert_eq!(builder.lines, vec![]);
//     }
//
//     #[test]
//     fn test_commit_many_coords_add() {
//         let test_line = MapLine {
//             color: "blue".to_string(),
//             points: vec![GameCoord(1.2, 55.0, 37.8), GameCoord(1.2, 65.0, 37.8)],
//         };
//         let mut builder = MapSectionBuilder {
//             line_stack: vec![test_line.clone()],
//             ..Default::default()
//         };
//         builder.commit(false);
//         assert_eq!(builder.line_stack, vec![]);
//         assert_eq!(builder.lines, vec![test_line]);
//     }
//
//     #[test]
//     fn test_commit_many_coords_continue_current() {
//         let test_line = MapLine {
//             color: "blue".to_string(),
//             points: vec![GameCoord(1.2, 55.0, 37.8), GameCoord(1.2, 65.0, 37.8)],
//         };
//         let mut builder = MapSectionBuilder {
//             line_stack: vec![test_line.clone()],
//             ..Default::default()
//         };
//         builder.commit(true);
//         assert_eq!(
//             builder.line_stack,
//             vec![MapLine {
//                 color: "blue".to_string(),
//                 points: vec![GameCoord(1.2, 65.0, 37.8)],
//             }]
//         );
//         assert_eq!(builder.lines, vec![test_line]);
//     }
//
//     #[test]
//     fn test_push_empty() {
//         let mut builder = MapSectionBuilder {
//             line_stack: vec![],
//             ..Default::default()
//         };
//         builder.push();
//         assert_eq!(
//             builder.line_stack,
//             vec![MapLine {
//                 color: "".to_string(),
//                 points: vec![],
//             },]
//         );
//         builder.push();
//         assert_eq!(
//             builder.line_stack,
//             vec![
//                 MapLine {
//                     color: "".to_string(),
//                     points: vec![],
//                 },
//                 MapLine {
//                     color: "".to_string(),
//                     points: vec![],
//                 },
//             ]
//         );
//     }
//
//     #[test]
//     fn test_push_existing_one() {
//         let test_line = MapLine {
//             color: "blue".to_string(),
//             points: vec![GameCoord(1.2, 55.0, 37.8)],
//         };
//         let mut builder = MapSectionBuilder {
//             line_stack: vec![test_line.clone()],
//             ..Default::default()
//         };
//         builder.push();
//         assert_eq!(
//             builder.line_stack,
//             vec![
//                 MapLine {
//                     color: "blue".to_string(),
//                     points: vec![GameCoord(1.2, 55.0, 37.8),],
//                 },
//                 MapLine {
//                     color: "blue".to_string(),
//                     points: vec![GameCoord(1.2, 55.0, 37.8),],
//                 }
//             ]
//         );
//     }
//
//     #[test]
//     fn test_push_existing_more_than_one() {
//         let test_line = MapLine {
//             color: "blue".to_string(),
//             points: vec![GameCoord(1.2, 55.0, 37.8), GameCoord(1.2, 65.0, 37.8)],
//         };
//         let mut builder = MapSectionBuilder {
//             line_stack: vec![test_line.clone()],
//             ..Default::default()
//         };
//         builder.push();
//         assert_eq!(
//             builder.line_stack,
//             vec![
//                 MapLine {
//                     color: "blue".to_string(),
//                     points: vec![GameCoord(1.2, 55.0, 37.8), GameCoord(1.2, 65.0, 37.8),],
//                 },
//                 MapLine {
//                     color: "blue".to_string(),
//                     points: vec![GameCoord(1.2, 65.0, 37.8)],
//                 }
//             ]
//         );
//     }
//
//     #[test]
//     fn test_build_icons_and_markers() {
//         let test_icons = vec![
//             MapIcon {
//                 id: "test".to_string(),
//                 ..Default::default()
//             },
//             MapIcon {
//                 id: "test1".to_string(),
//                 ..Default::default()
//             },
//         ];
//         let test_markers = vec![
//             MapMarker {
//                 color: "red".to_string(),
//                 ..Default::default()
//             },
//             MapMarker {
//                 color: "blue".to_string(),
//                 ..Default::default()
//             },
//         ];
//         let mut builder = MapSectionBuilder {
//             icons: test_icons.clone(),
//             markers: test_markers.clone(),
//             ..Default::default()
//         };
//         let section = builder.build();
//         assert_eq!(section.icons, test_icons);
//         assert_eq!(section.markers, test_markers);
//         assert_eq!(builder.icons, vec![]);
//         assert_eq!(builder.markers, vec![]);
//     }
//
//     #[test]
//     fn test_build_commit_line() {
//         let test_line = MapLine {
//             color: "blue".to_string(),
//             points: vec![GameCoord(1.2, 55.0, 37.8), GameCoord(1.2, 65.0, 37.8)],
//         };
//         let test_current_line = MapLine {
//             color: "red".to_string(),
//             points: vec![GameCoord(1.2, 66.0, 37.8), GameCoord(1.2, 67.0, 37.8)],
//         };
//         let mut builder = MapSectionBuilder {
//             lines: vec![test_line.clone()],
//             line_stack: vec![test_current_line.clone()],
//             ..Default::default()
//         };
//         let section = builder.build();
//         assert_eq!(section.lines, vec![test_line, test_current_line]);
//         assert_eq!(
//             builder.line_stack,
//             vec![MapLine {
//                 color: "red".to_string(),
//                 points: vec![GameCoord(1.2, 67.0, 37.8),],
//             }]
//         );
//         assert_eq!(builder.lines, vec![]);
//     }
// }
