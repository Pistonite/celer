//! Test utilities
use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::api::CompilerMetadata;
use crate::lang::Preset;
use crate::types::{Axis, MapCoordMap, MapMetadata, RouteMetadata, GameCoord, RouteMetadata};

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

pub fn create_test_compiler_with_coord_transform() -> Compiler<'static> {
    let project = RouteMetadata {
        map: MapMetadata {
            coord_map: MapCoordMap {
                mapping_3d: (Axis::X, Axis::Y, Axis::Z),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };
    let builder = super::CompilerBuilder::new(project, Default::default(), Default::default());
    builder.build()
}
