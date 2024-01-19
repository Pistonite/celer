//! Diagnostics API
//!
//! The diagnostics API provides a UI for displaying errors, warnings, and other messages.
//! It is used by the compiler to embed errors message in the route. Plugins may
//! also use this to customize the route
//!
//! There are 2 places to attach diagnostics:
//! - The route itself, showing at the beginning of the route, before the preface
//! - Each line, showing after each line

use std::borrow::Cow;
use std::fmt::Display;

use crate::env;
use crate::lang::{self, DocPoorText};
use crate::macros::derive_wasm;

/// One diagnostic message
#[derive(PartialEq, Default, Debug, Clone)]
#[derive_wasm]
pub struct DocDiagnostic {
    /// The diagnostic message. Poor text is used to automatically make links in the message
    /// clickable
    pub msg: DocPoorText,

    /// Type of the diagnostic
    ///
    /// The builtin ones are "error" and "warning", but this can be any value.
    /// Custom themes might utilize this for displaying extra messages.
    #[serde(rename = "type")]
    pub msg_type: Cow<'static, str>,

    /// Source of the diagnostic
    ///
    /// User can filter diagnostics by source
    pub source: Cow<'static, str>,
}

impl DocDiagnostic {
    /// Create a diagnostic message with "error" type
    pub fn error<TSource>(msg: &str, source: TSource) -> Self
    where
        TSource: Into<Cow<'static, str>>,
    {
        Self {
            msg: lang::parse_poor(msg),
            msg_type: Cow::Borrowed("error"),
            source: source.into(),
        }
    }

    /// Create a diagnostic message with "warning" type
    pub fn warning<TSource>(msg: &str, source: TSource) -> Self
    where
        TSource: Into<Cow<'static, str>>,
    {
        Self {
            msg: lang::parse_poor(msg),
            msg_type: Cow::Borrowed("warning"),
            source: source.into(),
        }
    }
}

pub trait IntoDiagnostic {
    fn into_diagnostic(self) -> DocDiagnostic;
}

pub trait BaseError: Display {
    /// Get the source
    fn source(&self) -> Cow<'static, str>;

    /// If the error is an error (or warning otherwise)
    fn is_error(&self) -> bool;

    /// An optional path to a help page, should be
    fn help_path(&self) -> Option<Cow<'static, str>>;
}

impl<T> IntoDiagnostic for T
where
    T: BaseError,
{
    /// Convert self into a diagnostic message, by using
    /// the string representation as the message
    fn into_diagnostic(self) -> DocDiagnostic {
        let message = match self.help_path() {
            Some(path) => {
                let site_origin = env::get_site_origin();
                let mut msg = format!("{self} See {site_origin}");
                if !path.starts_with('/') {
                    msg.push('/');
                }
                msg.push_str(&path);
                msg.push_str(" for more info.");
                msg
            }
            None => self.to_string(),
        };
        if self.is_error() {
            DocDiagnostic::error(&message, self.source())
        } else {
            DocDiagnostic::warning(&message, self.source())
        }
    }
}

impl IntoDiagnostic for DocDiagnostic {
    fn into_diagnostic(self) -> Self {
        self
    }
}
