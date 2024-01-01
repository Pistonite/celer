use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use crate::macros::derive_wasm;
use crate::lang::{DocDiagnostic, DocRichText};
use crate::comp::CompDoc;
use crate::prep::RouteConfig;

use super::{MapBuilder, ExecSection};

/// The executed document
///
/// This is the final output of compiler with
/// map items separated from doc items
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct ExecDoc<'a> {
    /// Project metadata
    pub project: Cow<'a, RouteConfig>,
    /// The preface
    pub preface: Vec<DocRichText>,
    /// The route
    pub route: Vec<ExecSection>,
    /// Overall diagnostics (that don't apply to any line)
    pub diagnostics: Vec<DocDiagnostic>,
}


impl<'p> CompDoc<'p> {
    /// Execute the document
    pub async fn exec(self, project: &RouteConfig) -> ExecDoc<'_> {
        todo!()
        // let mut map_builder = MapBuilder::default();
        // map_builder.add_coord("", &project.map.initial_coord);
        // let mut sections = vec![];
        // for (index, section) in self.route.into_iter().enumerate() {
        //     let exec_section = section.exec(project, index, &mut map_builder).await;
        //     sections.push(exec_section);
        // }
        // ExecDoc {
        //     project: Cow::Borrowed(project),
        //     preface: self.preface,
        //     route: sections,
        //     diagnostics: self.diagnostics,
        // }
    }
}

#[cfg(test)]
mod test {
    use crate::comp::{CompLine, CompMovement, CompSection};
    use crate::lang::parse_poor;

    use super::*;

    #[tokio::test]
    async fn test_copy() {
        // let test_metadata = RouteConfig {
        //     source: "test".to_string(),
        //     version: "test version".to_string(),
        //     ..Default::default()
        // };
        //
        // let test_preface = vec![DocRichText::text("test")];
        //
        // let test_diagnostics = vec![DocDiagnostic {
        //     msg: parse_poor("test msg"),
        //     msg_type: "test".into(),
        //     source: "test".into(),
        // }];
        //
        // let test_doc = CompDoc {
        //     preface: test_preface.clone(),
        //     diagnostics: test_diagnostics.clone(),
        //     ..CompDoc::from_diagnostic
        // };
        //
        // let exec_doc = test_doc.exec(&test_metadata).await;
        // assert_eq!(exec_doc.project, Cow::Borrowed(&test_metadata));
        // assert_eq!(exec_doc.preface, test_preface);
        // assert_eq!(exec_doc.diagnostics, test_diagnostics);
        todo!()
    }

    #[tokio::test]
    async fn test_sections() {
        // let test_sections = vec![
        //     CompSection {
        //         name: "test1".to_string(),
        //         lines: vec![CompLine {
        //             movements: vec![CompMovement::to(GameCoord(1.2, 2.2, 3.3))],
        //             line_color: "color".to_string(),
        //             ..Default::default()
        //         }],
        //     },
        //     CompSection {
        //         name: "test2".to_string(),
        //         lines: vec![CompLine {
        //             movements: vec![CompMovement::to(GameCoord(1.3, 2.2, 3.3))],
        //             line_color: "color".to_string(),
        //             ..Default::default()
        //         }],
        //     },
        // ];
        //
        // let project = RouteMetadata {
        //     map: MapMetadata {
        //         initial_coord: GameCoord(1.1, 2.2, 3.3),
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // };
        //
        // let test_doc = CompDoc {
        //     route: test_sections.clone(),
        //     ..Default::default()
        // };
        //
        // let exec_doc = test_doc.exec(&project).await;
        // assert_eq!(
        //     exec_doc.route,
        //     vec![
        //         ExecSection {
        //             name: "test1".to_string(),
        //             lines: vec![ExecLine {
        //                 section: 0,
        //                 index: 0,
        //                 map_coords: vec![GameCoord(1.2, 2.2, 3.3),],
        //                 line_color: "color".to_string(),
        //                 ..Default::default()
        //             }],
        //             map: ExecMapSection {
        //                 lines: vec![MapLine {
        //                     color: "color".to_string(),
        //                     points: vec![GameCoord(1.1, 2.2, 3.3), GameCoord(1.2, 2.2, 3.3),]
        //                 }],
        //                 ..Default::default()
        //             }
        //         },
        //         ExecSection {
        //             name: "test2".to_string(),
        //             lines: vec![ExecLine {
        //                 section: 1,
        //                 index: 0,
        //                 map_coords: vec![GameCoord(1.3, 2.2, 3.3),],
        //                 line_color: "color".to_string(),
        //                 ..Default::default()
        //             }],
        //             map: ExecMapSection {
        //                 lines: vec![MapLine {
        //                     color: "color".to_string(),
        //                     points: vec![GameCoord(1.2, 2.2, 3.3), GameCoord(1.3, 2.2, 3.3),]
        //                 }],
        //                 ..Default::default()
        //             }
        //         },
        //     ]
        // );
        todo!()
    }
}
