use serde::{Deserialize, Serialize};

use crate::env::yield_budget;
use crate::json::{
    Coerce, RouteBlobArrayIterResult, RouteBlobError, RouteBlobRef,
    RouteBlobSingleKeyObjectResult,
};
use crate::lang::{self, DocRichText, IntoDiagnostic};
use crate::pack::PackError;

use super::{CompError, CompLine, Compiler};

/// Compiled Section
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompSection {
    /// Name of the section
    pub name: String,
    /// The lines in the section
    pub lines: Vec<CompLine>,
}

impl CompSection {
    pub fn from_diagnostic<T>(error: T) -> Self
    where
        T: IntoDiagnostic,
    {
        let line = CompLine {
            diagnostics: vec![error.into_diagnostic()],
            ..Default::default()
        };
        Self {
            name: "[error]".to_string(),
            lines: vec![line],
        }
    }
}

impl<'p> Compiler<'p> {
    pub fn compile_preface(&self, value: RouteBlobRef<'p>) -> Result<DocRichText, RouteBlobError> {
        let value = value.checked()?;
        let text = value.coerce_into_string();
        Ok(lang::parse_rich(&text))
    }
    /// Compile a blob into a section
    ///
    /// If value is a preface, returns `None`
    pub async fn compile_section(
        &self,
        value: RouteBlobRef<'p>,
        route: &Vec<CompSection>,
    ) -> Option<CompSection> {
        let result = match value.try_as_single_key_object() {
            RouteBlobSingleKeyObjectResult::Ok(key, value) => Ok((key, value)),
            RouteBlobSingleKeyObjectResult::Err(error) => {
                Err(PackError::BuildRouteSectionError(error).into_diagnostic())
            }
            RouteBlobSingleKeyObjectResult::Empty => {
                if route.is_empty() {
                    return None;
                } else {
                    Err(CompError::EmptyObjectCannotBeSection.into_diagnostic())
                }
            }
            RouteBlobSingleKeyObjectResult::TooManyKeys => {
                if route.is_empty() {
                    return None;
                } else {
                    Err(CompError::TooManyKeysInObjectSection.into_diagnostic())
                }
            }
            RouteBlobSingleKeyObjectResult::NotObject => {
                if route.is_empty() {
                    return None;
                } else {
                    Err(CompError::InvalidSectionType.into_diagnostic())
                }
            }
        };

        let (name, value) = match result {
            Ok(v) => v,
            Err(e) => {
                return Some(CompSection::from_diagnostic(e));
            }
        };

        let array = match value.try_as_array_iter() {
            RouteBlobArrayIterResult::Ok(v) => v,
            RouteBlobArrayIterResult::Err(e) => {
                return Some(CompSection::from_diagnostic(
                    PackError::BuildRouteSectionError(e),
                ));
            }
            RouteBlobArrayIterResult::NotArray => {
                return Some(CompSection::from_diagnostic(CompError::InvalidSectionType));
            }
        };

        let mut lines = vec![];
        for line in array {
            yield_budget(64).await;
            lines.push(self.parse_line(line));
        }

        let section = CompSection {
            name: name.to_owned(),
            lines,
        };

        Some(section)
    }

    // fn create_empty_line_for_error(&self, errors: &[CompError]) -> CompLine {
    //     let mut diagnostics = vec![];
    //     for error in errors {
    //         error.add_to_diagnostics(&mut diagnostics);
    //     }
    //     CompLine {
    //         text: parse_rich("[compile error]"),
    //         line_color: self.color.clone(),
    //         diagnostics,
    //         map_coord: self.coord.clone(),
    //         ..Default::default()
    //     }
    // }
}
//
// impl<'a> Compiler<'a> {
//
//
//
//     fn create_empty_section_for_error(&self, errors: &[CompError]) -> CompSection {
//         let mut diagnostics = vec![];
//         for error in errors {
//             error.add_to_diagnostics(&mut diagnostics);
//         }
//         let line = CompLine {
//             line_color: self.color.clone(),
//             diagnostics,
//             map_coord: self.coord.clone(),
//             ..Default::default()
//         };
//         CompSection {
//             name: "[compiler error]".to_string(),
//             lines: vec![line],
//         }
//     }
//
// }
