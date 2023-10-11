use serde::{Deserialize, Serialize};

use crate::json::Cast;
use crate::lang::parse_rich;
use crate::pack::PackerValue;
use crate::util::async_for;

use super::{CompLine, Compiler, CompError};

/// Compiled Section
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompSection {
    /// Name of the section
    pub name: String,
    /// The lines in the section
    pub lines: Vec<CompLine>,
}

impl<'a> Compiler<'a> {
    pub async fn comp_section(&mut self, value: PackerValue) -> Result<CompSection, CompError> {
        if let PackerValue::Err(e) = value {
            return Err(CompError::PackerErrors(vec![e]));
        }
        if value.is_array() {
            return Err(CompError::InvalidSectionType);
        }

        let section_obj = match value.try_into_object() {
            Ok(v) => v,
            Err(v) => {
                // If not an array or object and is valid, treat as a preface value
                if let PackerValue::Ok(v) = v {
                    return Err(CompError::IsPreface(v));
                } else {
                    unreachable!();
                }
            }
        };

        let mut iter = section_obj.into_iter();
        let (section_name, section_value) = iter.next().ok_or(CompError::InvalidSectionType)?;
        if iter.next().is_some() {
            return Err(CompError::InvalidSectionType);
        }
        let mut section = CompSection {
            name: section_name,
            lines: vec![],
        };
        if let PackerValue::Err(e) = section_value {
            section.lines.push(
                self.create_empty_line_for_error(&[CompError::PackerErrors(vec![e])])
                    .await,
            );
            return Ok(section);
        }
        let section_lines = match section_value.try_into_array() {
            Ok(v) => v,
            Err(_) => {
                section.lines.push(
                    self.create_empty_line_for_error(&[CompError::InvalidSectionType])
                        .await,
                );
                return Ok(section);
            }
        };
        async_for!(line in section_lines, {
            match line.flatten().await {
                Ok(v) => {
                    let line = match self.comp_line(v).await {
                        Ok(l) => l,
                        Err((mut l, errors)) => {
                            async_for!(error in errors, {
                                error.add_to_diagnostics(&mut l.diagnostics);
                            })?;
                            l
                        }
                    };
                    section.lines.push(line);
                }
                Err(errors) => {
                    section.lines.push(self.create_empty_line_for_error(&[CompError::PackerErrors(errors)]).await);
                }
            }
        })?;

        Ok(section)
    }

    async fn create_empty_line_for_error(&self, errors: &[CompError]) -> CompLine {
        let mut diagnostics = vec![];
        // ignore if async loop fails
        let _ = async_for!(error in errors, {
            error.add_to_diagnostics(&mut diagnostics);
        });
        CompLine {
            text: parse_rich("[compile error]"),
            line_color: self.color.clone(),
            diagnostics,
            map_coord: self.coord.clone(),
            ..Default::default()
        }
    }
}
