use std::borrow::Cow;

use crate::comp::CompDoc;
use crate::lang::{DocDiagnostic, DocRichText};
use crate::macros::derive_wasm;
use crate::prep::RouteConfig;

use super::{ExecSection, MapBuilder};

/// The executed document
///
/// This is the final output of compiler with
/// map items separated from doc items
#[derive(Default, Debug, Clone)]
#[derive_wasm]
pub struct ExecDoc<'p> {
    /// Project metadata
    pub project: Cow<'p, RouteConfig>,
    /// The preface
    pub preface: Vec<DocRichText>,
    /// The route
    pub route: Vec<ExecSection>,
    /// Overall diagnostics (that don't apply to any line)
    pub diagnostics: Vec<DocDiagnostic>,
}

impl<'p> CompDoc<'p> {
    /// Execute the document
    pub async fn execute_document(self) -> ExecDoc<'p> {
        let route_config = self.ctx.config.as_ref();
        let mut map_builder = match &route_config.map {
            Some(map) => MapBuilder::new(map.initial_color.to_string(), map.initial_coord.clone()),
            None => MapBuilder::default(),
        };
        let mut sections = vec![];
        for (index, section) in self.route.into_iter().enumerate() {
            let exec_section = section.exec(route_config, index, &mut map_builder).await;
            sections.push(exec_section);
        }
        ExecDoc {
            project: self.ctx.config,
            preface: self.preface,
            route: sections,
            diagnostics: self.diagnostics,
        }
    }
}

#[cfg(test)]
mod test {
    use instant::Instant;

    use crate::comp::{CompLine, CompMovement, CompSection};
    use crate::exec::{ExecLine, MapLine, MapSection};
    use crate::lang::parse_poor;
    use crate::pack::CompileContext;
    use crate::prep::{GameCoord, MapMetadata, RouteMetadata, Setting};

    use super::*;

    #[tokio::test]
    async fn test_copy() {
        let test_metadata = RouteConfig {
            meta: RouteMetadata {
                source: "test".to_string(),
                version: "test version".to_string(),
                title: "test title".to_string(),
            },
            ..Default::default()
        };

        let test_preface = vec![DocRichText::text("test")];

        let test_diagnostics = vec![DocDiagnostic {
            msg: parse_poor("test msg"),
            msg_type: "test".into(),
            source: "test".into(),
        }];

        let setting = Setting::default();

        let test_doc = CompDoc {
            ctx: CompileContext {
                config: Cow::Borrowed(&test_metadata),
                setting: &setting,
                meta: Cow::Owned(Default::default()),
                start_time: Instant::now(),
            },
            preface: test_preface.clone(),
            diagnostics: test_diagnostics.clone(),
            route: Default::default(),
            known_props: Default::default(),
            plugin_runtimes: Default::default(),
        };

        let exec_doc = test_doc.execute_document().await;
        assert_eq!(exec_doc.project, Cow::Borrowed(&test_metadata));
        assert_eq!(exec_doc.preface, test_preface);
        assert_eq!(exec_doc.diagnostics, test_diagnostics);
    }

    #[tokio::test]
    async fn test_sections() {
        let test_sections = vec![
            CompSection {
                name: "test1".to_string(),
                lines: vec![CompLine {
                    movements: vec![CompMovement::to(GameCoord(1.2, 2.2, 3.3))],
                    line_color: Some("color".to_string()),
                    ..Default::default()
                }],
            },
            CompSection {
                name: "test2".to_string(),
                lines: vec![CompLine {
                    movements: vec![CompMovement::to(GameCoord(1.3, 2.2, 3.3))],
                    line_color: Some("color".to_string()),
                    ..Default::default()
                }],
            },
        ];

        let project = RouteConfig {
            map: Some(MapMetadata {
                initial_coord: GameCoord(1.1, 2.2, 3.3),
                ..Default::default()
            }),
            ..Default::default()
        };

        let setting = Setting::default();

        let test_doc = CompDoc {
            ctx: CompileContext {
                config: Cow::Borrowed(&project),
                setting: &setting,
                meta: Cow::Owned(Default::default()),
                start_time: Instant::now(),
            },
            preface: Default::default(),
            diagnostics: Default::default(),
            route: test_sections,
            known_props: Default::default(),
            plugin_runtimes: Default::default(),
        };

        let exec_doc = test_doc.execute_document().await;
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
                    map: MapSection {
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
                    map: MapSection {
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
