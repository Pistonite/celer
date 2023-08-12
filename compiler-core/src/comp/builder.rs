use std::collections::HashMap;

use celerctypes::{GameCoord, MapCoordMap, RouteMetadata};
use derivative::Derivative;

use crate::lang::Preset;

use super::Compiler;

#[derive(Default, Debug, Clone)]
pub struct CompilerBuilder {
    project: RouteMetadata,
    presets: HashMap<String, Preset>,
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
            presets: HashMap::new(),
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

    pub fn build(self) -> Compiler {
        Compiler {
            project: self.project,
            presets: self.presets,
            color: self.color,
            coord: self.coord,
            default_icon_priority: self.default_icon_priority,
            ..Default::default()
        }
    }
}
