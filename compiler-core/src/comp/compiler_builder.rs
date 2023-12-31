//! Builder for making a compiler in tests
use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::api::CompilerMetadata;
use crate::lang::Preset;
use crate::types::{GameCoord, RouteMetadata};

use super::Compiler;

#[derive(Default, Debug, Clone)]
pub struct CompilerBuilder {
    project: RouteMetadata,
    presets: BTreeMap<String, Preset>,
    color: String,
    coord: GameCoord,
    default_icon_priority: i64,
}

impl CompilerBuilder {
    pub fn new(project: RouteMetadata, color: String, coord: GameCoord) -> Self {
        CompilerBuilder {
            color,
            coord,
            project,
            ..Default::default()
        }
    }
    pub fn add_preset(&mut self, name: &str, preset: Preset) -> &mut Self {
        self.presets.insert(name.to_string(), preset);
        self
    }

    pub fn set_default_icon_priority(&mut self, priority: i64) -> &mut Self {
        self.default_icon_priority = priority;
        self
    }

    pub fn build(self) -> Compiler<'static> {
        Compiler {
            project: Cow::Owned(self.project),
            meta: Cow::Owned(CompilerMetadata {
                presets: self.presets,
                default_icon_priority: self.default_icon_priority,
                ..Default::default()
            }),
            color: self.color,
            coord: self.coord,
            ..Default::default()
        }
    }
}
