use std::borrow::Cow;

use crate::json::RouteBlobError;
use crate::lang::BaseError;
use crate::plugin::PluginError;

/// Error during the pack phase
#[derive(Debug, thiserror::Error)]
pub enum PackError {
    #[error("Failed to initialize plugins: {0}")]
    PluginInitError(#[from] PluginError),

    #[error("Failed to build route: {0}")]
    BuildRouteError(RouteBlobError),

    #[error("Failed to build route section: {0}")]
    BuildRouteSectionError(RouteBlobError),

    #[error("Failed to build route line: {0}")]
    BuildRouteLineError(RouteBlobError),
}

pub type PackResult<T> = Result<T, PackError>;

impl BaseError for PackError {
    fn source(&self) -> Cow<'static, str> {
        "celerc/pack".into()
    }

    fn is_error(&self) -> bool {
        true
    }

    fn help_path(&self) -> Option<Cow<'static, str>> {
        let path = match self {
            PackError::PluginInitError(e) => return e.help_path(),
            PackError::BuildRouteError(_) => "route/route-structure",
            PackError::BuildRouteSectionError(_) => "route/route-structure#sections",
            PackError::BuildRouteLineError(_) => "route/route-structure#lines",
        };
        Some(format!("/docs/{path}").into())
    }
}
