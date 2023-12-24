
use serde_json::Value;

use crate::json::Cast;
use crate::lang::Preset;
use crate::macros::async_recursion;
use crate::prop;
use crate::env::yield_budget;
use crate::prep::{PrepResult, PrepError};

use super::PreparedConfig;

impl PreparedConfig {
    /// Load the `presets` property
    pub async fn load_presets(&mut self, value: Value) -> PrepResult<()> {
        self.load_presets_internal("", value, 0).await
    }

    /// Recursively load presets helper
    #[async_recursion(auto)]
    async fn load_presets_internal(
        &mut self,
        preset_name: &str,
        value: Value,
        depth: usize,
    ) -> PrepResult<()> {
        if depth > self.setting.max_preset_namespace_depth {
            return Err(PrepError::MaxPresetNamespaceDepthExceeded(self.setting.max_preset_namespace_depth));
        }

        let obj = value.try_into_object().map_err(|_| {
            if preset_name.is_empty() {
                PrepError::InvalidConfigPropertyType(
                    self.trace.clone(), 
                    prop::PRESETS.into(),
                    prop::PRESETS.into()
                )
            } else {
                PrepError::InvalidPreset(self.trace.clone(), preset_name.to_string())
            }
        })?;

        for (key, value) in obj.into_iter() {
            yield_budget(128).await;
            if let Some(namespace) = key.strip_prefix('_') {
                // sub namespace
                let full_key = format_preset_str(preset_name, namespace);
                self.load_presets_internal(&full_key, value, depth + 1).await?;
            } else {
                // preset
                let full_key = format_preset_str(preset_name, &key);
                let preset = Preset::compile(value)
                    .ok_or_else(|| PrepError::InvalidPreset(self.trace.clone(), full_key.clone()))?;
                self.presets.insert(full_key, preset);
            }
        }

        Ok(())
    }
}

#[inline]
fn format_preset_str(namespace: &str, key: &str) -> String {
    if namespace.is_empty() {
        format!("_{key}")
    } else {
        format!("{namespace}::{key}")
    }
}
