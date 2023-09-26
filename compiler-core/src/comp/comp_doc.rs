use celerctypes::{DocPoorText, RouteMetadata};
use serde::{Deserialize, Serialize};

use crate::api::CompilerMetadata;
use crate::json::Cast;
use crate::json::Coerce;
use crate::lang::parse_poor;
use crate::pack::PackerError;
use crate::pack::PackerValue;
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
    pub async fn comp_doc(mut self, route: PackerValue) -> Result<(CompDoc, CompilerMetadata), CompilerError> {
        let mut route_vec = vec![];
        let mut preface = vec![];

        let mut errors: Vec<CompilerError> = vec![];

        match route.try_into_array() {
            Ok(sections) => {
                async_for!(value in sections.into_iter(), {
                    self.add_section_or_preface(&mut preface, &mut route_vec, value).await?;
                })?;
            }
            Err(_) => {
                errors.push(CompilerError::InvalidRouteType);
            }
        }

        if errors.is_empty() {
            Ok((
                CompDoc {
                    project: self.project,
                    preface,
                    route: route_vec,
                },
                self.meta,
            ))
        } else {
            Ok(self.create_empty_doc_for_error(&errors).await)
        }
    }

    async fn add_section_or_preface(
        &mut self,
        preface: &mut Vec<Vec<DocPoorText>>,
        route: &mut Vec<CompSection>,
        value: PackerValue,
    ) -> Result<(), CompilerError> {
        match self.comp_section(value).await {
            Ok(section) => route.push(section),
            Err(e) => {
                if e.is_cancel() {
                    return Err(e);
                }
                if let CompilerError::IsPreface(v) = &e {
                    if route.is_empty() {
                        let text = v.coerce_to_string();
                        preface.push(parse_poor(&text));
                        return Ok(());
                    }
                }
                let section = self.create_empty_section_for_error(&[e]).await;
                route.push(section);
            }
        }
        Ok(())
    }

    async fn create_empty_section_for_error(&self, errors: &[CompilerError]) -> CompSection {
        let mut diagnostics = vec![];
        let _: Result<(), _> = async_for!(error in errors, {
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

    pub async fn create_empty_doc_for_packer_error(
        self,
        error: PackerError
    ) -> (CompDoc, CompilerMetadata) {
        self.create_empty_doc_for_error(&[CompilerError::PackerErrors(vec![error])]).await
    }

    pub async fn create_empty_doc_for_error(
        self,
        errors: &[CompilerError],
    ) -> (CompDoc, CompilerMetadata) {
        (
            CompDoc {
                route: vec![self.create_empty_section_for_error(errors).await],
                project: self.project,
                preface: vec![],
            },
            self.meta,
        )
    }
}
