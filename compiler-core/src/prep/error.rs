use std::borrow::Cow;

use crate::lang::BaseError;
use crate::res::ResError;

use super::ConfigTrace;

/// Error during the prep phase
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum PrepError {
    #[error("Failed to load resource: {0}")]
    Res(#[from] ResError),

    #[error("The project file `{0}` should be a mapping object")]
    InvalidProjectResourceType(String),

    #[error("Project config ({0}) should be a mapping object")]
    InvalidConfigType(ConfigTrace),

    #[error("Project config ({0}): property `{1}` has an invalid type (expected {2})")]
    InvalidConfigPropertyType(ConfigTrace, Cow<'static, str>, Cow<'static, str>),

    #[error("Project config ({0}): property `{1}` is missing")]
    MissingConfigProperty(ConfigTrace, Cow<'static, str>),

    #[error("Project config ({0}): the `{1}` property is unused")]
    UnusedConfigProperty(ConfigTrace, Cow<'static, str>),

    #[error("Project config ({0}): cannot find tag `{1}`")]
    TagNotFound(ConfigTrace, Cow<'static, str>),

    #[error("Project config ({0}): `{1}` is not a valid built-in plugin or reference to a plugin script")]
    InvalidPlugin(ConfigTrace, String),

    #[error("Project config ({0}): config is nesting too deep! Check that you don't have circular dependency, or simplify the config structure")]
    MaxConfigDepthExceeded(ConfigTrace),

    #[error("Project config ({0}): defining map when a previous config already defines one")]
    DuplicateMap(ConfigTrace),

    #[error("Max preset namespace depth of {0} levels is reached. There might be a formatting error in your project files. If this is intentional, consider making the namespaces less complex.")]
    MaxPresetNamespaceDepthExceeded(usize),

    #[error("Project config ({0}): preset {1} is invalid")]
    InvalidPreset(ConfigTrace, String),

    #[error("Project metadata property `{0}` has invalid type (expecting {1})")]
    InvalidMetadataPropertyType(&'static str, &'static str),

    #[error("Project metadata has extra unused property: {0}")]
    UnusedMetadataProperty(String),

    #[error("Entry point `{0}` is invalid: `{1}` is neither an absolute path, nor a name of another entry point.")]
    InvalidEntryPoint(String, String),

    #[error("Entry point `{0}` is nesting too deep! Do you have a recursive loop?")]
    MaxEntryPointDepthExceeded(String),
}

pub type PrepResult<T> = Result<T, PrepError>;

impl BaseError for PrepError {
    fn source(&self) -> Cow<'static, str> {
        "celerc/prep".into()
    }

    fn is_error(&self) -> bool {
        true
    }

    fn help_path(&self) -> Option<Cow<'static, str>> {
        let path = match self {
            PrepError::MaxConfigDepthExceeded(_)
            | PrepError::MaxPresetNamespaceDepthExceeded(_)
            | PrepError::MaxEntryPointDepthExceeded(_) => return None,
            PrepError::InvalidProjectResourceType(_) => "route/hello-world",
            PrepError::Res(_) => "route/file-structure",
            PrepError::InvalidConfigType(_) => "route/configuration#example",
            PrepError::InvalidConfigPropertyType(_, _, _)
            | PrepError::MissingConfigProperty(_, _)
            | PrepError::UnusedConfigProperty(_, _)
            | PrepError::InvalidMetadataPropertyType(_, _)
            | PrepError::UnusedMetadataProperty(_) => "route/configuration",
            PrepError::TagNotFound(_, _) => "route/config/tags",
            PrepError::InvalidPlugin(_, _) => "route/config/plugins",
            PrepError::DuplicateMap(_) => "route/config/map",
            PrepError::InvalidPreset(_, _) => "route/config/preset",
            PrepError::InvalidEntryPoint(_, _) => {
                "route/file-structure#multiple-projects-in-the-same-repo"
            }
        };

        Some(format!("/docs/{path}").into())
    }
}
