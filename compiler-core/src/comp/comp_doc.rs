use celerctypes::{RouteMetadata, DocPoorText};
use serde::{Serialize, Deserialize};

use crate::lang::parse_poor;
use crate::pack::PackerValue;
use crate::json::Coerce;
use crate::util::async_for;

use super::{CompLine, CompSection, Compiler, CompilerError};


/// Compiled Document
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompDoc {
    /// Project metadata
    pub project: RouteMetadata,
    /// The preface
    pub preface: Vec<Vec<DocPoorText>>,
    /// The route
    pub route: Vec<CompSection>,
}

impl Compiler {
    pub async fn comp_doc(
        mut self,
        route: Vec<PackerValue>,
    ) -> CompDoc {
        let mut route_vec = vec![];
        let mut preface = vec![];

        async_for!(value in route.into_iter(), {
            self.add_section_or_preface(&mut preface, &mut route_vec, value).await;
        });

        CompDoc {
            project: self.project,
            preface,
            route: route_vec
        }
    }

    async fn add_section_or_preface(
        &mut self, 
        preface: &mut Vec<Vec<DocPoorText>>,
        route: &mut Vec<CompSection>,
        value: PackerValue) 
    {
        match self.comp_section(value).await {
            Ok(section) => route.push(section),
            Err(e) => {
                if let CompilerError::IsPreface(v) = &e {
                    if route.is_empty() {
                        let text = v.coerce_to_string();
                        preface.push(parse_poor(&text));
                        return;
                    }
                } 
                let section = self.create_empty_section_for_error(&[e]).await;
                route.push(section);
            }
        }
    }

    async fn create_empty_section_for_error(&self, errors: &[CompilerError]) -> CompSection {
        let mut diagnostics = vec![];
        async_for!(error in errors, {
            error.add_to_diagnostics(&mut diagnostics);
        });
        let line = CompLine {
            line_color: self.color.clone(),
            diagnostics, 
            map_coord: self.coord.clone(),
            ..Default::default()
        };
        CompSection {
            name: "[compiler error]".to_string(),
            lines: vec![line],
        }
    }
}

