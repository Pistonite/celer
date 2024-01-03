use std::borrow::Cow;

use crate::lang::BaseError;

/// Error during the exec phase
///
/// These errors should never be fatal and should always
/// be displayed in the document to the user
#[derive(Debug, thiserror::Error)]
pub enum ExecError {
    #[error("Cannot find icon `{0}`")]
    IconNotFound(String),
}

impl BaseError for ExecError {
    fn source(&self) -> Cow<'static, str> {
        "celerc/exec".into()
    }

    fn is_error(&self) -> bool {
        match self {
            ExecError::IconNotFound(_) => false,
        }
    }

    fn help_path(&self) -> Option<Cow<'static, str>> {
        let path = match self {
            ExecError::IconNotFound(_) => "route/config/icons",
        };
        Some(format!("/docs/{path}").into())
    }
}
