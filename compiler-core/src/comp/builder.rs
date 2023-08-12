use std::collections::HashMap;

use celerctypes::{GameCoord, MapCoordMap, RouteMetadata};

use crate::lang::Preset;

use super::Compiler;

#[derive(Default, Debug, Clone)]
pub struct CompilerBuilder {
    project: RouteMetadata,
    presets: HashMap<String, Preset>,
    color: String,
    coord: GameCoord,
}

impl CompilerBuilder {
    pub fn new(project: RouteMetadata, color: String, coord: GameCoord) -> Self {
        CompilerBuilder {
            color,
            coord,
            project,
            presets: HashMap::new(),
        }
    }
    pub fn add_preset(&mut self, name: &str, preset: Preset) -> &mut Self {
        self.presets.insert(name.to_string(), preset);
        self
    }

    pub fn build(self) -> Compiler {
        Compiler {
            project: self.project,
            presets: self.presets,
            color: self.color,
            coord: self.coord,
            ..Default::default()
        }
    }
}
