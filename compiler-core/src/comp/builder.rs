use std::collections::HashMap;

use crate::lang::Preset;

use super::Compiler;

#[derive(Default, Debug, Clone)]
pub struct CompilerBuilder {
    presets: HashMap<String, Preset>,
}

impl CompilerBuilder {
    pub fn add_preset(&mut self, name: &str, preset: Preset) -> &mut Self {
        self.presets.insert(name.to_string(), preset);
        self
    }

    pub fn build(self) -> Compiler {
        Compiler {
            presets: self.presets,
            ..Default::default() //TODO do the thing
        }
    }
}
