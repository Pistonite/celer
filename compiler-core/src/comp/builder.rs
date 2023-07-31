use std::collections::HashMap;

use celerctypes::{GameCoord, MapCoordMap};

use crate::lang::Preset;

use super::Compiler;

#[derive(Default, Debug, Clone)]
pub struct CompilerBuilder {
    presets: HashMap<String, Preset>,
    color: String,
    coord: GameCoord,
    coord_transform: Option<MapCoordMap>,
}

impl CompilerBuilder {
    pub fn new(color: String, coord: GameCoord) -> Self {
        CompilerBuilder {
            color,
            coord,
            ..Default::default()
        }
    }
    pub fn add_preset(&mut self, name: &str, preset: Preset) -> &mut Self {
        self.presets.insert(name.to_string(), preset);
        self
    }

    pub fn build(self) -> Compiler {
        Compiler {
            presets: self.presets,
            color: self.color,
            coord: self.coord,
            ..Default::default() //TODO do the thing
        }
    }
}
