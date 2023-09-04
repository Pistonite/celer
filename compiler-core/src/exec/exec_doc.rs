use celerctypes::ExecDoc;

use crate::comp::CompDoc;

use super::MapSectionBuilder;

impl CompDoc {
    /// Execute the document
    pub async fn exec(self) -> ExecDoc {
        let mut map_builder = MapSectionBuilder::default();
        map_builder.add_coord("", &self.project.map.initial_coord);
        let mut sections = vec![];
        for (index, section) in self.route.into_iter().enumerate() {
            let exec_section = section.exec(index, &mut map_builder).await;
            sections.push(exec_section);
        }
        ExecDoc {
            project: self.project,
            preface: self.preface,
            route: sections,
        }
    }
}

#[cfg(test)]
mod test {
    use celerctypes::{
        ExecLine, ExecMapSection, ExecSection, GameCoord, MapLine, MapMetadata, RouteMetadata, DocPoorText,
    };

    use crate::comp::{CompMovement, CompLine, CompSection};

    use super::*;

    #[tokio::test]
    async fn test_copy() {
        let test_metadata = RouteMetadata {
            name: "test".to_string(),
            version: "test version".to_string(),
            ..Default::default()
        };
        
        let test_preface = vec![vec![DocPoorText::Text("test".to_string())]];

        let test_doc = CompDoc {
            project: test_metadata.clone(),
            preface: test_preface.clone(),
            ..Default::default()
        };


        let exec_doc = test_doc.exec().await;
        assert_eq!(exec_doc.project, test_metadata);
        assert_eq!(exec_doc.preface, test_preface);
    }

    #[tokio::test]
    async fn test_sections() {
        let test_sections = vec![
            CompSection {
                name: "test1".to_string(),
                lines: vec![CompLine {
                    movements: vec![CompMovement::to(GameCoord(1.2, 2.2, 3.3))],
                    line_color: "color".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            CompSection {
                name: "test2".to_string(),
                lines: vec![CompLine {
                    movements: vec![CompMovement::to(GameCoord(1.3, 2.2, 3.3))],
                    line_color: "color".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
        ];

        let test_doc = CompDoc {
            project: RouteMetadata {
                map: MapMetadata {
                    initial_coord: GameCoord(1.1, 2.2, 3.3),
                    ..Default::default()
                },
                ..Default::default()
            },
            route: test_sections.clone(),
            ..Default::default()
        };

        let exec_doc = test_doc.exec().await;
        assert_eq!(
            exec_doc.route,
            vec![
                ExecSection {
                    name: "test1".to_string(),
                    lines: vec![ExecLine {
                        section: 0,
                        index: 0,
                        map_coords: vec![GameCoord(1.2, 2.2, 3.3),],
                        line_color: "color".to_string(),
                        ..Default::default()
                    }],
                    map: ExecMapSection {
                        lines: vec![MapLine {
                            color: "color".to_string(),
                            points: vec![GameCoord(1.1, 2.2, 3.3), GameCoord(1.2, 2.2, 3.3),]
                        }],
                        ..Default::default()
                    }
                },
                ExecSection {
                    name: "test2".to_string(),
                    lines: vec![ExecLine {
                        section: 1,
                        index: 0,
                        map_coords: vec![GameCoord(1.3, 2.2, 3.3),],
                        line_color: "color".to_string(),
                        ..Default::default()
                    }],
                    map: ExecMapSection {
                        lines: vec![MapLine {
                            color: "color".to_string(),
                            points: vec![GameCoord(1.2, 2.2, 3.3), GameCoord(1.3, 2.2, 3.3),]
                        }],
                        ..Default::default()
                    }
                },
            ]
        );
    }
}
