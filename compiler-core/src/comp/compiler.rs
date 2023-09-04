use std::collections::HashMap;

use celerctypes::{GameCoord, MapCoordMap, RouteMetadata};
use derivative::Derivative;

use crate::api::CompilerMetadata;
use crate::lang::Preset;

#[derive(Derivative, Debug, Clone)]
#[derivative(Default)]
pub struct Compiler {
    pub project: RouteMetadata,
    pub meta: CompilerMetadata,
    /// Current color of the map line
    pub color: String,
    /// Current position on the map
    pub coord: GameCoord,
    #[derivative(Default(value = "8"))]
    pub max_preset_depth: usize,
}

#[cfg(test)]
#[derive(Default, Debug, Clone)]
pub struct CompilerBuilder {
    project: RouteMetadata,
    presets: HashMap<String, Preset>,
    color: String,
    coord: GameCoord,
    default_icon_priority: i64,
}

#[cfg(test)]
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
            meta: CompilerMetadata {
                presets: self.presets,
                default_icon_priority: self.default_icon_priority,
            },
            color: self.color,
            coord: self.coord,
            ..Default::default()
        }
    }
}
