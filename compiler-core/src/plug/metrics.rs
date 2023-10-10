use celerctypes::RouteMetadata;
use instant::Instant;
use serde_json::Value;

use crate::{macros::{async_trait, maybe_send}, comp::CompDoc, prop, json::Coerce};

use super::{PluginRuntime, PlugResult};

/// Metrics collected during compilation
pub struct MetricsPlugin {
    /// If detailed metrics of each phase should be given
    detailed: bool,
    /// Starting time
    last_start_time: Instant,
    /// Time spent during pack phase 0
    prep_time_ms: u64,
    /// Time spent during comp phase
    comp_time_ms: u64,
}

impl MetricsPlugin {
    pub fn from_props(props: &Value, start_time: &Instant) -> Self {
        let detailed = props.as_object().and_then(|m|m.get(prop::DETAILED)).map_or(false, |x| x.coerce_truthy());
        Self::new(detailed, start_time)
    }
    /// Create a new metrics plugin for measuring compilation time starting from `start_time`
    pub fn new(detailed: bool, start_time: &Instant) -> Self {
        if detailed {
            Self {
                detailed,
                prep_time_ms: start_time.elapsed().as_millis() as u64,
                last_start_time: Instant::now(),
                comp_time_ms: 0,
            }
        } else {
            Self {
                detailed,
                prep_time_ms: 0,
                last_start_time: start_time.clone(),
                comp_time_ms: 0,
            }
        }
    }
}

#[maybe_send(async_trait)]
impl PluginRuntime for MetricsPlugin {
    async fn on_compile(&mut self, doc: &mut CompDoc) -> PlugResult<()> {
        if self.detailed {
            let comp_time_ms = self.last_start_time.elapsed().as_millis() as u64;
            self.last_start_time = Instant::now();
            doc.project.stats.insert("Pack0 Time".to_string(), format!("{}ms", self.prep_time_ms));
            doc.project.stats.insert("Pack1+Comp Time".to_string(), format!("{comp_time_ms}ms"));
            self.comp_time_ms = comp_time_ms;
        }
        Ok(())
    }
    async fn on_post_compile(&mut self, project: &mut RouteMetadata) -> PlugResult<()> {
        let exec_time_ms = self.last_start_time.elapsed().as_millis() as u64;
        if self.detailed {
            project.stats.insert("Exec Time".to_string(), format!("{exec_time_ms}ms"));
            let total_ms = self.prep_time_ms + self.comp_time_ms + exec_time_ms;
            project.stats.insert("Compiled In".to_string(), format!("{total_ms}ms"));
        } else {
            project.stats.insert("Compiled In".to_string(), format!("{exec_time_ms}ms"));
        }
        Ok(())
    }
}
