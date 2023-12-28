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

use serde::{Deserialize, Serialize};

use crate::macros::derive_wasm;
use crate::lang::{self, DocPoorText};

/// One diagnostic message
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
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

pub trait IntoDiagnostic: Display + Sized {
    /// Convert self into a diagnostic message, by using
    /// the string representation as the message
    fn into_diagnostic(self) -> DocDiagnostic {
        if self.is_error() {
            DocDiagnostic::error(&self.to_string(), self.source())
        } else {
            DocDiagnostic::warning(&self.to_string(), self.source())
        }
    }

    /// Get the source
    fn source(&self) -> Cow<'static, str>;

    /// If the error is an error (or warning otherwise)
    fn is_error(&self) -> bool;
}

// impl From<PrepError> for DocDiagnostic {
//     fn from(error: PrepError) -> Self {
//         Self::error(&error.to_string(), "celerc/prep")
//     }
// }
//
// impl From<PackError> for DocDiagnostic {
//     fn from(error: PackError) -> Self {
//         Self::error(&error.to_string(), "celerc/pack")
//     }
// }
