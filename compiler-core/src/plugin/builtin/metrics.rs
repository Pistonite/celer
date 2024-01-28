use std::borrow::Cow;

use instant::Instant;
use serde_json::Value;

use crate::comp::CompDoc;
use crate::exec::ExecDoc;
use crate::json::Coerce;
use crate::prop;

use crate::plugin::{PluginResult, PluginRuntime};

/// Metrics collected during compilation
pub struct MetricsPlugin {
    /// If detailed metrics of each phase should be given
    detailed: bool,
    /// Starting time of the current phase being measured
    last_start_time: Instant,
    /// Time spent before comp phase starts (prep+pack)
    before_comp_time_ms: u64,
    /// Time spent during comp phase
    comp_time_ms: u64,
}

impl MetricsPlugin {
    pub fn from_props(props: &Value, start_time: &Instant) -> Self {
        let detailed = props
            .as_object()
            .and_then(|m| m.get(prop::DETAILED))
            .map_or(false, |x| x.coerce_truthy());
        Self::new(detailed, start_time)
    }
    /// Create a new metrics plugin for measuring compilation time starting from `start_time`
    pub fn new(detailed: bool, start_time: &Instant) -> Self {
        if detailed {
            Self {
                detailed,
                before_comp_time_ms: start_time.elapsed().as_millis() as u64,
                last_start_time: Instant::now(),
                comp_time_ms: 0,
            }
        } else {
            Self {
                detailed,
                before_comp_time_ms: 0,
                last_start_time: start_time.to_owned(),
                comp_time_ms: 0,
            }
        }
    }
}

impl PluginRuntime for MetricsPlugin {
    fn on_after_compile(&mut self, _: &mut CompDoc) -> PluginResult<()> {
        // measure time since plugin created = comp phase time
        if self.detailed {
            self.comp_time_ms = self.last_start_time.elapsed().as_millis() as u64;
            self.last_start_time = Instant::now();
        }
        Ok(())
    }
    fn on_after_execute(&mut self, doc: &mut ExecDoc) -> PluginResult<()> {
        // measure time since comp finished = exec time
        let exec_time_ms = self.last_start_time.elapsed().as_millis() as u64;
        let project = doc.project.to_mut();

        // add time to statistics
        if self.detailed {
            project.stats.insert(
                "Prep Time".to_string(),
                format!("{}ms", self.before_comp_time_ms),
            );
            project
                .stats
                .insert("Comp Time".to_string(), format!("{}ms", self.comp_time_ms));
            project
                .stats
                .insert("Exec Time".to_string(), format!("{exec_time_ms}ms"));
            let total_ms = self.before_comp_time_ms + self.comp_time_ms + exec_time_ms;
            project
                .stats
                .insert("Compiled In".to_string(), format!("{total_ms}ms"));
        } else {
            project
                .stats
                .insert("Compiled In".to_string(), format!("{exec_time_ms}ms"));
        }
        Ok(())
    }

    fn get_id(&self) -> Cow<'static, str> {
        Cow::Owned(super::BuiltInPlugin::Metrics.id())
    }
}
