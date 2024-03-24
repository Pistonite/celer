use std::borrow::Cow;

use crate::lang::BaseError;

/// Error during plugin execution
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum PluginError {
    #[error("An exception occured while executing script: {0}")]
    ScriptException(String),

    #[error("Extra plugin at `{0}` from plugin options is invalid: {1}")]
    InvalidAddPlugin(usize, String),

    #[error("The plugin `{0}` does not implement the required `{1}` method!")]
    NotImplemented(String, String),

    #[error("The plugin `{0}` is duplicated. Please remove the duplicate(s) or refer to the documentation for advanced options.")]
    Duplicate(String),
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
        let path = match self {
            Self::InvalidAddPlugin(_, _) => "plugin/settings",
            Self::Duplicate(_) => "plugin/getting-started#allow-duplicates",
            _ => return None,
        };
        Some(format!("/docs/{path}").into())
    }
}
