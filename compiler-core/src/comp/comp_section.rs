use serde::{Deserialize, Serialize};

use crate::json::{Coerce, RouteBlobRef, Cast, RouteBlobError, RouteBlobSingleKeyObjectResult, RouteBlobArrayIterResult};
use crate::lang::{self, DocRichText, DocDiagnostic, IntoDiagnostic};
use crate::pack::{PackError};
use crate::env::{iter_futures, yield_budget};

use super::{CompError, CompLine, Compiler, CompResult};

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
        T: IntoDiagnostic
    {
        let mut line = CompLine {
            diagnostics: vec![error.into_diagnostic()],
            ..Default::default()
        };
        Self {
            name: "[error]".to_string(),
            lines: vec![line],
        }
    }

    // pub async fn sequential_pass(&mut self, compiler: &mut Compiler<'_>) {
    //     for line in &mut self.lines {
    //         yield_budget(64).await;
    //         line.sequential_pass(compiler);
    //     }
    // }
}

impl<'p> Compiler<'p> {
    pub fn compile_preface(
        &self,
        value: RouteBlobRef<'p>,
    ) -> Result<DocRichText, RouteBlobError> {
        let value = value.checked()?;
        let text = value.coerce_into_string();
        Ok(lang::parse_rich(&text))
    }
    /// Compile a blob into a section and add to the route
    ///
    /// If value is a preface, returns `None`
    pub async fn compile_section(&self, value: RouteBlobRef<'p>, route: &mut Vec<CompSection>, diagnostics: &mut Vec<DocDiagnostic>) -> Option<CompSection> {
        let result = match value.try_as_single_key_object() {
            RouteBlobSingleKeyObjectResult::Ok(key, value) => {
                Ok((key, value))
            }
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
                return Some(CompSection::from_diagnostic(PackError::BuildRouteSectionError(e)));
            }
            RouteBlobArrayIterResult::NotArray => {
                return Some(CompSection::from_diagnostic(CompError::InvalidSectionType));
            }
        };


        let lines = iter_futures(64, array.map(|line| async { self.parse_line(line) })).await;
        let section = CompSection {
            name: name.to_owned(),
            lines
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
