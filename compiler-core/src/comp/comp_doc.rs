use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::json::{Cast, Coerce};
use crate::lang::parse_rich;
use crate::pack::{PackerError, PackerValue};
use crate::types::{DocDiagnostic, DocRichText};
use crate::util::yield_budget;

use super::{CompError, CompLine, CompSection, Compiler};

/// Compiled Document
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompDoc {
    /// The preface
    pub preface: Vec<Vec<DocRichText>>,
    /// The route
    pub route: Vec<CompSection>,
    /// Overall diagnostics (that don't apply to any line)
    pub diagnostics: Vec<DocDiagnostic>,
    /// Properties that are marked as known by plugins
    pub known_props: HashSet<String>,
}

impl<'a> Compiler<'a> {
    pub async fn comp_doc(&mut self, route: PackerValue) -> Result<CompDoc, CompError> {
        let mut route_vec = vec![];
        let mut preface = vec![];

        let mut errors: Vec<CompError> = vec![];

        match route.try_into_array() {
            Ok(sections) => {
                for value in sections.into_iter() {
                    yield_budget(64).await;
                    self.add_section_or_preface(&mut preface, &mut route_vec, value).await?;
                }
            }
            Err(_) => {
                errors.push(CompError::InvalidRouteType);
            }
        }

        if errors.is_empty() {
            Ok(CompDoc {
                preface,
                route: route_vec,
                diagnostics: vec![],
                known_props: Default::default(),
            })
        } else {
            Ok(self.create_empty_doc_for_error(&errors))
        }
    }

    async fn add_section_or_preface(
        &mut self,
        preface: &mut Vec<Vec<DocRichText>>,
        route: &mut Vec<CompSection>,
        value: PackerValue,
    ) -> Result<(), CompError> {
        match self.comp_section(value).await {
            Ok(section) => route.push(section),
            Err(e) => {
                if let CompError::IsPreface(v) = &e {
                    if route.is_empty() {
                        let text = v.coerce_to_string();
                        preface.push(parse_rich(&text));
                        return Ok(());
                    }
                }
                let section = self.create_empty_section_for_error(&[e]);
                route.push(section);
            }
        }
        Ok(())
    }

    fn create_empty_section_for_error(&self, errors: &[CompError]) -> CompSection {
        let mut diagnostics = vec![];
        for error in errors {
            error.add_to_diagnostics(&mut diagnostics);
        }
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

    pub fn create_empty_doc_for_packer_error(&self, error: PackerError) -> CompDoc {
        self.create_empty_doc_for_error(&[CompError::PackerErrors(vec![error])])
    }

    pub fn create_empty_doc_for_error(&self, errors: &[CompError]) -> CompDoc {
        CompDoc {
            route: vec![self.create_empty_section_for_error(errors)],
            preface: vec![],
            diagnostics: vec![],
            known_props: Default::default(),
        }
    }
}
