use std::borrow::Cow;

use crate::lang::BaseError;

/// Error during plugin execution
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum PluginError {
    #[error("An exception occured while executing script: {0}")]
    ScriptException(String),
}

pub type PluginResult<T> = Result<T, PluginError>;

impl BaseError for PluginError {
    fn source(&self) -> Cow<'static, str> {
        "celerc/plugins".into()
    }

    fn is_error(&self) -> bool {
        true
    }

    fn help_path(&self) -> Option<Cow<'static, str>> {
        None
    }
}
