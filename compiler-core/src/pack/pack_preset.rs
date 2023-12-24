//! Packs the `presets` property in a config json blob to
//! list of [`Preset`]s

use serde_json::Value;

use crate::json::Cast;
use crate::lang::Preset;
use crate::macros::async_recursion;
use crate::prop;
use crate::env::yield_budget;

// use super::{ConfigTrace, PackerError, PackerResult};

// pub async fn pack_presets(
//     value: Value,
//     trace: &ConfigTrace,
//     max_depth: usize,
// ) -> PackerResult<Vec<(String, Preset)>> {
//     let mut output = Vec::new();
//     pack_presets_internal("", value, trace, 0, max_depth, &mut output).await?;
//     Ok(output)
// }
//
// #[async_recursion(auto)]
// async fn pack_presets_internal(
//     preset_name: &str,
//     value: Value,
//     trace: &ConfigTrace,
//     depth: usize,
//     max_depth: usize,
//     output: &mut Vec<(String, Preset)>,
// ) -> PackerResult<()> {
//     if depth > max_depth {
//         return Err(PackerError::MaxPresetNamespaceDepthExceeded(max_depth));
//     }
//
//     let obj = value.try_into_object().map_err(|_| {
//         if preset_name.is_empty() {
//             PackerError::InvalidConfigProperty(trace.clone(), prop::PRESETS.to_string())
//         } else {
//             PackerError::InvalidPreset(trace.clone(), preset_name.to_string())
//         }
//     })?;
//
//     for (key, value) in obj.into_iter() {
//         yield_budget(256).await;
//         if let Some(namespace) = key.strip_prefix('_') {
//             // sub namespace
//             let full_key = if preset_name.is_empty() {
//                 format!("_{namespace}")
//             } else {
//                 format!("{preset_name}::{namespace}")
//             };
//             pack_presets_internal(&full_key, value, trace, depth + 1, max_depth, output).await?;
//         } else {
//             // preset
//             let full_key = if preset_name.is_empty() {
//                 format!("_{key}")
//             } else {
//                 format!("{preset_name}::{key}")
//             };
//             let preset = Preset::compile(value)
//                 .ok_or_else(|| PackerError::InvalidPreset(trace.clone(), full_key.clone()))?;
//             output.push((full_key, preset));
//         }
//     }
//
//     Ok(())
// }
