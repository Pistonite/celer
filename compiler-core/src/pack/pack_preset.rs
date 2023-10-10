//! Packs the `presets` property in a config json blob to
//! list of [`Preset`]s

use serde_json::Value;

use crate::json::Cast;
use crate::lang::Preset;
use crate::macros::{maybe_send, async_recursion};
use crate::prop;
use crate::util::async_for;

use super::{PackerError, PackerResult};

pub async fn pack_presets(
    value: Value,
    index: usize,
    max_depth: usize,
) -> PackerResult<Vec<(String, Preset)>> {
    let mut output = Vec::new();
    pack_presets_internal("", value, index, 0, max_depth, &mut output).await?;
    Ok(output)
}

#[maybe_send(async_recursion)]
async fn pack_presets_internal(
    preset_name: &str,
    value: Value,
    index: usize,
    depth: usize,
    max_depth: usize,
    output: &mut Vec<(String, Preset)>,
) -> PackerResult<()> {
    if depth > max_depth {
        return Err(PackerError::MaxPresetNamespaceDepthExceeded(max_depth));
    }

    let obj = value.try_into_object().map_err(|_| {
        if preset_name.is_empty() {
            PackerError::InvalidConfigProperty(index, prop::PRESETS.to_string())
        } else {
            PackerError::InvalidPreset(index, preset_name.to_string())
        }
    })?;

    let _ = async_for!((key, value) in obj.into_iter(), {
        if let Some(namespace) = key.strip_prefix('_') {
            // sub namespace
            let full_key = if preset_name.is_empty() {
                format!("_{namespace}")
            } else {
                format!("{preset_name}::{namespace}")
            };
            pack_presets_internal(&full_key, value, index, depth+1, max_depth, output).await?;
        } else {
            // preset
            let full_key = if preset_name.is_empty() {
                format!("_{key}")
            } else {
                format!("{preset_name}::{key}")
            };
            let preset = Preset::compile(value).await.ok_or_else(|| {
                PackerError::InvalidPreset(index, full_key.clone())
            })?;
            output.push((full_key, preset));
        }
    });

    Ok(())
}
