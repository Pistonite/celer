//! Built-in plugins
//!
//! Built-in plugins are implemented in Rust and directly included in the compiler.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::pack::CompileContext;

use super::{PluginResult, PluginRuntime};

mod botw_unstable;
mod link;
mod metrics;
mod variables;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuiltInPlugin {
    Metrics,
    Link,
    Variables,
    BotwAbilityUnstable,
}

impl BuiltInPlugin {
    pub fn create_runtime<'a>(
        &self,
        ctx: &CompileContext<'a>,
        props: &Value,
    ) -> PluginResult<Box<dyn PluginRuntime>> {
        match &self {
            BuiltInPlugin::Link => Ok(Box::new(link::LinkPlugin)),
            BuiltInPlugin::Metrics => Ok(Box::new(metrics::MetricsPlugin::from_props(
                props,
                &ctx.start_time,
            ))),
            BuiltInPlugin::Variables => Ok(Box::new(variables::VariablesPlugin::from_props(props))),
            BuiltInPlugin::BotwAbilityUnstable => Ok(Box::new(
                botw_unstable::BotwAbilityUnstablePlugin::from_props(props),
            )),
        }
    }
}
