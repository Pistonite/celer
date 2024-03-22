use std::borrow::Cow;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use instant::Instant;

use crate::lang::{DocDiagnostic, DocRichText, IntoDiagnostic};
use crate::pack::CompileContext;
use crate::plugin;
use crate::prep::{CompilerMetadata, PrepError, RouteConfig, RouteMetadata};

use super::CompSection;

/// Compiled Document, which is the output of the comp phase
pub struct CompDoc<'p> {
    pub ctx: CompileContext<'p>,
    /// The preface
    pub preface: Vec<DocRichText>,
    /// The route
    pub route: Vec<CompSection>,
    /// Overall diagnostics (that don't apply to any line)
    pub diagnostics: Vec<DocDiagnostic>,
    /// Properties that are marked as known by plugins
    pub known_props: HashSet<String>,
    /// Plugins
    ///
    /// CompDoc holds this to pass it to the next phase. It does not uses it directly.
    pub plugin_runtimes: Vec<plugin::BoxedRuntime>,
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
        let ctx = CompileContext {
            start_time,
            config: Cow::Owned(config),
            meta: Cow::Owned(CompilerMetadata::default()),
            plugins: vec![],
            plugin_meta: vec![],
            setting: &super::DEFAULT_SETTING,
        };

        Self::from_diagnostic(error, ctx)
    }
}

impl<'p> CompDoc<'p> {
    /// Create a new document showing an error from a single diagnostic.
    ///
    /// If the error happened before the context is even available, use `ExecDoc::from_diagnostic`
    /// instead.
    pub fn from_diagnostic<T>(error: T, ctx: CompileContext<'p>) -> Self
    where
        T: IntoDiagnostic,
    {
        Self {
            ctx,
            preface: Default::default(),
            route: Default::default(),
            diagnostics: vec![error.into_diagnostic()],
            known_props: Default::default(),
            plugin_runtimes: Default::default(),
        }
    }
}
