//! Built-in plugins
//!
//! Built-in plugins are implemented in Rust and directly included in the compiler.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::pack::CompileContext;

use super::{PluginResult, PluginRuntime};

mod botw_unstable;
mod export_livesplit;
mod link;
mod metrics;
mod split_format;
mod variables;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuiltInPlugin {
    BotwAbilityUnstable, // TODO #24: remove this
    #[serde(rename = "export-livesplit")]
    ExportLiveSplit,
    Link,
    Metrics,
    SplitFormat,
    Variables,
}

impl BuiltInPlugin {
    pub fn create_runtime(
        &self,
        ctx: &CompileContext<'_>,
        props: &Value,
    ) -> PluginResult<Box<dyn PluginRuntime>> {
        match &self {
            BuiltInPlugin::BotwAbilityUnstable => Ok(Box::new(
                botw_unstable::BotwAbilityUnstablePlugin::from_props(props),
            )),
            BuiltInPlugin::ExportLiveSplit => Ok(Box::new(export_livesplit::ExportLiveSplitPlugin)),
            BuiltInPlugin::Link => Ok(Box::new(link::LinkPlugin)),
            BuiltInPlugin::Metrics => Ok(Box::new(metrics::MetricsPlugin::from_props(
                props,
                &ctx.start_time,
            ))),
            BuiltInPlugin::SplitFormat => {
                Ok(Box::new(split_format::SplitFormatPlugin::from_props(props)))
            }
            BuiltInPlugin::Variables => Ok(Box::new(variables::VariablesPlugin::from_props(props))),
        }
    }

    pub fn id(&self) -> String {
        serde_json::to_string(self)
            .map(|x| x.trim_matches('"').to_string())
            .unwrap_or_default()
    }
}
