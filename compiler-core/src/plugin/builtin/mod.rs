//! Built-in plugins
//!
//! Built-in plugins are implemented in Rust and directly included in the compiler.

use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::pack::CompileContext;

use super::{PluginRuntime, PluginResult};

mod variables;
mod botw_unstable;
mod compat;
mod link;
mod metrics;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuiltInPlugin {
    Metrics,
    Link,
    Variables,
    // Compat,
    BotwAbilityUnstable,
}

impl BuiltInPlugin {
    pub fn create_runtime<'a>(&self, ctx: &CompileContext<'a>, props: &Value) -> PluginResult<Box<dyn PluginRuntime>> {
        match &self{
            BuiltInPlugin::Link => Ok(Box::new(link::LinkPlugin)),
            BuiltInPlugin::Metrics => Ok(Box::new(metrics::MetricsPlugin::from_props(
                props,
                &ctx.start_time,
            ))),
            BuiltInPlugin::Variables => {
                Ok(Box::new(variables::VariablesPlugin::from_props(props)))
            }
            // BuiltInPlugin::Compat => Box::new(compat::CompatPlugin),
            BuiltInPlugin::BotwAbilityUnstable => Ok(Box::new(
                botw_unstable::BotwAbilityUnstablePlugin::from_props(props),
            )),
        }
    }
}

