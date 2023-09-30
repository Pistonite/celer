use celerctypes::ExecSection;

use crate::comp::CompSection;
use crate::util::async_for;

use super::{ExecResult, MapSectionBuilder};

impl CompSection {
    /// Execute the section.
    ///
    /// Map features will be added to the builder
    pub async fn exec(
        self,
        section_number: usize,
        map_builder: &mut MapSectionBuilder,
    ) -> ExecResult<ExecSection> {
        let mut lines = vec![];
        async_for!((index, line) in self.lines.into_iter().enumerate(), {
            let exec_line = line.exec(section_number, index, map_builder).await?;
            lines.push(exec_line);
        })?;
        Ok(ExecSection {
            name: self.name,
            lines,
            map: map_builder.build(),
        })
    }
}

#[cfg(test)]
mod test {
    use celerctypes::{GameCoord, MapIcon, MapLine, MapMarker};

    use crate::comp::{CompLine, CompMarker, CompMovement};

    use super::*;

    #[tokio::test]
    async fn test_name() {
        let test_section = CompSection {
            name: "test".to_string(),
            ..Default::default()
        };
        let exec_section = test_section
            .exec(1, &mut MapSectionBuilder::default())
            .await
            .unwrap();

        assert_eq!(exec_section.name, "test");
    }

    #[tokio::test]
    async fn test_section_and_line_index() {
        let test_section = CompSection {
            lines: vec![Default::default(), Default::default()],
            ..Default::default()
        };
        let exec_section = test_section
            .exec(3, &mut MapSectionBuilder::default())
            .await
            .unwrap();
        assert_eq!(exec_section.lines[0].section, 3);
        assert_eq!(exec_section.lines[0].index, 0);
        assert_eq!(exec_section.lines[1].section, 3);
        assert_eq!(exec_section.lines[1].index, 1);
    }

    #[tokio::test]
    async fn test_icons() {
        let test_section = CompSection {
            lines: vec![
                CompLine {
                    map_icon: Some("test 1".to_string()),
                    ..Default::default()
                },
                CompLine {
                    map_icon: Some("test 2".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let exec_section = test_section
            .exec(4, &mut MapSectionBuilder::default())
            .await
            .unwrap();
        assert_eq!(
            exec_section.map.icons,
            vec![
                MapIcon {
                    id: "test 1".to_string(),
                    section_index: 4,
                    line_index: 0,
                    priority: 0,
                    ..Default::default()
                },
                MapIcon {
                    id: "test 2".to_string(),
                    section_index: 4,
                    line_index: 1,
                    priority: 0,
                    ..Default::default()
                }
            ]
        );
    }

    #[tokio::test]
    async fn test_markers() {
        let test_section = CompSection {
            lines: vec![
                CompLine {
                    markers: vec![
                        CompMarker {
                            color: Some("test 1".to_string()),
                            ..Default::default()
                        },
                        CompMarker {
                            color: Some("test 2".to_string()),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                CompLine {
                    markers: vec![CompMarker::default()],
                    line_color: "test".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let exec_section = test_section
            .exec(4, &mut MapSectionBuilder::default())
            .await
            .unwrap();
        assert_eq!(
            exec_section.map.markers,
            vec![
                MapMarker {
                    section_index: 4,
                    line_index: 0,
                    color: "test 1".to_string(),
                    ..Default::default()
                },
                MapMarker {
                    section_index: 4,
                    line_index: 0,
                    color: "test 2".to_string(),
                    ..Default::default()
                },
                MapMarker {
                    section_index: 4,
                    line_index: 1,
                    color: "test".to_string(),
                    ..Default::default()
                },
            ]
        );
    }

    #[tokio::test]
    async fn test_lines() {
        let test_section = CompSection {
            lines: vec![
                CompLine {
                    line_color: "test".to_string(),
                    movements: vec![
                        CompMovement::to(GameCoord(1.0, 2.0, 3.0)),
                        CompMovement::to(GameCoord(1.0, 3.0, 3.0)),
                    ],
                    ..Default::default()
                },
                CompLine {
                    line_color: "test".to_string(),
                    movements: vec![
                        CompMovement::to(GameCoord(1.0, 4.0, 3.0)),
                        CompMovement::to(GameCoord(1.0, 5.0, 3.0)),
                    ],
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let mut builder = MapSectionBuilder::default();
        builder.add_coord("test", &GameCoord(1.0, 1.0, 3.0));

        let exec_section = test_section.exec(4, &mut builder).await.unwrap();

        assert_eq!(
            exec_section.map.lines,
            vec![MapLine {
                color: "test".to_string(),
                points: vec![
                    GameCoord(1.0, 1.0, 3.0),
                    GameCoord(1.0, 2.0, 3.0),
                    GameCoord(1.0, 3.0, 3.0),
                    GameCoord(1.0, 4.0, 3.0),
                    GameCoord(1.0, 5.0, 4.0),
                ],
            }]
        );
    }
}
