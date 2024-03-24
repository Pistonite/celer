//! Built-in plugins
//!
//! Built-in plugins are implemented in Rust and directly included in the compiler.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::pack::CompileContext;

use super::{BoxedEarlyRuntime, BoxedRuntime, PluginResult};

mod botw_unstable;
mod export_livesplit;
mod link;
mod metrics;
mod split_format;
mod variables;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Native {
    BotwAbilityUnstable, // TODO #24: remove this
    #[serde(rename = "export-livesplit")]
    ExportLiveSplit,
    Link,
    Metrics,
    SplitFormat,
    Variables,
}

impl Native {
    pub fn create_early_runtime(&self) -> PluginResult<BoxedEarlyRuntime> {
        Ok(Box::new(super::DefaultEarlyRuntime))
    }

    pub fn create_runtime(
        &self,
        ctx: &CompileContext<'_>,
        props: &Value,
    ) -> PluginResult<BoxedRuntime> {
        match self {
            Self::BotwAbilityUnstable => Ok(Box::new(
                botw_unstable::BotwAbilityUnstable::from_props(props),
            )),
            Self::ExportLiveSplit => Ok(Box::new(export_livesplit::ExportLiveSplit)),
            Self::Link => Ok(Box::new(link::Link)),
            Self::Metrics => Ok(Box::new(metrics::Metrics::from_props(
                props,
                &ctx.start_time,
            ))),
            Self::SplitFormat => Ok(Box::new(split_format::SplitFormat::from_props(props))),
            Self::Variables => Ok(Box::new(variables::Variables::from_props(props))),
        }
    }

    pub fn id(&self) -> String {
        serde_json::to_string(self)
            .map(|x| x.trim_matches('"').to_string())
            .unwrap_or_default()
    }
}
