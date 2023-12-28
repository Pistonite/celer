use std::borrow::Cow;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use instant::Instant;
use serde::{Deserialize, Serialize};

use crate::json::{Cast, Coerce};
use crate::lang::parse_rich;
use crate::lang;
use crate::res::Loader;
use crate::pack::{PackerError, PackerValue, CompileContext, PackError};
use crate::prep::{PrepError, RouteConfig, RouteMetadata, CompilerMetadata, PreparedContext};
use crate::types::{DocDiagnostic, DocRichText};
use crate::util::yield_budget;

use super::{CompError, CompLine, CompSection, Compiler};

/// Compiled Document
pub struct CompDoc<'p> {
    ctx: CompileContext<'p>,
    /// The preface
    pub preface: Vec<DocRichText>,
    /// The route
    pub route: Vec<CompSection>,
    /// Overall diagnostics (that don't apply to any line)
    pub diagnostics: Vec<DocDiagnostic>,
    /// Properties that are marked as known by plugins
    pub known_props: HashSet<String>,
}

impl<'p> Deref for CompDoc<'p> {
    type Target = CompileContext<'p>;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl<'p> DerefMut for CompDoc<'p> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

impl<'p> AsRef<CompileContext<'p>> for CompDoc<'p> {
    fn as_ref(&self) -> &CompileContext<'p> {
        &self.ctx
    }
}

impl CompDoc<'static> {
    /// Create a new document showing an error from the prep phase.
    pub fn from_prep_error(error: PrepError, start_time: Instant) -> Self {
        let config = RouteConfig {
            meta: RouteMetadata {
                title: "[compile error]".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        Self {
            ctx: CompileContext {
                start_time,
                config: Cow::Owned(config),
                meta: Cow::Owned(CompilerMetadata::default()),
                setting: &super::DEFAULT_SETTING,
            },
            preface: Default::default(),
            route: Default::default(),
            diagnostics: vec![
                DocDiagnostic {
                    msg: lang::parse_poor(&error.to_string()),
                    msg_type: "error".to_string(),
                    source: "celerc/prep".to_string(),
                }
            ],
            known_props: Default::default(),
        }
    }
}

impl<'p> CompDoc<'p> {
    /// Create a new document showing an error from the pack phase.
    pub fn from_pack_error(error: PackError, ctx: CompileContext<'p>) -> Self
    {
        Self {
            ctx,
            preface: Default::default(),
            route: Default::default(),
            diagnostics: vec![
                DocDiagnostic {
                    msg: lang::parse_poor(&error.to_string()),
                    msg_type: "error".to_string(),
                    source: "celerc/pack".to_string(),
                }
            ],
            known_props: Default::default(),
        }
    }
}


impl<'a> Compiler<'a> {


    async fn add_section_or_preface(
        &mut self,
        preface: &mut Vec<DocRichText>,
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

}
